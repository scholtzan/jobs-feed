use crate::openai::OpenAIApi;
use crate::openai::BASE_URL;
use reqwest::StatusCode;
use std::cmp;

use serde_json::json;
use serde_json::Value;

use std::time::Duration;

use anyhow::anyhow;
use anyhow::Result;

const EMBEDDING_MODEL: &str = "text-embedding-3-small";

pub struct Embeddings {
	api_key: String,
	pub model: String,
}

impl OpenAIApi for Embeddings {
	fn api_key(&self) -> &String {
		&self.api_key
	}
}

impl Embeddings {
	pub fn new(api_key: &String) -> Self {
		Self {
			api_key: api_key.clone(),
			model: EMBEDDING_MODEL.to_string(),
		}
	}

	pub async fn create(&self, input: &String) -> Result<Vec<f32>> {
		let url = format!("{BASE_URL}/embeddings");
		let headers = self.headers()?;

		let body = json!({
			"input": input,
			"model": &self.model,
			"encoding_format": "float"
		});

		let client = self.client();
		let res = client.post(url).headers(headers).json(&body).send().await?;

		if res.status() == StatusCode::OK {
			let response_body = res.json::<Value>().await?;
			let data = response_body.get("data").unwrap().as_array().unwrap();
			if data.len() > 0 {
				return Ok(data[0].get("embedding").unwrap().as_array().unwrap().iter().map(|e| e.as_f64().unwrap() as f32).collect::<Vec<f32>>());
			}
		}

		return Err(anyhow!("Could not get embedding"));
	}

	// https://www.simonwenkel.com/notes/ai/metrics/cosine_distance.html
	pub fn get_similarity(&self, vec_a: &Vec<f32>, vecs_b: &Vec<Vec<f32>>) -> f32 {
		vecs_b
			.iter()
			.map(|vec_b| {
				let mut a_dot_b: f32 = 0.0;
				let mut a_mag: f32 = 0.0;
				let mut b_mag: f32 = 0.0;
				let vec_size: usize = cmp::min(vec_a.len(), vec_b.len());

				for i in 0..vec_size as usize {
					a_dot_b += vec_a[i] * vec_b[i];
					a_mag += vec_a[i] * vec_a[i];
					b_mag += vec_b[i] * vec_b[i];
				}

				let dist: f32 = a_dot_b / (a_mag.sqrt() * b_mag.sqrt());

				return dist;
			})
			.max_by(|a, b| a.total_cmp(b))
			.unwrap_or(0.0)
	}
}
