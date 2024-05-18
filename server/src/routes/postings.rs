use crate::entities::{prelude::*, *};
use crate::extract::PostingsExtractor;

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::pool::Db;
use sea_orm::sea_query::Expr;
use sea_orm::{entity::*, query::*};
use sea_orm_rocket::Connection;

/// Return all postings that have not been seen by the user.
#[get("/postings/unread")]
pub async fn unread_postings(conn: Connection<'_, Db>) -> Result<Json<Vec<posting::Model>>, Status> {
	let db = conn.into_inner();

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

/// Return all postings that have been bookmarked, order by creation time descending.
#[get("/postings/bookmarked")]
pub async fn bookmarked_postings(conn: Connection<'_, Db>) -> Result<Json<Vec<posting::Model>>, Status> {
	let db = conn.into_inner();

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

/// Retrieve new postings from a specific source.
///
/// Return unread postings.
#[get("/postings/refresh?<source_id>")]
pub async fn refresh_postings(conn: Connection<'_, Db>, source_id: Option<i32>) -> Result<Json<Vec<posting::Model>>, Status> {
	let db = conn.into_inner();

	let source = Source::find()
		.filter(source::Column::Deleted.eq(false))
		.filter(source::Column::Id.eq(source_id))
		.all(db)
		.await
		.expect("Could not retrieve source information");
	if source.is_empty() {
		return Ok(Json(vec![]));
	}

	let filters = Filter::find().all(db).await.expect("Could not get filters");
	let settings = Settings::find().one(db).await.expect("Could not get settings").expect("No settings stored");

	// start extraction process
	let mut extractor = PostingsExtractor::new(
		source.first().unwrap().url.clone(),
		source.first().unwrap().id,
		settings,
		source.first().unwrap().selector.clone(),
		source.first().unwrap().pagination.clone(),
		filters,
		source.first().unwrap().content.clone(),
	);
	extractor.extract(db).await.expect("Could not refresh postings");

	// save extracted postings to database
	extractor.save(db).await.expect("Could not cache source content");

	Ok(Json(
		Posting::find()
			.filter(posting::Column::SourceId.eq(source.first().unwrap().id))
			.order_by_desc(posting::Column::CreatedAt)
			.all(db)
			.await
			.expect("Could not retrieve postings"),
	))
}

/// Return a specific posting.
#[get("/postings/<id>")]
pub async fn posting_by_id(conn: Connection<'_, Db>, id: i32) -> Result<Json<Option<posting::Model>>, Status> {
	let db = conn.into_inner();

	Ok(Json(Posting::find().filter(posting::Column::Id.eq(id)).one(db).await.expect("Could not retrieve posting")))
}

/// Return postings.
/// If `source_id` has been provided then only return postings associated with that source.
/// If `read` has been provided then only return postings based on their `seen` state.
///
/// Return list of postings, ordered by creation timestamp descending.
#[get("/postings?<source_id>&<read>")]
pub async fn get_postings(conn: Connection<'_, Db>, source_id: Option<i32>, read: Option<bool>) -> Result<Json<Vec<posting::Model>>, Status> {
	let db = conn.into_inner();

	let mut filter_condition = Condition::all();

	if let Some(source_id) = source_id {
		filter_condition = filter_condition.add(posting::Column::SourceId.eq(source_id));
	}

	if let Some(read) = read {
		filter_condition = filter_condition.add(posting::Column::Seen.eq(read));
	}

	Ok(Json(
		Posting::find()
			.filter(filter_condition)
			.order_by_desc(posting::Column::CreatedAt)
			.all(db)
			.await
			.expect("Could not retrieve postings"),
	))
}

/// Update the `seen` state of a set of postings.
/// The request body is expected to contain a list of posting IDs to update.
#[put("/postings/mark_read", data = "<input>")]
pub async fn mark_postings_read(conn: Connection<'_, Db>, input: Json<Vec<i32>>) -> Result<(), Status> {
	let db = conn.into_inner();

	let _ = Posting::update_many()
		.col_expr(posting::Column::Seen, Expr::value(true))
		.filter(posting::Column::Id.is_in(input.into_inner()))
		.exec(db)
		.await
		.expect("Could not mark posting as read");

	Ok(())
}

/// Update a specific posting.
/// Onlye `seen`, `bookmarked` and `is_match` properties can be updated, any other property update is ignored.
///
/// Return updated posting.
#[put("/postings/<id>", data = "<input>")]
pub async fn update_posting(conn: Connection<'_, Db>, id: i32, input: Json<posting::Model>) -> Result<Json<posting::Model>, Status> {
	let db = conn.into_inner();

	let existing_posting = Posting::find_by_id(id).one(db).await.expect("Could not find posting");
	let mut existing_posting: posting::ActiveModel = existing_posting.unwrap().into();
	let updated_posting: posting::Model = input.into_inner();

	existing_posting.seen = Set(updated_posting.seen);
	existing_posting.bookmarked = Set(updated_posting.bookmarked);
	existing_posting.is_match = Set(updated_posting.is_match);

	let existing_posting: posting::Model = existing_posting.update(db).await.expect("Could not update posting");

	Ok(Json(existing_posting))
}
