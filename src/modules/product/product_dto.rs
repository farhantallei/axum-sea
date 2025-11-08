use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::user::user_dto::GetUsersResponse;

#[derive(Debug, Serialize)]
pub struct BaseProductResponse {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub content: Option<String>,
    pub price: Decimal,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct GetProductsResponse {
    #[serde(flatten)]
    pub product: BaseProductResponse,
    pub owner: Option<GetUsersResponse>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateProductPayload {
    #[validate(length(min = 1, max = 100))]
    pub title: String,

    pub content: Option<String>,

    #[validate(range(min = 0.0))]
    pub price: f64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateProductPayload {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,

    pub content: Option<String>,

    #[validate(range(min = 0.0))]
    pub price: Option<f64>,
}
