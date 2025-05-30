//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "collection")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_time: DateTime,
    pub is_public: i32,
    pub author: String,
    pub listing: Option<i32>,
    pub icon_url: Option<String>,
    pub package_id: Option<String>,
    pub seq: i32,
    pub status: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
