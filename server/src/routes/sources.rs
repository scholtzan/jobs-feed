use crate::entities::{prelude::*, *};
use chrono::FixedOffset;

use crate::openai::assistant::{Assistant, AssistantType};
use crate::pool::Db;
use rocket::http::Status;
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use sea_orm::*;

/// Return active sources.
#[get("/sources")]
pub async fn sources(conn: Connection<'_, Db>) -> Result<Json<Vec<source::Model>>, Status> {
	let db = conn.into_inner();

	Ok(Json(
		Source::find()
			.filter(source::Column::Deleted.eq(false))
			.all(db)
			.await
			.expect("Could not retrieve sources")
			.into_iter()
			.collect::<Vec<_>>(),
	))
}

/// Add a new source.
/// The request body is expected to have the source information.
///
/// Return newly created source.
#[post("/sources", data = "<input>")]
pub async fn add_source(conn: Connection<'_, Db>, input: Json<source::Model>) -> Result<Json<source::Model>, Status> {
	let db = conn.into_inner();

	let mut new_source: source::ActiveModel = input.into_inner().into();
	new_source.id = NotSet;
	new_source.created_at = Set(Some(chrono::offset::Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())));
	let inserted_source: source::Model = new_source.insert(db).await.expect("Could not insert source");

	// get similar sources
	let _ = _refresh_source_suggestions(&db, inserted_source.id).await?;

	Ok(Json(inserted_source))
}

/// Remove a specific source.
#[delete("/sources/<id>")]
pub async fn delete_source(conn: Connection<'_, Db>, id: i32) -> Result<(), Status> {
	let db = conn.into_inner();

	let source: Option<source::Model> = Source::find_by_id(id).one(db).await.expect("Could not find source");
	let mut source: source::ActiveModel = source.unwrap().into();
	// don't actually delete the source from the database, but instead set `deleted` flag
	// this prevents related postings from being removed
	source.deleted = Set(true);

	source.update(db).await.expect("Could not delete source");

	Ok(())
}

/// Return a specific source.
#[get("/sources/<id>")]
pub async fn source_by_id(conn: Connection<'_, Db>, id: i32) -> Result<Json<Option<source::Model>>, Status> {
	let db = conn.into_inner();

	Ok(Json(Source::find().filter(source::Column::Id.eq(id)).one(db).await.expect("Could not retrieve source")))
}

/// Return source suggestions that are similar to the source with the provided ID.
#[get("/sources/<id>/suggestions")]
pub async fn source_suggestions(conn: Connection<'_, Db>, id: i32) -> Result<Json<Vec<suggestion::Model>>, Status> {
	let db = conn.into_inner();

	Ok(Json(
		Suggestion::find()
			.filter(suggestion::Column::SourceId.eq(id))
			.limit(10)
			.all(db)
			.await
			.expect("Could not retrieve suggestions"),
	))
}

/// Get sources that are similar to the source with the provided `id`.
///
/// Returns the retrieved source suggestions.
async fn _refresh_source_suggestions(db: &DatabaseConnection, id: i32) -> Result<Json<Vec<suggestion::Model>>, Status> {
	// get the source similar suggestions should be determined
	let source = Source::find().filter(source::Column::Id.eq(id)).one(db).await.expect("Could not retrieve source");

	// get a set of existing sources
	// to prevent duplicate suggestions, these sources will be passed to the LLM to be ignored
	let sources = Source::find().filter(source::Column::Deleted.eq(false)).limit(50).all(db).await.expect("Could not retrieve sources");

	// get existing suggestions for source
	let suggestions = Suggestion::find()
		.filter(suggestion::Column::SourceId.eq(id))
		.limit(20)
		.all(db)
		.await
		.expect("Could not retrieve existing suggestions");
	let mut existing_suggestions: Vec<String> = suggestions.iter().map(|s| s.name.clone()).collect();
	let existing_sources: Vec<String> = sources.iter().map(|s| s.name.clone()).collect();

	// existing suggestions that should be ignored by the LLM when searching for new suggestions
	existing_suggestions.extend(existing_sources);

	// create a new assistant instance
	let settings = Settings::find().one(db).await.expect("Could not retrieve settings").unwrap();
	let api_key = settings.api_key.unwrap().clone();
	let model = settings.model.unwrap().clone();
	let mut assistant = Assistant::new(&api_key, &model, AssistantType::JobsSuggestion).await.expect("Could not retrieve assistant");

	// create the prompt to get suggestions, and ignore existing ones
	let source_name = source.unwrap().name;
	let ignore = existing_suggestions.join(", ");
	let message = format!(
		"Company: {source_name}; \
    Ignore career pages of the following companies: {ignore}"
	);
	let response = assistant.run(&vec![message]).await.expect("Could not get source suggestions");

	// store the retrieved suggestions
	let parsed_response: Vec<suggestion::Model> = serde_json::from_str(response.first().expect("Did not get suggestions")).expect("Could not extract suggestions");
	let active_suggestions: Vec<suggestion::ActiveModel> = parsed_response
		.into_iter()
		.map(|res| {
			let mut active_suggestion: suggestion::ActiveModel = res.into();
			active_suggestion.id = NotSet;
			active_suggestion.source_id = Set(Some(id));
			active_suggestion
		})
		.collect();

	let _ = Suggestion::insert_many(active_suggestions).exec(db).await;

	Ok(Json(
		Suggestion::find()
			.filter(suggestion::Column::SourceId.eq(id))
			.limit(10)
			.all(db)
			.await
			.expect("Could not retrieve suggestions"),
	))
}

/// Get new suggestions for a specific source.
///
/// Returns the similar source suggestions.
#[put("/sources/<id>/suggestions/refresh")]
pub async fn refresh_source_suggestions(conn: Connection<'_, Db>, id: i32) -> Result<Json<Vec<suggestion::Model>>, Status> {
	let db = conn.into_inner();

	_refresh_source_suggestions(&db, id).await
}

/// Update an existing source.
///
/// Returns the updated source information.
#[put("/sources/<id>", data = "<input>")]
pub async fn update_source(conn: Connection<'_, Db>, id: i32, input: Json<source::Model>) -> Result<Json<source::Model>, Status> {
	let db = conn.into_inner();

	let existing_source = Source::find_by_id(id).one(db).await.expect("Could not find source").unwrap();
	let updated_source: source::Model = input.into_inner();
	let content_changed = existing_source.url != updated_source.url || existing_source.selector != updated_source.selector || existing_source.pagination != updated_source.pagination;

	let mut existing_source_active: source::ActiveModel = existing_source.into();

	if content_changed {
		existing_source_active.content = Set(Some("".to_string()));
	}

	existing_source_active.name = Set(updated_source.name);
	existing_source_active.url = Set(updated_source.url);
	existing_source_active.selector = Set(updated_source.selector);
	existing_source_active.pagination = Set(updated_source.pagination);
	existing_source_active.favicon = Set(updated_source.favicon);

	let existing_source: source::Model = existing_source_active.update(db).await.expect("Could not update source");

	Ok(Json(existing_source))
}

/// Clear the source's content cache.
/// The next time postings are refreshed, the source page will be parsed entirely instead of just the source page changes.
#[put("/sources/<id>/reset")]
pub async fn reset_source_cache(conn: Connection<'_, Db>, id: i32) -> Result<(), Status> {
	let db = conn.into_inner();

	let existing_source = Source::find_by_id(id).one(db).await.expect("Could not find source").unwrap();
	let mut existing_source_active: source::ActiveModel = existing_source.into();
	existing_source_active.content = Set(Some("".to_string()));

	let _ = existing_source_active.update(db).await.expect("Could not reset source");

	Ok(())
}
