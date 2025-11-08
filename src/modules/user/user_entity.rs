use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::modules::product::product_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub email: String,
    pub name: Option<String>,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "product_entity::Entity")]
    Products,
}

impl Related<product_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Products.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
