use anyhow::anyhow;
use anyhow::Result;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};

const ASSISTANT_NAME: &str = "Jobs Feed";
const BASE_URL: &str = "https://api.openai.com/v1";

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Usage {
	pub prompt_tokens: i32,
	pub completion_tokens: i32,
	pub total_tokens: i32,
}

impl Usage {
	pub fn add(&mut self, other: Usage) {
		self.prompt_tokens += other.prompt_tokens;
		self.completion_tokens += other.completion_tokens;
		self.total_tokens += other.total_tokens;
	}

	pub fn get_cost(&self, model: &str) -> Option<f32> {
		// todo: better configuration
		let pricing = HashMap::from([
			("gpt-4", (0.03, 0.06)),
			("gpt-4-32k", (0.06, 0.12)),
			("gpt-4-0125-preview", (0.01, 0.03)),
			("gpt-4-1106-preview", (0.01, 0.03)),
			("gpt-3.5-turbo-0125", (0.0005, 0.0015)),
			("gpt-3.5-turbo-instruct", (0.0015, 0.0020)),
			("gpt-3.5-turbo", (0.0005, 0.0015)),
		]);

		match pricing.get(model) {
			Some((input, output)) => Some(self.prompt_tokens as f32 / 1000.0 * *input + self.completion_tokens as f32 / 1000.0 * *output),
			None => None,
		}
	}
}

pub struct Assistant {
	api_key: String,
	model: String,
	id: Option<String>,
	pub usage: Usage,
}

impl Assistant {
	pub async fn new(api_key: &String, model: &String) -> Result<Self> {
		let mut assistant = Self {
			api_key: api_key.clone(),
			model: model.clone(),
			id: None,
			usage: Usage::default(),
		};

		if let Some(assistant_id) = assistant.get().await? {
			assistant.id = Some(assistant_id);
		} else {
			assistant.id = assistant.create().await?;
		}

		Ok(assistant)
	}

	pub async fn get_models(&self) -> Result<Vec<String>> {
		let url = format!("{BASE_URL}/models");
		let headers = self.headers()?;
		let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?;
		let res = client.get(url).headers(headers).send().await?;

		if res.status() == StatusCode::OK {
			let response_body = res.json::<Value>().await?;
			let data = response_body.get("data").unwrap().as_array().unwrap();
			let models = data.into_iter().map(|d| d.get("id").unwrap().as_str().unwrap().to_string()).collect();
			return Ok(models);
		}

		return Err(anyhow!("Couldn't get models"));
	}

	async fn get(&self) -> Result<Option<String>> {
		let url = format!("{BASE_URL}/assistants");
		let headers = self.headers()?;

		let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?;
		let res = client.get(url).headers(headers).send().await?;

		if res.status() == StatusCode::OK {
			let response_body = res.json::<Value>().await?;
			let data = response_body.get("data").unwrap().as_array().unwrap();
			if let Some(assistant) = data.into_iter().find(|&a| a.get("name").unwrap().as_str().unwrap() == ASSISTANT_NAME) {
				return Ok(Some(assistant.get("id").unwrap().as_str().unwrap().to_string()));
			}
		}

		return Ok(None);
	}

	fn headers(&self) -> Result<HeaderMap> {
		let bearer = format!("Bearer {}", self.api_key);
		let mut headers = HeaderMap::new();
		headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
		headers.insert(AUTHORIZATION, bearer.parse()?);
		headers.insert(HeaderName::from_static("openai-beta"), HeaderValue::from_static("assistants=v1"));

		Ok(headers)
	}

	async fn create(&self) -> Result<Option<String>> {
		let url = format!("{BASE_URL}/assistants");
		let headers = self.headers()?;

		let body = json!({
			"instructions": "Extract a complete list of job postings with descriptions from the provided inputs that match the provided criteria. \
				Return the results in a single response as JSON. \
				Extract the job descriptions and shorted to 200 characters. Do not miss any posting! \
				Response format: [{{\"title\": \"\", \"description\": \"\"}}]",
			"name": "Jobs Feed",
			"model": &self.model
		});

		let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?;

		let res = client.post(url).headers(headers).json(&body).send().await?;

		if res.status() == StatusCode::OK {
			let response_body = res.json::<Value>().await?;
			let id = response_body.get("id").unwrap().as_str().unwrap();
			return Ok(Some(id.to_string()));
		} else {
			return Err(anyhow!("Cannot create assistant"));
		}
	}

	pub async fn run(&mut self, messages: &Vec<String>) -> Result<String> {
		self.usage = Usage::default();
		let url = format!("{BASE_URL}/threads/runs");
		let bearer = format!("Bearer {}", self.api_key);
		let mut headers = HeaderMap::new();
		headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
		headers.insert(AUTHORIZATION, bearer.parse()?);
		headers.insert(HeaderName::from_static("openai-beta"), HeaderValue::from_static("assistants=v1"));

		let messages_json: Vec<Value> = messages
			.into_iter()
			.map(|m| {
				json!({
					"content": m,
					"role": &"user".to_string()
				})
			})
			.collect();

		let body = json!({
			"assistant_id": self.id.clone().unwrap_or("".to_string()),
			"thread": json!({
				"messages": messages_json
			})
		});

		let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?;

		let res = client.post(url).headers(headers).json(&body).send().await?;

		if res.status() != StatusCode::OK {
			return Err(anyhow!("Cannot create thread"));
		}

		let response_body = res.json::<Value>().await?;
		let thread_id = response_body.get("thread_id").unwrap().as_str().unwrap();
		let run_id = response_body.get("id").unwrap().as_str().unwrap();
		// todo: move to separate result() method?
		let usage = self.wait_for_run(thread_id, run_id, Duration::from_secs(30)).await?;
		let result = self.get_run_result(thread_id, run_id, Duration::from_secs(100)).await;
		self.usage.add(usage);
		result
	}

	async fn wait_for_run(&self, thread_id: &str, run_id: &str, timeout: Duration) -> Result<Usage> {
		let interval = Duration::from_secs(1);
		let end = Instant::now() + timeout;
		let mut next_time = Instant::now() + interval;
		let mut finished = false;
		let mut usage = Usage::default();

		let url = format!("{BASE_URL}/threads/{thread_id}/runs/{run_id}");
		let bearer = format!("Bearer {}", self.api_key);
		let mut headers = HeaderMap::new();
		headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
		headers.insert(AUTHORIZATION, bearer.parse()?);
		headers.insert(HeaderName::from_static("openai-beta"), HeaderValue::from_static("assistants=v1"));
		let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?;

		while next_time < end || !finished {
			let res = client.get(&url).headers(headers.clone()).send().await?;

			if res.status() == StatusCode::OK {
				let response_body = res.json::<Value>().await?;
				let status = response_body.get("status").unwrap().as_str().unwrap();

				if status == "completed" {
					let run_usage: Usage = serde_json::from_value(response_body.get("usage").unwrap().clone())?;
					usage.add(run_usage);
					finished = true;
				} else if status != "queued" && status != "in_progress" {
					return Err(anyhow!("Run failed"));
				}
			} else {
				return Err(anyhow!("Cannot get run"));
			}

			sleep(next_time - Instant::now());
			next_time += interval;
		}

		if !finished {
			return Err(anyhow!("Run timed out"));
		}

		Ok(usage)
	}

	async fn get_run_result(&self, thread_id: &str, run_id: &str, timeout: Duration) -> Result<String> {
		let interval = Duration::from_secs(1);
		let end = Instant::now() + timeout;
		let mut next_time = Instant::now() + interval;

		let url = format!("{BASE_URL}/threads/{thread_id}/messages");
		let bearer = format!("Bearer {}", self.api_key);
		let mut headers = HeaderMap::new();
		headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
		headers.insert(AUTHORIZATION, bearer.parse()?);
		headers.insert(HeaderName::from_static("openai-beta"), HeaderValue::from_static("assistants=v1"));
		let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?;

		while next_time < end {
			let res = client.get(&url).headers(headers.clone()).send().await?;

			if res.status() == StatusCode::OK {
				let response_body = res.json::<Value>().await?;
				let data = response_body.get("data").unwrap().as_array().unwrap();

				let messages: Vec<&Value> = data
					.into_iter()
					.filter(|&m| m.get("run_id").unwrap_or(&Value::String("".to_string())).as_str() == Some(run_id))
					.collect();

				let total_messages = messages.len();

				for message in messages {
					let content = message
						.get("content")
						.unwrap()
						.as_array()
						.unwrap()
						.first()
						.unwrap()
						.get("text")
						.unwrap()
						.get("value")
						.unwrap()
						.as_str()
						.unwrap();

					if total_messages > 0 && content != "" {
						return Ok(content.to_string());
					}
				}
			} else {
				return Err(anyhow!("Cannot get run"));
			}

			sleep(next_time - Instant::now());
			next_time += interval;
		}

		return Err(anyhow!("No message returned by run"));
	}
}
