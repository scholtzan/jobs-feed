use anyhow::anyhow;
use anyhow::Result;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

use crate::openai::OpenAIApi;
use crate::openai::BASE_URL;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, Instant};

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

pub enum AssistantType {
	JobsFeed,
	JobsSuggestion,
}

impl AssistantType {
	fn name(&self) -> &'static str {
		match self {
			AssistantType::JobsFeed => "Jobs Feed",
			AssistantType::JobsSuggestion => "Jobs Suggestion",
		}
	}

	fn instructions(&self) -> &'static str {
		match self {
			AssistantType::JobsFeed => {
				"Extract a complete list of job postings with descriptions from the provided inputs that match the provided criteria. \
            Return the results in a single response as JSON. \
            Extract the job descriptions and shorted to 200 characters, if available. Do not invent a job description! Do not miss any posting! \
            Response format: [{{\"title\": \"\", \"description\": \"\"}}]"
			}
			AssistantType::JobsSuggestion => {
				"Return a list of 10 career websites of companies similar to the company provided as input. \
            Response format: [{{\"name\": \"\", \"url\": \"\"}}]"
			}
		}
	}
}

impl OpenAIApi for Assistant {
	fn api_key(&self) -> &String {
		return &self.api_key;
	}
}

pub struct Assistant {
	api_key: String,
	pub model: String,
	id: Option<String>,
	assistant_type: AssistantType,
	pub usage: Usage,
}

impl Assistant {
	pub async fn new(api_key: &String, model: &String, assistant_type: AssistantType) -> Result<Self> {
		let mut assistant = Self {
			api_key: api_key.clone(),
			model: model.clone(),
			id: None,
			usage: Usage::default(),
			assistant_type: assistant_type,
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
			if let Some(assistant) = data.into_iter().find(|&a| a.get("name").unwrap().as_str().unwrap() == self.assistant_type.name()) {
				return Ok(Some(assistant.get("id").unwrap().as_str().unwrap().to_string()));
			}
		}

		return Ok(None);
	}

	async fn create(&self) -> Result<Option<String>> {
		let url = format!("{BASE_URL}/assistants");
		let headers = self.headers()?;

		let body = json!({
			"instructions": self.assistant_type.instructions(),
			"name": self.assistant_type.name(),
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
		let headers = self.headers()?;

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
			"stream": true,
			"thread": json!({
				"messages": messages_json
			})
		});

		let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?;
		let mut stream = client.post(url).headers(headers).json(&body).send().await?.bytes_stream();

		let mut thread_id: String = "".to_string();
		let mut run_id: String = "".to_string();
		let mut is_run_metadata = false;
		let mut buffer = Vec::new();
		while let Some(item) = stream.next().await {
			for byte in item? {
				// Might need to consider carriage returns too depending
				// on how the server is expected to send the data.
				let mut buf: String = "".to_string();
				if byte == b'\n' {
					let _ = buffer.as_slice().read_to_string(&mut buf);
					if buf == "data: [DONE]" {
						break;
					} else if buf == "event: thread.run.created" {
						is_run_metadata = true;
					} else if is_run_metadata {
						let json_metadata = buf.replace("data: ", "");
						let run_metadata: Value = serde_json::from_str(&json_metadata).unwrap();
						thread_id = run_metadata.get("thread_id").unwrap().as_str().unwrap().to_string();
						run_id = run_metadata.get("id").unwrap().as_str().unwrap().to_string();
						is_run_metadata = false;
					}
					buffer.clear();
				} else {
					buffer.push(byte);
				}
			}
		}

		let result = self.get_run_result(&thread_id, &run_id).await;

		result
	}

	async fn get_run_result(&self, thread_id: &str, run_id: &str) -> Result<String> {
		let url = format!("{BASE_URL}/threads/{thread_id}/messages");
		let headers = self.headers()?;
		let client = reqwest::Client::builder().timeout(Duration::from_secs(10)).build()?;

		let res = client.get(&url).headers(headers.clone()).send().await?;

		if res.status() == StatusCode::OK {
			let response_body = res.json::<Value>().await?;
			let data = response_body.get("data").unwrap().as_array().unwrap();

			let messages: Vec<&Value> = data
				.into_iter()
				.filter(|&m| m.get("run_id").unwrap_or(&Value::String("".to_string())).as_str() == Some(run_id))
				.collect();

			eprintln!("total messages {:?}", messages.len());
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

				// todo: support multiple messages
				return Ok(content.to_string());
			}
		} else {
			return Err(anyhow!("Cannot get run"));
		}

		return Err(anyhow!("No message returned by run"));
	}
}
