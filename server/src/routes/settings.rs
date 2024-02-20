use crate::assistant::Assistant;
use crate::entities;
use crate::entities::prelude::*;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use sea_orm::*;

#[get("/settings")]
pub async fn settings(db: &State<DatabaseConnection>) -> Result<Json<Option<entities::settings::Model>>, Status> {
	let db = db as &DatabaseConnection;

	Ok(Json(Settings::find().one(db).await.expect("Could not retrieve settings")))
}

#[put("/settings", data = "<input>")]
pub async fn update_settings(db_state: &State<DatabaseConnection>, input: Json<entities::settings::Model>) -> Result<Json<Option<entities::settings::Model>>, Status> {
	let db = db_state as &DatabaseConnection;

	let txn = db.begin().await.expect("Could not create transaction.");

	Settings::delete_many().exec(&txn).await.expect("Could not delete settings.");

	let new_settings: entities::settings::ActiveModel = input.into_inner().into();
	Settings::insert(new_settings).exec(&txn).await.expect("Could not update settings");

	txn.commit().await.expect("Cannot commit transaction");

	settings(db_state).await
}

#[get("/settings/models")]
pub async fn get_models(db: &State<DatabaseConnection>) -> Result<Json<Vec<String>>, Status> {
	let db = db as &DatabaseConnection;
	let settings = Settings::find().one(db).await.expect("Could not retrieve settings");

	if settings.is_none() {
		return Ok(Json(vec![]));
	}

	let assistant = Assistant::new(&settings.unwrap().api_key.unwrap(), &"".to_string())
		.await
		.expect("Could not retrieve model information");
	let models = assistant.get_models().await.expect("Could not retrieve model information");

	Ok(Json(models))
}
