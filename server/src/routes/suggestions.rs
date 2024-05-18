use crate::entities::{prelude::*, *};

use crate::pool::Db;
use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use sea_orm::*;

/// Returns all source suggestions.
#[get("/suggestions")]
pub async fn suggestions(conn: Connection<'_, Db>) -> Result<Json<Vec<suggestion::Model>>, Status> {
	let db = conn.into_inner();

	Ok(Json(
		Suggestion::find().limit(10).all(db).await.expect("Could not retrieve suggestions").into_iter().collect::<Vec<_>>(),
	))
}
