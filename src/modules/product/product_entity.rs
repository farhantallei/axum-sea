use rust_decimal::Decimal;
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

use crate::modules::user::user_entity;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub content: Option<String>,
    pub price: Decimal,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user_entity::Entity",
        from = "Column::OwnerId",
        to = "user_entity::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    User,
}

impl Related<user_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let current_time = chrono::Utc::now().naive_utc();
        self.updated_at = Set(current_time);
        Ok(self)
    }
}
