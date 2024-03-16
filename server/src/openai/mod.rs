pub mod assistant;
pub mod embeddings;

use anyhow::Result;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

const BASE_URL: &str = "https://api.openai.com/v1";

pub trait OpenAIApi {
	fn api_key(&self) -> &String;

	fn headers(&self) -> Result<HeaderMap> {
		let bearer = format!("Bearer {}", self.api_key());
		let mut headers = HeaderMap::new();
		headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
		headers.insert(AUTHORIZATION, bearer.parse()?);
		headers.insert(HeaderName::from_static("openai-beta"), HeaderValue::from_static("assistants=v1"));

		Ok(headers)
	}
}
