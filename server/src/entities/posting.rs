//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "posting")]
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id: i32,
	pub title: String,
	pub description: Option<String>,
	pub url: Option<String>,
	pub created_at: Option<DateTimeWithTimeZone>,
	pub seen: Option<bool>,
	pub source_id: Option<i32>,
	pub bookmarked: Option<bool>,
	#[sea_orm(column_type = "Text", nullable)]
	pub content: Option<String>,
	pub is_match: Option<bool>,
	#[sea_orm(column_type = "Float", nullable)]
	pub match_similarity: Option<f32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::embedding::Entity")]
	Embedding,
	#[sea_orm(
		belongs_to = "super::source::Entity",
		from = "Column::SourceId",
		to = "super::source::Column::Id",
		on_update = "NoAction",
		on_delete = "NoAction"
	)]
	Source,
}

impl Related<super::embedding::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Embedding.def()
	}
}

impl Related<super::source::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Source.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
