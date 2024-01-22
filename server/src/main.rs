mod entities;
mod migration;
mod setup;

#[macro_use]
extern crate rocket;

use rocket::{
    fs::{relative, NamedFile},
    shield::Shield,
};

use futures::executor::block_on;
use serde::Deserialize;
use serde_json::{json, Value};
use setup::set_up_db;
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

// todo: make configurable
const DIST: &str = relative!("dist");


#[get("/sources")]
async fn sources(db: &State<DatabaseConnection>) -> Result<Json<Vec<source::Model>>, Status> {
    let db = db as &DatabaseConnection;

    Ok(Json(
        Source::find()
            .all(db)
            .await
            .expect("could not retrieve cakes")
            .into_iter()
            .collect::<Vec<_>>(),
    ))

}

#[get("/<file..>", rank = 1)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(DIST).join("assets/").join(file))
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
        name: String,
        url: String,
    }

    let config: DatabaseConfig = figment.extract_inner::<DatabaseConfig>("database")
        .unwrap();
    
    let db = match set_up_db(&config.url, &config.name,).await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build()
        .manage(db)
        .mount("/assets", routes![static_files])
        .mount(
            "/",
            routes![sources],
        )
        .mount("/", routes![index])
}
