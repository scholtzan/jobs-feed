use crate::entities;
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
