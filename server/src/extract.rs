use crate::openai::embeddings::Embeddings;
use crate::util::base_url;
use crate::{
	entities::{prelude::*, *},
	openai::assistant::{Assistant, AssistantType},
};
use anyhow::anyhow;
use anyhow::Result;

use chrono::FixedOffset;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use html2md::parse_html;
use sea_orm::entity::prelude::*;
use sea_orm::*;
use similar::{ChangeTag, TextDiff};
use std::sync::Arc;
use std::time::Duration;

use url::Url;

const MESSAGE_MAX_CHARS: usize = 32000;
const MAX_EXTRACT_CHARS: usize = 1000000;

pub struct PostingsExtractorHandler {
	extractors: Vec<PostingsExtractor>,
}

impl PostingsExtractorHandler {
	pub fn new() -> Self {
		PostingsExtractorHandler { extractors: vec![] }
	}

	pub async fn refresh(&mut self, db: &DatabaseConnection) -> Result<Vec<posting::Model>> {
		self.fetch(db).await
	}

	async fn fetch(&mut self, db: &DatabaseConnection) -> Result<Vec<posting::Model>> {
		let sources = Source::find().filter(source::Column::Deleted.eq(false)).all(db).await?;
		let settings = Settings::find().one(db).await?.expect("No settings stored");
		let filters = Filter::find().all(db).await?;

		let tasks: Vec<_> = sources
			.into_iter()
			.map(|source: source::Model| {
				let settings = settings.clone();
				let filters = filters.clone();

				tokio::spawn(async move {
					let opt = LaunchOptionsBuilder::default().headless(true).idle_browser_timeout(Duration::from_millis(60_000)).build().unwrap();
					let browser = Browser::new(opt).unwrap();

					let mut extractor = PostingsExtractor::new(
						source.url.clone(),
						source.id,
						settings,
						source.selector.clone(),
						source.pagination.clone(),
						filters,
						source.content.clone(),
						browser,
					);

					let _ = extractor.extract().await;
					extractor
				})
			})
			.collect();

		let res = futures::future::join_all(tasks).await;

		self.extractors = res
			.into_iter()
			.flat_map(|e| match e {
				Ok(r) => Some(r),
				_ => None,
			})
			.collect();

		let postings: Vec<posting::Model> = self.extractors.iter().map(|e| e.extracted_postings.clone()).flatten().flatten().collect();

		Ok(postings)
	}

	pub async fn save(&self, db: &DatabaseConnection) -> Result<()> {
		let settings = Settings::find().one(db).await?.expect("No settings stored");
		let liked_postings: Vec<Vec<f32>> = Embedding::find()
			.select_only()
			.column(embedding::Column::Vector)
			.filter(posting::Column::IsMatch.eq(true))
			.join(JoinType::InnerJoin, embedding::Relation::Posting.def())
			.order_by_desc(posting::Column::CreatedAt)
			.limit(30)
			.into_tuple()
			.all(db)
			.await?;
		let disliked_postings: Vec<Vec<f32>> = Embedding::find()
			.select_only()
			.column(embedding::Column::Vector)
			.filter(posting::Column::IsMatch.eq(false))
			.join(JoinType::InnerJoin, embedding::Relation::Posting.def())
			.order_by_desc(posting::Column::CreatedAt)
			.limit(30)
			.into_tuple()
			.all(db)
			.await?;

		let embedding = Embeddings::new(&settings.api_key.unwrap());

		for extractor in &self.extractors {
			for posting in extractor.extracted_postings.clone().unwrap_or(vec![]) {
				let content = posting.content.clone();
				let mut active_posting: posting::ActiveModel = posting.into();
				active_posting.id = NotSet;
				active_posting.created_at = Set(Some(chrono::offset::Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())));
				active_posting.seen = Set(Some(false));
				active_posting.source_id = Set(Some(extractor.source_id));
				let embedding_vector = embedding.create(&content.unwrap_or("".to_string())).await?;
				let like_similarity = embedding.get_similarity(&embedding_vector, &liked_postings);
				let dislike_similarity = embedding.get_similarity(&embedding_vector, &disliked_postings);
				active_posting.match_similarity = Set(Some(like_similarity - dislike_similarity));

				let inserted_posting = active_posting.insert(db).await?;

				let active_embedding = embedding::ActiveModel {
					id: NotSet,
					posting_id: Set(Some(inserted_posting.id)),
					vector: Set(Some(embedding_vector)),
				};

				active_embedding.insert(db).await?;
			}
		}

		for extractor in &self.extractors {
			let _ = Source::update_many()
				.col_expr(source::Column::Content, Expr::value(extractor.parsed_content.to_string().clone()))
				.col_expr(source::Column::Unreachable, Expr::value(extractor.unreachable.clone()))
				.filter(source::Column::Id.eq(extractor.source_id))
				.exec(db)
				.await;
		}

		Ok(())
	}
}

#[derive(Clone, Default, Debug)]
struct ParsedSource {
	parsed_pages: Vec<ParsedPage>,
}

impl ParsedSource {
	pub fn to_string(&self) -> String {
		let contents: Vec<String> = self.parsed_pages.iter().map(|p| p.content.to_string()).collect();
		contents.join("\n")
	}

	pub fn get_page_for_index(&self, index: Option<usize>) -> Option<&ParsedPage> {
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

#[derive(Clone, Default, Debug)]
struct ParsedPage {
	content: String,
	url: String,
}

#[derive(Clone)]
pub struct PostingsExtractor {
	url: String,
	source_id: i32,
	selector: Option<String>,
	pagination: Option<String>,
	filters: Vec<filter::Model>,
	settings: settings::Model,
	browser: Browser,

	parsed_content: ParsedSource,
	cached_content: Option<String>,
	extracted_postings: Option<Vec<posting::Model>>,
	unreachable: bool,
}

impl PostingsExtractor {
	pub fn new(
		url: String,
		source_id: i32,
		settings: settings::Model,
		selector: Option<String>,
		pagination: Option<String>,
		filters: Vec<filter::Model>,
		cached_content: Option<String>,
		browser: Browser,
	) -> Self {
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

	pub async fn extract(&mut self) -> Result<Vec<posting::Model>> {
		self.parse_source_content().await?;
		let content_diff = self.new_source_content();

		if !content_diff.parsed_pages.is_empty() {
			let postings = self.extract_postings(&content_diff).await?;
			self.extracted_postings = Some(postings.clone());
			return Ok(postings);
		}

		return Ok(vec![]);
	}

	async fn parse_source_content(&mut self) -> Result<()> {
		let tab = self.browser.new_tab()?;

		match tab.navigate_to(&self.url) {
			Err(_) => {
				self.unreachable = true;
				return Err(anyhow!("Source is unreachable."));
			}
			_ => {}
		}

		tab.wait_until_navigated()?;

		let head = tab.wait_for_element("head")?.get_content()?;
		if head == "<head><meta name=\"color-scheme\" content=\"light dark\"></head>" {
			// raw JSON content, return as is
			let parsed_page = ParsedPage {
				content: tab.wait_for_element("body")?.get_inner_text()?,
				url: tab.get_url(),
			};
			self.parsed_content = ParsedSource { parsed_pages: vec![parsed_page] };
		} else {
			let parsed_pages = self.parse_source_pages(tab, &ParsedPage::default())?;
			self.parsed_content = ParsedSource { parsed_pages };
		}

		return Ok(());
	}

	fn parse_source_pages(&self, tab: Arc<Tab>, prev_content: &ParsedPage) -> Result<Vec<ParsedPage>> {
		std::thread::sleep(std::time::Duration::from_secs(5)); // todo: needed?

		let selector = match &self.selector {
			Some(s) => s,
			None => "body",
		};

		match tab.wait_for_element(selector) {
			Ok(el) => {
				let content = el.get_inner_text()?;
				let parsed_page = ParsedPage {
					content: content.clone(),
					url: tab.get_url(),
				};

				if &content == &prev_content.content {
					// current page has already been parsed; stop recursion
					return Ok(vec![]);
				}

				if let Some(pagination_selector) = &self.pagination {
					if let Ok(pagination_element) = tab.wait_for_element(pagination_selector) {
						match pagination_element.tag_name.as_str() {
							"A" => {
								// pagination element is a link

								if let Some(base_url) = base_url(Url::parse(&tab.get_url())?) {
									let a_attributes = pagination_element.attributes.unwrap_or(vec![]);
									let next_page_url_index = a_attributes.clone().into_iter().position(|r| r == "href".to_string());
									if let Some(next_page_url_index) = next_page_url_index {
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
								let pagination_click = pagination_element.click();
								if pagination_click.is_ok() {
									std::thread::sleep(std::time::Duration::from_secs(15));
									tab.wait_until_navigated()?;
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
			Err(_) => Ok(vec![]), // todo: return some error
		}
	}

	fn new_source_content(&self) -> ParsedSource {
		let cached_content = match &self.cached_content {
			Some(c) => c,
			None => "",
		};

		let parsed = self.parsed_content.to_string();
		let content_diff = TextDiff::from_lines(cached_content, &parsed);
		let mut new_content: ParsedSource = ParsedSource::default();

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

		return new_content.limit_content(MAX_EXTRACT_CHARS);
	}

	async fn extract_postings(&mut self, content: &ParsedSource) -> Result<Vec<posting::Model>> {
		let mut postings: Vec<posting::Model> = vec![];

		for page in &content.parsed_pages {
			let mut content_chunks = self.chunk_message(&page.content);
			let chatgpt_result = self.chatgpt_extract_postings(&mut content_chunks).await?;

			for response in chatgpt_result {
				let parsed_response: Vec<posting::Model> = serde_json::from_str(&response)?;

				for mut posting in parsed_response {
					self.add_posting_details(&mut posting, &page)?;
					postings.push(posting);
				}
			}
		}

		Ok(postings)
	}

	fn add_posting_details(&self, posting: &mut posting::Model, page: &ParsedPage) -> Result<()> {
		let tab = self.browser.new_tab()?;
		tab.navigate_to(&page.url)?;
		tab.wait_until_navigated()?;

		let selector = match &self.selector {
			Some(s) => s,
			None => "body",
		};

		let title = &posting.title;

		match tab.wait_for_element(selector) {
			Ok(el) => match el.find_elements_by_xpath(&format!("//*[contains(text(), '{title}')]")) {
				Ok(elements_with_text) => {
					let tab_url = &page.url;
					for el in elements_with_text {
						if &tab.get_url() != tab_url {
							tab.navigate_to(&tab_url)?;
							tab.wait_until_navigated()?;
						}

						if el.click().is_ok() {
							std::thread::sleep(std::time::Duration::from_secs(15));
							tab.wait_until_navigated()?;
							let new_url = &tab.get_url();

							if new_url != tab_url {
								if let Ok(page_element) = tab.wait_for_element("body") {
									posting.url = Some(new_url.to_string());
									let content = page_element.get_content()?;
									let markdown_content = parse_html(&content);
									posting.content = Some(markdown_content);
								}
							}
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

	fn chunk_message(&self, message: &String) -> Vec<String> {
		message
			.chars()
			.collect::<Vec<char>>()
			.chunks(MESSAGE_MAX_CHARS)
			.map(|c| c.iter().collect::<String>())
			.collect::<Vec<String>>()
	}

	async fn chatgpt_extract_postings(&mut self, message_parts: &mut Vec<String>) -> Result<Vec<String>> {
		let mut assistant = Assistant::new(
			&self.settings.api_key.clone().unwrap_or("".to_string()),
			&self.settings.model.clone().unwrap_or("".to_string()),
			AssistantType::JobsFeed,
		)
		.await?;

		let criteria = self
			.filters
			.iter()
			.fold("".to_string(), |cur: String, next: &filter::Model| cur + &format!("{}: {}", next.name, next.value));

		let last_message = format!(
			"Criteria: {criteria} Provide a single response. \
            Response format: [{{\"title\": \"\", \"description\": \"\"}}]. \
            Description should not contain location. Description should be up to 200 characters. Only return complete and valid JSON."
		);
		message_parts.push(last_message);

		let response = assistant.run(message_parts).await?;

		Ok(response)
	}
}
