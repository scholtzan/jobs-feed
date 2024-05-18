use crate::entities::{prelude::*, *};
use chrono::FixedOffset;

use crate::pool::Db;
use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use sea_orm::*;

/// Returns all existing filters.
#[get("/filters")]
pub async fn filters(conn: Connection<'_, Db>) -> Result<Json<Vec<filter::Model>>, Status> {
	let db = conn.into_inner();

	Ok(Json(Filter::find().all(db).await.expect("Could not retrieve filters").into_iter().collect::<Vec<_>>()))
}

/// Update the filters.
/// This method is also used to create new filters and delete existing filters
/// as it will overwrite all the stored filters.
///
/// Returns filters that got created.
#[put("/filters", data = "<input>")]
pub async fn update_filters(conn: Connection<'_, Db>, input: Json<Vec<filter::Model>>) -> Result<Json<Vec<filter::Model>>, Status> {
	let db = conn.into_inner();

	let txn = db.begin().await.expect("Could not create transaction.");
	// delete existing filters
	Filter::delete_many().exec(&txn).await.expect("Could not delete filters.");
	// and add the filters that were provided in the request
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

	// return filters that were created
	Ok(Json(Filter::find().all(db).await.expect("Could not retrieve filters").into_iter().collect::<Vec<_>>()))
}
