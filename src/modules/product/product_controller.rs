use axum::{Json, extract::State, http::StatusCode};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use validator::{Validate, ValidationError, ValidationErrors};

use super::product_service::ProductService;
use crate::{
    middleware::AuthClaims,
    modules::{
        product::product_dto::{BaseProductResponse, CreateProductPayload, GetProductsResponse},
        shared::error::AppError,
        user::user_dto::GetUsersResponse,
    },
    state::AppState,
};

pub async fn find_all_products_handler(
    State(state): State<AppState>,
    AuthClaims(_claims): AuthClaims,
) -> Result<(StatusCode, Json<Vec<GetProductsResponse>>), AppError> {
    let products = ProductService::find_all_products_with_owner(&state.db)
        .await
        .map_err(|e| AppError::internal(e))?;

    let response: Vec<GetProductsResponse> = products
        .into_iter()
        .map(|(product, owner)| GetProductsResponse {
            product: BaseProductResponse {
                id: product.id,
                owner_id: product.owner_id,
                title: product.title,
                content: product.content,
                price: product.price,
                created_at: product.created_at.to_string(),
                updated_at: product.updated_at.to_string(),
            },
            owner: owner.map(|user| GetUsersResponse {
                id: user.id,
                email: user.email,
                name: user.name,
            }),
        })
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

pub async fn create_product_handler(
    State(state): State<AppState>,
    AuthClaims(claims): AuthClaims,
    Json(payload): Json<CreateProductPayload>,
) -> Result<(StatusCode, Json<BaseProductResponse>), AppError> {
    payload.validate().map_err(AppError::validation)?;

    let mut validate_price = ValidationErrors::new();

    validate_price.add(
        "price",
        ValidationError {
            code: "range".into(),
            message: Some("Price must be a non-negative number".into()),
            params: std::collections::HashMap::new(),
        },
    );

    let price_decimal =
        Decimal::from_f64(payload.price).ok_or_else(|| AppError::validation(validate_price))?;

    let new_product = ProductService::create_product(
        &state.db,
        claims.sub,
        payload.title,
        payload.content,
        price_decimal,
    )
    .await
    .map_err(|e| AppError::internal(e))?;

    let response = BaseProductResponse {
        id: new_product.id,
        owner_id: new_product.owner_id,
        title: new_product.title,
        content: new_product.content,
        price: new_product.price,
        created_at: new_product.created_at.to_string(),
        updated_at: new_product.updated_at.to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}
