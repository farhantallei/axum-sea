use axum::{
    Router,
    routing::{get, post},
};

use super::user_controller::register_user_handler;
use super::user_controller::{find_all_users_handler, login_user_handler, me_handler};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(find_all_users_handler))
        .route("/me", get(me_handler))
        .route("/", post(register_user_handler))
        .route("/login", post(login_user_handler))
}
