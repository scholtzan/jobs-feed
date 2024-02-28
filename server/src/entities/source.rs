//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "source")]
pub struct Model {
	#[sea_orm(primary_key)]
	#[serde(skip_deserializing)]
	pub id: i32,
	pub name: String,
	pub url: String,
	pub created_at: Option<DateTimeWithTimeZone>,
	pub selector: Option<String>,
	pub pagination: Option<String>,
	#[sea_orm(column_type = "Text", nullable)]
	pub content: Option<String>,
	pub favicon: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::extraction::Entity")]
	Extraction,
	#[sea_orm(has_many = "super::posting::Entity")]
	Posting,
}

impl Related<super::extraction::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Extraction.def()
	}
}

impl Related<super::posting::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Posting.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
