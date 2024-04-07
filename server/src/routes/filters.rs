use crate::entities::{prelude::*, *};
use chrono::FixedOffset;

use crate::pool::Db;
use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use sea_orm::*;

#[get("/filters")]
pub async fn filters(conn: Connection<'_, Db>) -> Result<Json<Vec<filter::Model>>, Status> {
	let db = conn.into_inner();

	Ok(Json(Filter::find().all(db).await.expect("Could not retrieve filters").into_iter().collect::<Vec<_>>()))
}

#[put("/filters", data = "<input>")]
pub async fn update_filters(conn: Connection<'_, Db>, input: Json<Vec<filter::Model>>) -> Result<Json<Vec<filter::Model>>, Status> {
	let db = conn.into_inner();

	let txn = db.begin().await.expect("Could not create transaction.");

	Filter::delete_many().exec(&txn).await.expect("Could not delete filters.");

	let filter_models: Vec<filter::ActiveModel> = input
		.into_inner()
		.into_iter()
		.map(|f| {
			let mut new_filter: filter::ActiveModel = f.into();
			new_filter.id = NotSet;
			new_filter.created_at = Set(Some(chrono::offset::Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())));
			new_filter
		})
		.collect();

	Filter::insert_many(filter_models).exec(&txn).await.expect("Could not update filters");

	txn.commit().await.expect("Cannot commit transaction");

	Ok(Json(Filter::find().all(db).await.expect("Could not retrieve filters").into_iter().collect::<Vec<_>>()))
}
