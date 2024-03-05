use crate::entities::{prelude::*, *};
use chrono::FixedOffset;

use crate::assistant::{Assistant, AssistantType};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use sea_orm::*;

#[get("/sources")]
pub async fn sources(db: &State<DatabaseConnection>) -> Result<Json<Vec<source::Model>>, Status> {
	let db = db as &DatabaseConnection;

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

#[post("/sources", data = "<input>")]
pub async fn add_source(db: &State<DatabaseConnection>, input: Json<source::Model>) -> Result<Json<source::Model>, Status> {
	let db_connection = db as &DatabaseConnection;

	let mut new_source: source::ActiveModel = input.into_inner().into();
	new_source.id = NotSet;
	new_source.created_at = Set(Some(chrono::offset::Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap())));
	let inserted_source: source::Model = new_source.insert(db_connection).await.expect("Could not insert source");

	let _ = refresh_source_suggestions(db, inserted_source.id);

	Ok(Json(inserted_source))
}

#[delete("/sources/<id>")]
pub async fn delete_source(db: &State<DatabaseConnection>, id: i32) -> Result<(), Status> {
	let db = db as &DatabaseConnection;

	let source: Option<source::Model> = Source::find_by_id(id).one(db).await.expect("Could not find source");
	let mut source: source::ActiveModel = source.unwrap().into();
	source.deleted = Set(true);

	source.update(db).await.expect("Could not delete source");

	Ok(())
}

#[get("/sources/<id>")]
pub async fn source_by_id(db: &State<DatabaseConnection>, id: i32) -> Result<Json<Option<source::Model>>, Status> {
	let db = db as &DatabaseConnection;

	Ok(Json(Source::find().filter(source::Column::Id.eq(id)).one(db).await.expect("Could not retrieve source")))
}

#[get("/sources/<id>/suggestions")]
pub async fn source_suggestions(db: &State<DatabaseConnection>, id: i32) -> Result<Json<Vec<suggestion::Model>>, Status> {
	let db = db as &DatabaseConnection;

	Ok(Json(
		Suggestion::find()
			.filter(suggestion::Column::SourceId.eq(id))
			.limit(10)
			.all(db)
			.await
			.expect("Could not retrieve suggestions"),
	))
}

#[put("/sources/<id>/suggestions/refresh")]
pub async fn refresh_source_suggestions(db: &State<DatabaseConnection>, id: i32) -> Result<Json<Vec<suggestion::Model>>, Status> {
	let db = db as &DatabaseConnection;

	let source = Source::find().filter(source::Column::Id.eq(id)).one(db).await.expect("Could not retrieve source");
	let sources = Source::find().filter(source::Column::Deleted.eq(false)).limit(20).all(db).await.expect("Could not retrieve sources");
	let suggestions = Suggestion::find()
		.filter(suggestion::Column::SourceId.eq(id))
		.limit(15)
		.all(db)
		.await
		.expect("Could not retrieve existing suggestions");
	let mut existing_suggestions: Vec<String> = suggestions.iter().map(|s| s.name.clone()).collect();
	let existing_sources: Vec<String> = sources.iter().map(|s| s.name.clone()).collect();
	existing_suggestions.extend(existing_sources);

	let settings = Settings::find().one(db).await.expect("Could not retrieve settings").unwrap();
	let api_key = settings.api_key.unwrap().clone();
	let model = settings.model.unwrap().clone();
	let mut assistant = Assistant::new(&api_key, &model, AssistantType::JobsSuggestion).await.expect("Could not retrieve assistant");

	let source_name = source.unwrap().name;
	let ignore = existing_suggestions.join(", ");
	let message = format!(
		"Company: {source_name}; \
    Ignore career pages of the following companies: {ignore}"
	);
	let response = assistant.run(&vec![message]).await.expect("Could not get source suggestions");

	let parsed_response: Vec<suggestion::Model> = serde_json::from_str(&response).expect("Could not extract suggestions");
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

	let suggestion_usage: extraction::ActiveModel = extraction::ActiveModel {
		id: NotSet,
		created_at: Set(Some(chrono::offset::Utc::now().with_timezone(&FixedOffset::east_opt(0).unwrap()))),
		model: Set(Some(assistant.model.clone())),
		prompt_tokens: Set(Some(assistant.usage.prompt_tokens)),
		completion_tokens: Set(Some(assistant.usage.completion_tokens)),
		source_id: Set(Some(id)),
		cost: Set(assistant.usage.get_cost(&assistant.model)),
	};
	suggestion_usage.insert(db).await.expect("Could not update suggestion usage");

	Ok(Json(
		Suggestion::find()
			.filter(suggestion::Column::SourceId.eq(id))
			.limit(10)
			.all(db)
			.await
			.expect("Could not retrieve suggestions"),
	))
}

#[put("/sources/<id>", data = "<input>")]
pub async fn update_source(db: &State<DatabaseConnection>, id: i32, input: Json<source::Model>) -> Result<Json<source::Model>, Status> {
	let db = db as &DatabaseConnection;

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

#[put("/sources/<id>/reset")]
pub async fn reset_source_cache(db: &State<DatabaseConnection>, id: i32) -> Result<(), Status> {
	let db = db as &DatabaseConnection;

	let existing_source = Source::find_by_id(id).one(db).await.expect("Could not find source").unwrap();
	let mut existing_source_active: source::ActiveModel = existing_source.into();
	existing_source_active.content = Set(Some("".to_string()));

	let _ = existing_source_active.update(db).await.expect("Could not reset source");

	Ok(())
}
