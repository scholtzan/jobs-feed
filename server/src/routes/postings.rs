use rocket::{
    fs::{relative, NamedFile},
    shield::Shield,
};
use chrono::{DateTime, Local, FixedOffset, Utc};
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
use crate::extract::PostingsExtractorHandler;
use std::sync::{Arc};
use futures::lock::Mutex;


#[get("/postings/unread")]
pub async fn unread_postings(db: &State<DatabaseConnection>) -> Result<Json<Vec<posting::Model>>, Status> {
    let db = db as &DatabaseConnection;

    Ok(Json(
        Posting::find()
            .filter(posting::Column::Seen.eq(false))
            .order_by_desc(posting::Column::CreatedAt)
            .all(db)
            .await
            .expect("Could not retrieve postings")
            .into_iter()
            .collect::<Vec<_>>(),
    ))
}

#[get("/postings/refresh")]
pub async fn refresh_postings(
    db: &State<DatabaseConnection>,
    extractor_handler: &State<Arc<Mutex<PostingsExtractorHandler>>>
) -> Result<Json<Vec<posting::Model>>, Status> {
    let db_connection = db as &DatabaseConnection;
    let mut extractor_handler = extractor_handler.inner().lock().await;

    let _ = extractor_handler.refresh(db_connection).await;
    let _ = extractor_handler.save(db_connection).await;

    unread_postings(db).await
}