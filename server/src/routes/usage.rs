use crate::entities::*;
use serde::{Deserialize, Serialize};

use chrono;
use chrono::Duration;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use sea_orm::*;

#[derive(Clone, Debug, Serialize, Deserialize, FromQueryResult)]
pub struct Usage {
	source_name: String,
	source_id: i32,
	cost: f32,
}

#[get("/usage/cost?<days>")]
pub async fn extraction_costs(db: &State<DatabaseConnection>, days: Option<i64>) -> Result<Json<Vec<Usage>>, Status> {
	let db = db as &DatabaseConnection;

	let mut filter_condition = Condition::all();
	if let Some(days) = days {
		filter_condition = filter_condition.add(extraction::Column::CreatedAt.gte(chrono::offset::Utc::now() - Duration::days(days)));
	}

	Ok(Json(
		extraction::Entity::find()
			.select_only()
			.column_as(source::Column::Name, "source_name")
			.column(extraction::Column::SourceId)
			.column_as(extraction::Column::Cost.sum(), "cost")
			.join(JoinType::InnerJoin, extraction::Relation::Source.def())
			.group_by(extraction::Column::SourceId)
			.group_by(source::Column::Name)
			.filter(filter_condition)
			.order_by_asc(source::Column::Name)
			.into_model::<Usage>()
			.all(db)
			.await
			.expect("Cannot retrieve extraction cost"),
	))
}
