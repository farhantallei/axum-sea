use anyhow::Result;
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::modules::user::user_entity;

use super::product_entity;

#[derive(Clone)]
pub struct ProductService;

impl ProductService {
    pub async fn create_product(
        db: &DatabaseConnection,
        owner_id: i32,
        title: String,
        content: Option<String>,
        price: Decimal,
    ) -> Result<product_entity::Model> {
        let product = product_entity::ActiveModel {
            owner_id: sea_orm::ActiveValue::Set(owner_id),
            title: sea_orm::ActiveValue::Set(title),
            content: sea_orm::ActiveValue::Set(content),
            price: sea_orm::ActiveValue::Set(price),
            ..Default::default()
        };

        let inserted = product.insert(db).await?;
        Ok(inserted)
    }

    pub async fn update_product(
        db: &DatabaseConnection,
        product_id: i32,
        title: Option<String>,
        content: Option<String>,
        price: Option<Decimal>,
    ) -> Result<Option<product_entity::Model>> {
        if let Some(product) = product_entity::Entity::find_by_id(product_id)
            .one(db)
            .await?
        {
            let mut active_model: product_entity::ActiveModel = product.into();

            if let Some(t) = title {
                active_model.title = Set(t);
            }
            if let Some(c) = content {
                active_model.content = Set(Some(c));
            }
            if let Some(p) = price {
                active_model.price = Set(p);
            }

            let updated_product = active_model.update(db).await?;
            Ok(Some(updated_product))
        } else {
            Ok(None)
        }
    }

    pub async fn find_all_products(db: &DatabaseConnection) -> Result<Vec<product_entity::Model>> {
        let products = product_entity::Entity::find().all(db).await?;
        Ok(products)
    }

    pub async fn find_all_products_with_owner(
        db: &DatabaseConnection,
    ) -> Result<Vec<(product_entity::Model, Option<user_entity::Model>)>> {
        let products_with_owners = product_entity::Entity::find()
            .find_also_related(user_entity::Entity)
            .all(db)
            .await?;
        Ok(products_with_owners)
    }

    pub async fn find_product_by_id(
        db: &DatabaseConnection,
        product_id: i32,
    ) -> Result<Option<product_entity::Model>> {
        let product = product_entity::Entity::find_by_id(product_id)
            .one(db)
            .await?;
        Ok(product)
    }

    pub async fn delete_product(db: &DatabaseConnection, product_id: i32) -> Result<bool> {
        let result = product_entity::Entity::delete_by_id(product_id)
            .exec(db)
            .await?;
        Ok(result.rows_affected > 0)
    }
}
