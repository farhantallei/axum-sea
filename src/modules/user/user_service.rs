use anyhow::Result;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use super::user_entity;
use crate::utils::hash::hash_password;

#[derive(Clone)]
pub struct UserService;

impl UserService {
    pub async fn create_user(
        db: &DatabaseConnection,
        email: String,
        name: Option<String>,
        password: String,
    ) -> Result<user_entity::Model> {
        let password_hash = hash_password(&password);

        let user = user_entity::ActiveModel {
            name: Set(name),
            email: Set(email),
            password: Set(password_hash),
            ..Default::default()
        };

        let inserted = user.insert(db).await?;
        Ok(inserted)
    }

    pub async fn find_all_users(db: &DatabaseConnection) -> Result<Vec<user_entity::Model>> {
        let users = user_entity::Entity::find().all(db).await?;
        Ok(users)
    }

    pub async fn find_user_by_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Option<user_entity::Model>> {
        let user = user_entity::Entity::find_by_id(user_id).one(db).await?;
        Ok(user)
    }

    pub async fn find_user_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user_entity::Model>> {
        let user = user_entity::Entity::find()
            .filter(user_entity::Column::Email.eq(email))
            .one(db)
            .await?;
        Ok(user)
    }
}
