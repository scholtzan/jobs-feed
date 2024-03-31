use anyhow::anyhow;
use anyhow::Result;

use reqwest::StatusCode;
use serde_json::json;
use serde_json::Value;

use crate::openai::OpenAIApi;
use crate::openai::BASE_URL;
use futures_util::StreamExt;
use std::io::Read;

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
				"Extract a complete list of job posting titles from the provided inputs that are related to the provided criteria. \
            Return the results in a single response as JSON. \
			Only return postings that are in the input. Do not miss any posting! \
            Response format: [{{\"title\":\"\"}}]"
			}
			AssistantType::JobsSuggestion => {
				"Return a list of 10 career websites of companies similar to the company provided as input. \
            Response format: [{{\"name\":\"\",\"url\":\"\"}}]"
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
}

impl Assistant {
	pub async fn new(api_key: &String, model: &String, assistant_type: AssistantType) -> Result<Self> {
		let mut assistant = Self {
			api_key: api_key.clone(),
			model: model.clone(),
			id: None,
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
		let client = self.client();
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
		let client = self.client();
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
		let client = self.client();

		let body = json!({
			"instructions": self.assistant_type.instructions(),
			"name": self.assistant_type.name(),
			"model": &self.model
		});

		let res = client.post(url).headers(headers).json(&body).send().await?;

		if res.status() == StatusCode::OK {
			let response_body = res.json::<Value>().await?;
			let id = response_body.get("id").unwrap().as_str().unwrap();
			return Ok(Some(id.to_string()));
		} else {
			return Err(anyhow!("Cannot create assistant"));
		}
	}

	pub async fn run(&mut self, messages: &Vec<String>) -> Result<Vec<String>> {
		let url = format!("{BASE_URL}/threads/runs");
		let headers = self.headers()?;
		let client = self.client();

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

	async fn get_run_result(&self, thread_id: &str, run_id: &str) -> Result<Vec<String>> {
		let url = format!("{BASE_URL}/threads/{thread_id}/messages");
		let headers = self.headers()?;
		let client = self.client();

		let res = client.get(&url).headers(headers.clone()).send().await?;

		if res.status() == StatusCode::OK {
			let response_body = res.json::<Value>().await?;
			let data = response_body.get("data").unwrap().as_array().unwrap();

			let messages: Vec<&Value> = data
				.into_iter()
				.filter(|&m| m.get("run_id").unwrap_or(&Value::String("".to_string())).as_str() == Some(run_id))
				.collect();

			let result: Vec<String> = messages
				.iter()
				.map(|m| {
					m.get("content")
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
						.unwrap()
						.to_string()
				})
				.collect();

			return Ok(result);
		}
		return Err(anyhow!("Cannot get run"));
	}
}
