use crate::entities::{prelude::*, *};
use chrono::{DateTime, FixedOffset, Local, Utc};
use futures::executor::block_on;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{
	fs::{relative, NamedFile},
	shield::Shield,
};
use sea_orm::*;
use sea_orm::{entity::*, error::*, query::*, FromQueryResult};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
	env,
	path::{Path, PathBuf},
};

#[get("/filters")]
pub async fn filters(db: &State<DatabaseConnection>) -> Result<Json<Vec<filter::Model>>, Status> {
	let db = db as &DatabaseConnection;

	Ok(Json(Filter::find().all(db).await.expect("Could not retrieve filters").into_iter().collect::<Vec<_>>()))
}

#[put("/filters", data = "<input>")]
pub async fn update_filters(db_state: &State<DatabaseConnection>, input: Json<Vec<filter::Model>>) -> Result<Json<Vec<filter::Model>>, Status> {
	let db = db_state as &DatabaseConnection;

	let txn = db.begin().await.expect("Could not create transaction.");

	Filter::delete_many().exec(&txn).await.expect("Could not delete filters.");

	let filter_models: Vec<filter::ActiveModel> = input
		.into_inner()
		.into_iter()
		.map(|f| {
			let mut new_filter: filter::ActiveModel = f.into();
			new_filter.id = NotSet;
			new_filter.created_at = Set(Some(chrono::offset::Utc::now().with_timezone(&FixedOffset::east(0))));
			new_filter
		})
		.collect();

	Filter::insert_many(filter_models).exec(&txn).await.expect("Could not update filters");

	txn.commit().await.expect("Cannot commit transaction");

	filters(db_state).await
}
