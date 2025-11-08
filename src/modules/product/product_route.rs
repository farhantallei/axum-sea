use axum::{
    Router,
    routing::{get, post},
};

use super::product_controller::{create_product_handler, find_all_products_handler};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(find_all_products_handler))
        .route("/", post(create_product_handler))
}
