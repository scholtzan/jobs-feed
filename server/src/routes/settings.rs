use crate::entities;
use crate::entities::prelude::*;
use crate::openai::assistant::{Assistant, AssistantType};

use crate::pool::Db;
use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use sea_orm::*;

/// Return all stored settings.
#[get("/settings")]
pub async fn settings(conn: Connection<'_, Db>) -> Result<Json<Option<entities::settings::Model>>, Status> {
	let db = conn.into_inner();

	Ok(Json(Settings::find().one(db).await.expect("Could not retrieve settings")))
}

/// Update stored settings.
/// The request body is expected to contain the new setting values.
///
/// Return updated settings.
#[put("/settings", data = "<input>")]
pub async fn update_settings(conn: Connection<'_, Db>, input: Json<entities::settings::Model>) -> Result<Json<Option<entities::settings::Model>>, Status> {
	let db = conn.into_inner();

	let txn = db.begin().await.expect("Could not create transaction.");

	// delete existing settings and overwrite
	Settings::delete_many().exec(&txn).await.expect("Could not delete settings.");
	let new_settings: entities::settings::ActiveModel = input.into_inner().into();
	Settings::insert(new_settings).exec(&txn).await.expect("Could not update settings");

	txn.commit().await.expect("Cannot commit transaction");

	Ok(Json(Settings::find().one(db).await.expect("Could not retrieve settings")))
}

/// Get available LLM models.
///
/// Return list of model names.
#[get("/settings/models")]
pub async fn get_models(conn: Connection<'_, Db>) -> Result<Json<Vec<String>>, Status> {
	let db = conn.into_inner();
	let settings = Settings::find().one(db).await.expect("Could not retrieve settings");

	if settings.is_none() {
		return Ok(Json(vec![]));
	}

	// get models via the OpenAI API
	let assistant = Assistant::new(&settings.unwrap().api_key.unwrap(), &"".to_string(), AssistantType::JobsFeed)
		.await
		.expect("Could not retrieve model information");
	let models = assistant.get_models().await.expect("Could not retrieve model information");

	Ok(Json(models))
}
