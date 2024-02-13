mod assistant;
mod entities;
mod extract;
mod routes;
mod util;

#[macro_use]
extern crate rocket;

use rocket::{
    fs::{relative, NamedFile},
    shield::Shield,
};

use futures::executor::block_on;
use serde::Deserialize;
use serde_json::{json, Value};
use sea_orm::*;
use sea_orm::{entity::*, error::*, query::*, FromQueryResult};
use entities::{prelude::*, *};
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
use crate::assistant::Assistant;


// todo: make configurable
const DIST: &str = relative!("dist");


#[get("/<file..>", rank = 1)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(DIST).join("_app/").join(file))
        .await
        .ok()
}

#[get("/<_..>", rank = 2)]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new(DIST).join("index.html"))
        .await
        .ok()
}


#[launch]
async fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    #[derive(Deserialize)]
    #[serde(crate = "rocket::serde")]
    struct DatabaseConfig {
        url: String,
    }

    let config: DatabaseConfig = figment.extract_inner::<DatabaseConfig>("database")
        .unwrap();

    let db = Database::connect(config.url)
    .await
    .unwrap();

    let postings_extractor_handler = PostingsExtractorHandler::new();

    rocket::build()
        .manage(db)
        .manage(Arc::new(Mutex::new(postings_extractor_handler)))
        .mount("/_app", routes![static_files])
        .mount(
            "/",
            routes![
                routes::sources::sources,
                routes::sources::add_source,
                routes::sources::source_by_id,
                routes::sources::delete_source,
                routes::sources::update_source,
                routes::filters::filters,
                routes::filters::update_filters,
                routes::postings::unread_postings,
                routes::postings::refresh_postings,
                routes::postings::posting_by_id,
                routes::postings::mark_postings_read,
                routes::postings::update_posting,
                routes::postings::bookmarked_postings,
                routes::settings::settings,
                routes::settings::update_settings,
            ],
        )
        .mount("/", routes![index])
}
