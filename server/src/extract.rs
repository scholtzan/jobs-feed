
use crate::entities::{prelude::*, *};
use headless_chrome::{Browser, Tab};
use similar::{ChangeTag, TextDiff};
use thiserror::Error;
use anyhow::Result;
use std::sync::Arc;
use crate::util::base_url;
use url::Url;
use serde::Deserialize;
use chatgpt::prelude::*;
use std::time::Duration;
use sea_orm::entity::prelude::*;
use sea_orm::*;
use chrono::{DateTime, Local, FixedOffset, Utc};

const MESSAGE_MAX_CHARS: usize = 7000;
const CONTEXT_MAX_CHARS: usize = 15000;
const CONTEXT_OVERLAP: usize = 200;
const MAX_REQUESTS_PER_SITE: usize = 3; // needed?

pub struct PostingsExtractorHandler {
    extractors: Vec<PostingsExtractor>
}

impl PostingsExtractorHandler {
    pub fn new() -> Self {
        PostingsExtractorHandler {
            extractors: vec![]
        }
    }

    pub async fn refresh(&mut self, db: &DatabaseConnection) -> Result<Vec<posting::Model>> {
        self.fetch(db).await
    }

    async fn fetch(&mut self, db: &DatabaseConnection) -> Result<Vec<posting::Model>> {
        let sources = Source::find().all(db).await?;
        let settings = Settings::find().one(db).await?.expect("No settings stored");
        let filters = Filter::find().all(db).await?;

        let tasks: Vec<_> = sources
        .into_iter()
        .map( |source: source::Model| {
            let settings = settings.clone();
            let filters = filters.clone();
            
            tokio::spawn(async move {
                let mut extractor = PostingsExtractor::new(
                    source.url.clone(), 
                    source.id,
                    settings,
                    source.selector.clone(), 
                    source.pagination.clone(),
                    filters,
                    source.content.clone(),
                );

                extractor.extract().await;
                extractor
            })
        })
        .collect();

        let res = futures::future::join_all(tasks).await;

        self.extractors = res.into_iter().flat_map(|e| {
            match e {
                Ok(r) => Some(r),
                _ => None
            }
        }).collect();

        let postings: Vec<posting::Model> = self.extractors.iter().map(|e| {
            e.extracted_postings.clone()
        }).flatten().flatten().collect();

        Ok(postings)
    }

    pub async fn save(&self, db: &DatabaseConnection) -> Result<()> {
        let postings: Vec<posting::ActiveModel> = self.extractors.iter().flat_map(|e| {
            let active_postings: Vec<posting::ActiveModel> = e.extracted_postings.clone().unwrap_or(vec![]).into_iter().map(|p| {
                let mut active_posting: posting::ActiveModel = p.into();
                active_posting.id = NotSet;
                active_posting.created_at = Set(Some(chrono::offset::Utc::now().with_timezone(&FixedOffset::east(0))));
                active_posting.seen = Set(false);
                active_posting.source_id = Set(Some(e.source_id));
                active_posting
            }).collect();

            active_postings
        }).collect();

        Posting::insert_many(postings).exec(db).await;

        for extractor in &self.extractors {
            Source::update_many()
            .col_expr(source::Column::Content, Expr::value(extractor.parsed_content.clone()))
            .filter(source::Column::Id.eq(extractor.source_id))
            .exec(db)
            .await;
        }

        Ok(())
    }
}



#[derive(Deserialize, Debug)]
struct ExtractResponse {
    postings: Vec<posting::Model>,
    last_posting: String
}


#[derive(Clone)]
pub struct PostingsExtractor {
    url: String,
    source_id: i32,
    selector: Option<String>,
    pagination: Option<String>,
    filters: Vec<filter::Model>,
    settings: settings::Model,

    parsed_content: Option<String>,
    cached_content: Option<String>,
    extracted_postings: Option<Vec<posting::Model>>
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
    ) -> Self {
        PostingsExtractor {
            url,
            source_id,
            selector,
            pagination,
            cached_content,
            filters,
            settings,
            parsed_content: None,
            extracted_postings: None
        }
    }

    pub async fn extract(&mut self) -> Result<Vec<posting::Model>> {
        self.parsed_content = Some(self.parse_source_content().await?);
        let content_diff = self.new_source_content();

        if let Some(content_diff) = content_diff {
            let postings = self.extract_postings(&content_diff).await?;
            self.extracted_postings = Some(postings.clone());
            return Ok(postings)
        }

        return Ok(vec![])
    }

    async fn parse_source_content(&self) -> Result<String> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to(&self.url)?;
        tab.wait_until_navigated()?;

        let head = tab.wait_for_element("head")?.get_content()?;
        if head == "<head><meta name=\"color-scheme\" content=\"light dark\"></head>" {
            // raw JSON content, return as is
            return tab.wait_for_element("body")?.get_inner_text();
        } else {
            let content = self.parse_source_pages(tab, &"".to_string())?;
            return Ok(content);
        }
    }

    fn parse_source_pages(&self, tab: Arc<Tab>, prev_content: &String) -> Result<String> {
        std::thread::sleep(std::time::Duration::from_secs(5)); // todo: needed?

        let selector = match &self.selector {
            Some(s) => s,
            None => "body"
        };

        match tab.wait_for_element(selector) {
            Ok(el) => {
                let content = el.get_inner_text()?;

                if &content == prev_content {
                    // current page has already been parsed; stop recursion
                    return Ok("".to_string())
                }

                if let Some(pagination_selector) = &self.pagination {
                    if let Ok(pagination_element) = tab.wait_for_element(pagination_selector) {
                        match pagination_element.tag_name.as_str() {
                            "A" => { // pagination element is a link

                                if let Some(base_url) = base_url(Url::parse(&tab.get_url())?) {
                                    let a_attributes = pagination_element.attributes.unwrap_or(vec![]);
                                    let next_page_url_index = a_attributes.clone().into_iter().position(|r| r == "href".to_string());
                                    if let Some(next_page_url_index) = next_page_url_index {
                                        let paginated_url = base_url.join(&a_attributes[next_page_url_index + 1])?;
                                        tab.navigate_to(paginated_url.as_str())?;
                                        tab.wait_until_navigated()?;
                                        let parsed_page = self.parse_source_pages(tab, &content)?;
                                        return Ok(content.clone() + "\n" + &parsed_page)
                                    }
                                } else {
                                    return Ok(content)
                                }
                            },
                            _ => {
                                let pagination_click = pagination_element.click();
                                // todo: not quite working
                                if pagination_click.is_ok() {       
                                    std::thread::sleep(std::time::Duration::from_secs(5));
                                    let parsed_page = self.parse_source_pages(tab, &content)?;
                                    return Ok(content.clone() + "\n" + &parsed_page)
                                }
                            }
                        }
                    }
                }

                return Ok(content)
            },
            Err(_) => Ok("".to_string())  // todo: return some error
        }
    }

    fn new_source_content(&self) -> Option<String> {
        if let Some(parsed_content) = &self.parsed_content {
            let cached_content = match &self.cached_content {
                Some(c) => c,
                None => ""
            };

            let content_diff = TextDiff::from_lines(cached_content, parsed_content);
            let mut new_content = "".to_string();

            for change in content_diff.iter_all_changes() {
                new_content += match change.tag() {
                    ChangeTag::Insert => change.as_str().unwrap(),
                    _ => "",
                };
            }

            return Some(new_content)
        }

        return None
    }

    async fn extract_postings(&self, content: &String) -> Result<Vec<posting::Model>> {
        let (mut start, mut end, mut message_start) = (0, 0, 0);
        let mut postings: Vec<posting::Model> = vec![];

        while end < content.len() && end / CONTEXT_MAX_CHARS < MAX_REQUESTS_PER_SITE {
            let mut message_parts: Vec<String> = vec![];
            while end < message_start + CONTEXT_MAX_CHARS && end < content.len() {
                end = *vec![start + MESSAGE_MAX_CHARS, message_start + CONTEXT_MAX_CHARS, content.len()].iter().min().unwrap_or(&0);
                let message_part = content[start..end].to_string();
                message_parts.push(message_part);
                start = end;
            }

            let response: ExtractResponse = serde_json::from_str(&(self.chatgpt_extract_postings(&message_parts).await?))?;
            postings.extend(response.postings);
            let message = message_parts.join("");
            let (last_index, _) = message.match_indices(&response.last_posting).last().unwrap_or((message.len() - CONTEXT_OVERLAP, ""));
            start = message_start + last_index;
            message_start = start;
        }

        Ok(postings)
    }

    async fn chatgpt_extract_postings(&self, message_parts: &Vec<String>) -> Result<String> {  
        let client = ChatGPT::new_with_config(
            &self.settings.api_key,
            ModelConfigurationBuilder::default()
                .engine(ChatGPTEngine::Gpt35Turbo)
                .timeout(Duration::new(60, 0))
                .temperature(0.01)
                .build()
                .unwrap(),
        ).unwrap();

        let criteria = self.filters.iter().fold(
            "".to_string(),
            |cur: String, next: &filter::Model| cur + &format!("{}: {}", next.name, next.value)
        );
    
        let message_start = format!("Extract job postings from the provided input. The input is split over multiple parts. \
            Return the job postings matching the provided criteria as 'postings'. \
            Criteria: {{{criteria}}} \
            Expected Return Format: JSON
            Expected Return Structure: {{ postings: [{{title: '', description: ''}}], last_posting: '' }}
            If no matching results return: []
            Return the title of the last job posting that does not match the criteria as 'last_posting'. \
            Return actual results based on the input provided, do not return code examples. \
        ");
    
        let message_end = format!("All parts have been sent. Process the request and return the results. Expected Return Structure: {{ postings: [{{title: '', description: ''}}], last_posting: '' }}");
    
        let message_to_be_continued = format!("Do not answer yet. This is just another part of the input. \
            Just receive and aknowledge with 'Part received'. And wait for the next part of the input.
        ");
    
        let total_parts = message_parts.len() - 1;
        let mut response: String = "[]".to_string();
    
        for (i, message_part) in message_parts.iter().enumerate() {
            let formatted_part = format!("[INPUT START PART {i}/{total_parts}] \
                {message_part}
                [INPUT END PART {i}/{total_parts}]\
            ");
    
            let mut message: String = "".to_string();
            if i == 0 {
                message = message_start.clone();
            }
    
            message += &formatted_part;
    
            if i == total_parts {
                message += &message_end;
            } else {
                message += &message_to_be_continued;
            }
    
            response = client.send_message(message).await?.message().content.to_string();
        }
    
        Ok(response)
    }
}