/// Source parsing and posting extraction.
use crate::openai::embeddings::Embeddings;
use crate::util::base_url;
use crate::{
	entities::{prelude::*, *},
	openai::assistant::{Assistant, AssistantType},
};
use anyhow::Result;

use chrono;
use chrono::FixedOffset;
use chrono::Utc;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use html2md::parse_html;
use sea_orm::entity::prelude::*;
use sea_orm::*;
use similar::{ChangeTag, TextDiff};
use std::sync::Arc;
use std::time::Duration;

use std::cmp::min;
use url::Url;

/// maximum number of characters per message sent to OpenAI API
const MESSAGE_MAX_CHARS: usize = 32000;

/// maximum number of characters that should be extracted from a source page
const MAX_EXTRACT_CHARS: usize = 10000000;

/// maximum number of characters to create embedding vector from
const EMBEDDING_MAX_CHARS: usize = 8000;

/// Represents a source that is being processed.
#[derive(Clone, Default, Debug)]
struct ParsedSource {
	/// Set of pages that were parsed.
	/// Multiple pages will be parsed if source is paginated.
	parsed_pages: Vec<ParsedPage>,
}

impl ParsedSource {
	/// Return a string representation of the parsed page.
	pub fn to_string(&self) -> String {
		let contents: Vec<String> = self.parsed_pages.iter().map(|p| p.content.to_string()).collect();
		contents.join("\n")
	}

	/// Return the parsed page based on the character index offset.
	pub fn get_page_for_index(&self, index: Option<usize>) -> Option<&ParsedPage> {
		// iterate through parsed pages until we get to the index offset based on the text content
		if let Some(index) = index {
			let mut i = 0;
			for page in &self.parsed_pages {
				i += page.content.len();

				if index <= i {
					return Some(page);
				}
			}
		}

		return None;
	}

	/// Add a parsed page.
	pub fn add_content(&mut self, content: &String, url: &String) {
		for page in &mut self.parsed_pages {
			if &page.url == url {
				page.content.push_str(&format!("\n {content}"));
				return;
			}
		}

		self.parsed_pages.push(ParsedPage {
			content: content.clone(),
			url: url.clone(),
		})
	}

	/// Remove all parsed pages based on whether their content is after the `max_char` limit.
	/// The limit is based on the text content length per parsed page.
	pub fn limit_content(&self, max_chars: usize) -> Self {
		let mut content_len = 0;
		let mut parsed_source = ParsedSource { parsed_pages: vec![] };

		for parsed_page in self.parsed_pages.iter() {
			if content_len + parsed_page.content.len() <= max_chars {
				parsed_source.parsed_pages.push(parsed_page.clone());
				content_len += parsed_page.content.len();
			}
		}

		parsed_source
	}
}

/// Represents a single parsed page.
#[derive(Clone, Default, Debug)]
struct ParsedPage {
	/// Textual page content
	content: String,

	/// Page URL
	url: String,
}

/// Handler for extracting postings from a specific source.
#[derive(Clone)]
pub struct PostingsExtractor {
	/// Source URL
	url: String,

	/// Source ID
	source_id: i32,

	/// CSS selector to use to fetch job postings from specific page element
	selector: Option<String>,

	/// CSS selector pointing to pagination page element
	pagination: Option<String>,

	/// Configured filters that should be used to determine relevant job postings
	filters: Vec<filter::Model>,

	/// App settings
	settings: settings::Model,

	/// Headless browser handler
	browser: Browser,

	/// Processed page content
	parsed_content: ParsedSource,

	/// Previously cached content for source
	cached_content: Option<String>,

	/// Job postings that were extracted from source
	extracted_postings: Option<Vec<posting::Model>>,

	/// Whether the source URL could be opened
	unreachable: bool,
}

impl PostingsExtractor {
	/// Create and return a new posting handler instance.
	pub fn new(url: String, source_id: i32, settings: settings::Model, selector: Option<String>, pagination: Option<String>, filters: Vec<filter::Model>, cached_content: Option<String>) -> Self {
		// open a headles browser instance
		let opt = LaunchOptionsBuilder::default().headless(true).idle_browser_timeout(Duration::from_millis(120_000)).build().unwrap();
		let browser = Browser::new(opt).unwrap();

		PostingsExtractor {
			url,
			source_id,
			selector,
			pagination,
			cached_content,
			filters,
			settings,
			parsed_content: ParsedSource::default(),
			extracted_postings: None,
			browser,
			unreachable: false,
		}
	}

	/// Start extracting job postings from the source.
	pub async fn extract(&mut self, db: &DatabaseConnection) -> Result<Vec<posting::Model>> {
		// parse the source content
		self.parse_source_content().await?;
		// use the previously cached content to determine content that has been added since last extraction
		let content_diff = self.new_source_content();

		eprintln!("{:?}", content_diff);

		if !content_diff.parsed_pages.is_empty() {
			// extract job postings from the new page content
			let postings = self.extract_postings(&content_diff, db).await?;
			self.extracted_postings = Some(postings.clone());
			return Ok(postings);
		}

		// close the headless browser
		self.close_tabs()?;

		return Ok(vec![]);
	}

	/// Parse the text content of the source and store.
	async fn parse_source_content(&mut self) -> Result<()> {
		let tab = self.browser.new_tab()?;

		// open the source URL
		match tab.navigate_to(&self.url) {
			Err(_) => {
				self.unreachable = true;
				return Ok(());
			}
			_ => {}
		}
		match tab.wait_until_navigated() {
			Err(_) => {
				self.unreachable = true;
				return Ok(());
			}
			_ => {}
		}

		let head = tab.wait_for_element("head")?.get_content()?;

		if head == "<head><meta name=\"color-scheme\" content=\"light dark\"></head>" {
			// raw JSON content, return as is
			let parsed_page = ParsedPage {
				content: tab.wait_for_element("body")?.get_inner_text()?,
				url: tab.get_url(),
			};
			self.parsed_content = ParsedSource { parsed_pages: vec![parsed_page] };
		} else {
			// HTML content, parse all relevant pages
			let parsed_pages = self.parse_source_pages(tab, &ParsedPage::default())?;
			self.parsed_content = ParsedSource { parsed_pages };
		}

		return Ok(());
	}

	/// Close all browser tabs.
	fn close_tabs(&self) -> Result<()> {
		self.browser.get_tabs().lock().unwrap().iter().for_each(|t| {
			let _ = t.close(true);
		});
		Ok(())
	}

	/// Parses and returns the pages related to the source.
	fn parse_source_pages(&self, tab: Arc<Tab>, prev_content: &ParsedPage) -> Result<Vec<ParsedPage>> {
		// select relevant part of the page to get postings from
		let selector = match &self.selector {
			Some(s) => {
				if s.trim() != "" {
					s
				} else {
					"body"
				}
			}
			None => "body",
		};

		match tab.wait_for_element(selector) {
			Ok(el) => {
				// get text representation of page content
				let content = el.get_inner_text()?;
				let parsed_page = ParsedPage {
					content: content.clone(),
					url: tab.get_url(),
				};

				if &content == &prev_content.content {
					// current page has already been parsed; stop recursion
					return Ok(vec![]);
				}

				// open next page if pagination exists
				if let Some(pagination_selector) = &self.pagination {
					if let Ok(pagination_element) = tab.wait_for_element(pagination_selector) {
						match pagination_element.tag_name.as_str() {
							"A" => {
								// pagination element is a link
								if let Some(base_url) = base_url(Url::parse(&tab.get_url())?) {
									let a_attributes = pagination_element.attributes.unwrap_or(vec![]);
									// get the link URL
									let next_page_url_index = a_attributes.clone().into_iter().position(|r| r == "href".to_string());
									if let Some(next_page_url_index) = next_page_url_index {
										// open the next page and parse
										let paginated_url = base_url.join(&a_attributes[next_page_url_index + 1])?;
										tab.navigate_to(paginated_url.as_str())?;
										tab.wait_until_navigated()?;
										let mut parsed_pages = self.parse_source_pages(tab, &parsed_page)?;
										parsed_pages.insert(0, parsed_page);
										return Ok(parsed_pages);
									}
								} else {
									return Ok(vec![parsed_page]);
								}
							}
							_ => {
								// pagination element is a button or some other element
								// click okn the element
								let pagination_click = pagination_element.click();
								if pagination_click.is_ok() {
									let tab_url = &tab.get_url();
									let start_time = Utc::now().time();
									let site_content = tab.wait_for_element("body").unwrap().get_inner_text()?;
									// wait for up to 10 seconds to see if page content has changed
									while &tab.get_url() == tab_url && (Utc::now().time() - start_time).num_seconds() < 10 && site_content == tab.wait_for_element("body").unwrap().get_inner_text()? {
										tab.wait_until_navigated()?;
									}

									// parse newly changed page
									let mut parsed_pages = self.parse_source_pages(tab, &parsed_page)?;
									parsed_pages.insert(0, parsed_page);
									return Ok(parsed_pages);
								}
							}
						}
					}
				}

				return Ok(vec![parsed_page]);
			}
			Err(err) => Err(err),
		}
	}

	/// Determine which content has been newly added compared to the cached source content.
	fn new_source_content(&self) -> ParsedSource {
		let cached_content = match &self.cached_content {
			Some(c) => c,
			None => "",
		};

		let parsed = self.parsed_content.to_string();
		let content_diff = TextDiff::from_lines(cached_content, &parsed);
		let mut new_content: ParsedSource = ParsedSource::default();

		// compare cache with source content and keep added changes
		for change in content_diff.iter_all_changes() {
			match change.tag() {
				ChangeTag::Insert => {
					let c = change.as_str().unwrap().to_string();
					if let Some(page) = self.parsed_content.get_page_for_index(change.new_index()) {
						new_content.add_content(&c, &page.url)
					} else {
						new_content.add_content(&c, &self.url)
					}
				}
				_ => {}
			};
		}

		// limit the source content length to save cost and performance
		return new_content.limit_content(MAX_EXTRACT_CHARS);
	}

	/// Extracts and returns job postings fetched from the source.
	async fn extract_postings(&mut self, content: &ParsedSource, db: &DatabaseConnection) -> Result<Vec<posting::Model>> {
		let mut postings: Vec<posting::Model> = vec![];

		for page in &content.parsed_pages {
			// limit the size of the page content for every message sent to the OpenAI assistant
			let mut content_chunks = self.chunk_message(&page.content);

			// use OpenAI assistant to extract job postings
			let chatgpt_result = self.chatgpt_extract_postings(&mut content_chunks).await?;

			for response in chatgpt_result {
				let parsed_response: Vec<posting::Model> = serde_json::from_str(&response)?;
				let posting_titles: Vec<&String> = parsed_response.iter().map(|p| &p.title).collect();

				// filter postings that were seen recently
				let existing_postings = Posting::find()
					.filter(posting::Column::SourceId.eq(self.source_id))
					.filter(posting::Column::Title.is_in(posting_titles))
					.filter(posting::Column::CreatedAt.gte(chrono::offset::Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()) - chrono::Duration::days(30)))
					.all(db)
					.await
					.expect("Could not get existing postings")
					.into_iter()
					.collect::<Vec<_>>();

				for mut posting in parsed_response {
					if existing_postings.iter().find(|ep| ep.title == posting.title).is_none() {
						if page.content.contains(&posting.title) {
							// add additional posting information
							self.add_posting_details(&mut posting, &page)?;
							postings.push(posting);
						}
					}
				}
			}
		}

		Ok(postings)
	}

	/// Extract information from a page related to a specific job posting.
	fn add_posting_details(&self, posting: &mut posting::Model, page: &ParsedPage) -> Result<()> {
		// open the URL of the page the job posting was found on
		let tab = self.browser.new_tab()?;
		tab.navigate_to(&page.url)?;
		tab.wait_until_navigated()?;

		let selector = match &self.selector {
			Some(s) => s,
			None => "body",
		};

		let title = &posting.title;

		// wait until page is loaded
		match tab.wait_for_element(selector) {
			// based on the job posting title, find a page element that contains the title
			Ok(el) => match el.find_elements_by_xpath(&format!("//*[contains(text(), '{title}')]")) {
				Ok(elements_with_text) => {
					let tab_url = &page.url;

					for el in elements_with_text {
						if &tab.get_url() != tab_url {
							// go back to the page job posting was extracted from
							tab.navigate_to(&tab_url)?;
							tab.wait_until_navigated()?;
						}

						match el.tag_name.as_str() {
							"A" => {
								// element is a link
								if let Some(base_url) = base_url(Url::parse(&tab_url)?) {
									// get the job posting specific URL
									let a_attributes = el.attributes.unwrap_or(vec![]);
									let next_page_url_index = a_attributes.clone().into_iter().position(|r| r == "href".to_string());
									if let Some(next_page_url_index) = next_page_url_index {
										// open the posting URL
										let paginated_url = base_url.join(&a_attributes[next_page_url_index + 1])?;
										tab.navigate_to(paginated_url.as_str())?;
										tab.wait_until_navigated()?;
									}
								}
							}
							_ => {
								// element is a button or some other type of HTML element
								if el.click().is_ok() {
									// click on the element
									let tab_url = &tab.get_url();
									let start_time = Utc::now().time();
									let site_content = tab.wait_for_element("body").unwrap().get_inner_text()?;
									// wait up to 10 seconds until a new page has been opened or the page content has changed
									while &tab.get_url() == tab_url && (Utc::now().time() - start_time).num_seconds() < 10 && site_content == tab.wait_for_element("body").unwrap().get_inner_text()? {
										tab.wait_until_navigated()?;
									}
								}
							}
						};

						// get the new URL, unless it's identical to the source URL
						let new_url = &tab.get_url();
						if new_url != tab_url {
							if let Ok(page_element) = tab.wait_for_element("body") {
								// parse the posting content
								posting.url = Some(new_url.to_string());
								let content = page_element.get_content()?;
								let markdown_content = parse_html(&content);
								posting.content = Some(markdown_content);
							}
						} else {
							// pages are not changing on click; skip further elements
							return Ok(());
						}
					}
					return Ok(());
				}
				_ => {
					return Ok(());
				}
			},
			_ => return Ok(()),
		}
	}

	/// Split the input message into smaller messages with a maximum size.
	fn chunk_message(&self, message: &String) -> Vec<String> {
		message
			.chars()
			.collect::<Vec<char>>()
			.chunks(MESSAGE_MAX_CHARS)
			.map(|c| c.iter().collect::<String>())
			.collect::<Vec<String>>()
	}

	/// Use OpenAI assistant to extract job postings from the source content.
	async fn chatgpt_extract_postings(&mut self, message_parts: &mut Vec<String>) -> Result<Vec<String>> {
		// create a new assistant
		let mut assistant = Assistant::new(
			&self.settings.api_key.clone().unwrap_or("".to_string()),
			&self.settings.model.clone().unwrap_or("".to_string()),
			AssistantType::JobsFeed,
		)
		.await?;

		// get filters
		let criteria = self
			.filters
			.iter()
			.fold("".to_string(), |cur: String, next: &filter::Model| cur + &format!("{}: {}", next.name, next.value));

		// create the prompt
		let last_message = format!(
			"Criteria: {criteria} Provide a single response. \
            Response format: [{{\"title\": \"\"}}]. \
			Extract a complete list of job posting titles from the provided inputs that are related to the provided criteria. \
            Only return complete and valid JSON."
		);
		message_parts.push(last_message);

		let response = assistant.run(message_parts).await?;

		Ok(response)
	}

	/// Saves extracted job postings to the database.
	pub async fn save(&self, db: &DatabaseConnection) -> Result<()> {
		let settings = Settings::find().one(db).await?.expect("No settings stored");

		// get a set of postings that were previously "liked"
		let liked_postings: Vec<Vec<f32>> = Embedding::find()
			.select_only()
			.column(embedding::Column::Vector)
			.filter(posting::Column::IsMatch.eq(true))
			.join(JoinType::InnerJoin, embedding::Relation::Posting.def())
			.order_by_desc(posting::Column::CreatedAt)
			.limit(50)
			.into_tuple()
			.all(db)
			.await?;

		// get a set of postings that were previously "disliked"
		let disliked_postings: Vec<Vec<f32>> = Embedding::find()
			.select_only()
			.column(embedding::Column::Vector)
			.filter(posting::Column::IsMatch.eq(false))
			.join(JoinType::InnerJoin, embedding::Relation::Posting.def())
			.order_by_desc(posting::Column::CreatedAt)
			.limit(50)
			.into_tuple()
			.all(db)
			.await?;

		// create new embeddings handler
		let embedding = Embeddings::new(&settings.api_key.unwrap());

		// for each newly extracted posting compute the similarity scores to determine if they would be a good match
		for posting in self.extracted_postings.clone().unwrap_or(vec![]) {
			// use the posting content and title as input for getting the embedding vector
			let content = posting.content.clone();
			let title = posting.title.clone();
			let mut active_posting: posting::ActiveModel = posting.into();
			active_posting.id = NotSet;
			active_posting.created_at = Set(Some(chrono::offset::Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())));
			active_posting.seen = Set(Some(false));
			active_posting.source_id = Set(Some(self.source_id));
			let embedding_content = content.unwrap_or(title.to_string());

			// limit the content that is used to create the embedding
			let end_index = embedding_content.char_indices().map(|(i, _)| i).nth(min(EMBEDDING_MAX_CHARS, embedding_content.len() - 1)).unwrap_or(0);
			let embedding_vector = embedding.create(&&embedding_content[..end_index].to_string()).await?;

			// compute similarity to "like"d and "dislike"d postings to compute a similarity score
			let like_similarity = embedding.get_similarity(&embedding_vector, &liked_postings);
			let dislike_similarity = embedding.get_similarity(&embedding_vector, &disliked_postings);
			active_posting.match_similarity = Set(Some(like_similarity - dislike_similarity));

			// store posting
			let inserted_posting = active_posting.insert(db).await?;

			// store embedding
			let active_embedding = embedding::ActiveModel {
				id: NotSet,
				posting_id: Set(Some(inserted_posting.id)),
				vector: Set(Some(embedding_vector)),
			};

			active_embedding.insert(db).await?;
		}

		// update the source
		let _ = Source::update_many()
			.col_expr(source::Column::Content, Expr::value(self.parsed_content.to_string().clone()))
			.col_expr(source::Column::Unreachable, Expr::value(self.unreachable.clone()))
			.filter(source::Column::Id.eq(self.source_id))
			.exec(db)
			.await;

		Ok(())
	}
}
