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

#[get("/sources")]
pub async fn sources(db: &State<DatabaseConnection>) -> Result<Json<Vec<source::Model>>, Status> {
	let db = db as &DatabaseConnection;

	Ok(Json(Source::find().all(db).await.expect("Could not retrieve sources").into_iter().collect::<Vec<_>>()))
}

#[post("/sources", data = "<input>")]
pub async fn add_source(db: &State<DatabaseConnection>, input: Json<source::Model>) -> Result<Json<source::Model>, Status> {
	let db = db as &DatabaseConnection;

	let mut new_source: source::ActiveModel = input.into_inner().into();
	new_source.id = NotSet;
	new_source.created_at = Set(Some(chrono::offset::Utc::now().with_timezone(&FixedOffset::east(0))));
	let inserted_source: source::Model = new_source.insert(db).await.expect("Could not insert source");
	Ok(Json(inserted_source))
}

#[delete("/sources/<id>")]
pub async fn delete_source(db: &State<DatabaseConnection>, id: i32) -> Result<(), Status> {
	let db = db as &DatabaseConnection;

	let source: Option<source::Model> = Source::find_by_id(id).one(db).await.expect("Could not find source");
	let source: source::Model = source.unwrap();

	let res: DeleteResult = source.delete(db).await.expect("Cannot delete source");

	Ok(())
}

#[get("/sources/<id>")]
pub async fn source_by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Json<Option<source::Model>>, Status> {
	let db = db as &DatabaseConnection;

	Ok(Json(Source::find().filter(source::Column::Id.eq(id)).one(db).await.expect("Could not retrieve source")))
}

#[put("/sources/<id>", data = "<input>")]
pub async fn update_source(db: &State<DatabaseConnection>, id: i32, input: Json<source::Model>) -> Result<Json<source::Model>, Status> {
	let db = db as &DatabaseConnection;

	let mut existing_source = Source::find_by_id(id).one(db).await.expect("Could not find source").unwrap();
	let mut updated_source: source::Model = input.into_inner();
	let content_changed = existing_source.url != updated_source.url || existing_source.selector != updated_source.selector || existing_source.pagination != updated_source.pagination;

	let mut existing_source_active: source::ActiveModel = existing_source.into();

	if content_changed {
		existing_source_active.content = Set(Some("".to_string()));
	}

	existing_source_active.name = Set(updated_source.name);
	existing_source_active.url = Set(updated_source.url);
	existing_source_active.selector = Set(updated_source.selector);
	existing_source_active.pagination = Set(updated_source.pagination);

	let existing_source: source::Model = existing_source_active.update(db).await.expect("Could not update source");

	Ok(Json(existing_source))
}
