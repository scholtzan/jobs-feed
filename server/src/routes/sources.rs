use rocket::{
    fs::{relative, NamedFile},
    shield::Shield,
};

use futures::executor::block_on;
use serde::Deserialize;
use serde_json::{json, Value};
use sea_orm::*;
use sea_orm::{entity::*, error::*, query::*, FromQueryResult};
use crate::entities::{prelude::*, *};
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::State;
use std::{
    env,
    path::{Path, PathBuf},
};
use rocket::http::ContentType;

#[get("/sources")]
pub async fn sources(db: &State<DatabaseConnection>) -> Result<Json<Vec<source::Model>>, Status> {
    let db = db as &DatabaseConnection;

    Ok(Json(
        Source::find()
            .all(db)
            .await
            .expect("Could not retrieve sources")
            .into_iter()
            .collect::<Vec<_>>(),
    ))
}

#[post("/sources", data = "<input>")]
pub async fn add_source(
    db: &State<DatabaseConnection>,
    input: Json<source::Model>,
) -> Result<Json<source::Model>, Status> {
    let db = db as &DatabaseConnection;
    let mut new_source: source::ActiveModel = input.into_inner().into();
    new_source.set(source::Column::CreatedAt, chrono::offset::Utc::now().into());
    let inserted_source: source::Model = new_source.insert(db).await.expect("Could not insert source");
    Ok(
        Json(
            inserted_source
        )
    )
}