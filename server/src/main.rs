mod entities;
mod extract;
mod openai;
mod pool;
mod routes;
mod util;

#[macro_use]
extern crate rocket;

use pool::Db;
use rocket::fs::{relative, NamedFile};

use sea_orm::*;
use sea_orm_rocket::{Config, Database};

use std::{
	env,
	path::{Path, PathBuf},
};

use crate::entities::prelude::*;
use migration::MigratorTrait;

const DIST: &str = relative!("dist");

// serve any static file
#[get("/<file..>", rank = 2)]
async fn static_files(file: PathBuf) -> Option<NamedFile> {
	NamedFile::open(Path::new(DIST).join("_app/").join(file)).await.ok()
}

// serve the favicon
#[get("/favicon.svg", rank = 1)]
async fn favicon() -> Option<NamedFile> {
	NamedFile::open(Path::new(DIST).join("favicon.svg")).await.ok()
}

// serve the index.html
#[get("/<_..>", rank = 3)]
async fn index() -> Option<NamedFile> {
	NamedFile::open(Path::new(DIST).join("index.html")).await.ok()
}

#[launch]
async fn rocket() -> _ {
	let args: Vec<String> = env::args().collect();

	// determine the environment from the provided arguments
	let mut environment = "debug";
	if args.len() > 1 {
		environment = &args[1];
	}

	let rocket = rocket::build();

	// get the config based on the current environment
	let figment = rocket.figment().clone().select(environment);
	let config: Config = figment.extract::<Config>().unwrap();

	// run the migration if tables don't exist
	let db = sea_orm::Database::connect(config.url).await.unwrap();
	migration::Migrator::up(&db, None).await.unwrap();

	// create default settings if they haven't been stored yet
	match env::var("API_KEY") {
		Ok(api_key) => {
			if Settings::find().count(&db).await.unwrap() == 0 {
				let new_settings = entities::settings::ActiveModel {
					id: NotSet,
					api_key: Set(Some(api_key)),
					model: Set(Some("gpt-3.5-turbo".to_string())),
				};
				let _ = new_settings.insert(&db).await;
			}
		}
		_ => {}
	};

	// mount API routes
	rocket::custom(figment)
		.attach(Db::init())
		.mount("/_app", routes![static_files])
		.mount(
			"/api/v1",
			routes![
				routes::sources::sources,
				routes::sources::add_source,
				routes::sources::source_by_id,
				routes::sources::delete_source,
				routes::sources::update_source,
				routes::sources::reset_source_cache,
				routes::sources::refresh_source_suggestions,
				routes::sources::source_suggestions,
				routes::filters::filters,
				routes::filters::update_filters,
				routes::postings::unread_postings,
				routes::postings::refresh_postings,
				routes::postings::posting_by_id,
				routes::postings::mark_postings_read,
				routes::postings::update_posting,
				routes::postings::bookmarked_postings,
				routes::postings::get_postings,
				routes::settings::settings,
				routes::settings::update_settings,
				routes::settings::get_models,
				routes::suggestions::suggestions,
			],
		)
		.mount("/", routes![favicon, index])
}
