use crate::entities::{prelude::*, *};
use crate::extract::PostingsExtractorHandler;

use futures::lock::Mutex;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use sea_orm::{entity::*, query::*};
use sea_orm::{sea_query::Expr, *};

use std::sync::Arc;

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

#[get("/postings/bookmarked")]
pub async fn bookmarked_postings(db: &State<DatabaseConnection>) -> Result<Json<Vec<posting::Model>>, Status> {
	let db = db as &DatabaseConnection;

	Ok(Json(
		Posting::find()
			.filter(posting::Column::Bookmarked.eq(true))
			.order_by_desc(posting::Column::CreatedAt)
			.all(db)
			.await
			.expect("Could not retrieve postings")
			.into_iter()
			.collect::<Vec<_>>(),
	))
}

#[get("/postings/refresh")]
pub async fn refresh_postings(db: &State<DatabaseConnection>, extractor_handler: &State<Arc<Mutex<PostingsExtractorHandler>>>) -> Result<Json<Vec<posting::Model>>, Status> {
	let db_connection = db as &DatabaseConnection;
	let mut extractor_handler = extractor_handler.inner().lock().await;

	let _ = extractor_handler.refresh(db_connection).await;
	let _ = extractor_handler.save(db_connection).await;

	unread_postings(db).await
}

#[get("/postings/<id>")]
pub async fn posting_by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Json<Option<posting::Model>>, Status> {
	let db = db as &DatabaseConnection;

	Ok(Json(Posting::find().filter(posting::Column::Id.eq(id)).one(db).await.expect("Could not retrieve posting")))
}

#[put("/postings/mark_read", data = "<input>")]
pub async fn mark_postings_read(db: &State<DatabaseConnection>, input: Json<Vec<i32>>) -> Result<(), Status> {
	let db = db as &DatabaseConnection;

	let _ = Posting::update_many()
		.col_expr(posting::Column::Seen, Expr::value(true))
		.filter(posting::Column::Id.is_in(input.into_inner()))
		.exec(db)
		.await
		.expect("Could not mark posting as read");

	Ok(())
}

#[put("/postings/<id>", data = "<input>")]
pub async fn update_posting(db: &State<DatabaseConnection>, id: i32, input: Json<posting::Model>) -> Result<Json<posting::Model>, Status> {
	let db = db as &DatabaseConnection;

	let existing_posting = Posting::find_by_id(id).one(db).await.expect("Could not find posting");
	let mut existing_posting: posting::ActiveModel = existing_posting.unwrap().into();
	let updated_posting: posting::Model = input.into_inner();

	existing_posting.seen = Set(updated_posting.seen);
	existing_posting.bookmarked = Set(updated_posting.bookmarked);

	let existing_posting: posting::Model = existing_posting.update(db).await.expect("Could not update posting");

	Ok(Json(existing_posting))
}
