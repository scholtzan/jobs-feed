use crate::entities::{prelude::*, *};

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use sea_orm::*;

#[get("/suggestions")]
pub async fn suggestions(db: &State<DatabaseConnection>) -> Result<Json<Vec<suggestion::Model>>, Status> {
	let db = db as &DatabaseConnection;

	Ok(Json(
		Suggestion::find().limit(10).all(db).await.expect("Could not retrieve suggestions").into_iter().collect::<Vec<_>>(),
	))
}
