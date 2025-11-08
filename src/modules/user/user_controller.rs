use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

use super::user_dto::{
    CreateUserPayload, CreateUserResponse, GetUsersResponse, LoginUserPayload, LoginUserResponse,
};
use super::user_service::UserService;
use crate::{
    middleware::AuthClaims,
    modules::shared::error::AppError,
    state::AppState,
    utils::{auth::create_token, hash},
};

pub async fn me_handler(
    State(state): State<AppState>,
    AuthClaims(claims): AuthClaims,
) -> Result<(StatusCode, Json<CreateUserResponse>), AppError> {
    let user_option = UserService::find_user_by_id(&state.db, claims.sub)
        .await
        .map_err(|e| AppError::internal(e))?;

    if let None = user_option {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    let user = user_option.unwrap();

    Ok((
        StatusCode::OK,
        Json(CreateUserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
        }),
    ))
}

pub async fn find_all_users_handler(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<GetUsersResponse>>), AppError> {
    let users = UserService::find_all_users(&state.db)
        .await
        .map_err(|e| AppError::internal(e))?;

    let response: Vec<GetUsersResponse> = users
        .into_iter()
        .map(|user| GetUsersResponse {
            id: user.id,
            email: user.email,
            name: user.name,
        })
        .collect();

    Ok((StatusCode::OK, Json(response)))
}

pub async fn register_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> Result<(StatusCode, Json<CreateUserResponse>), AppError> {
    payload.validate().map_err(AppError::validation)?;

    let user = UserService::create_user(&state.db, payload.email, payload.name, payload.password)
        .await
        .map_err(|e| AppError::internal(e))?;

    Ok((
        StatusCode::CREATED,
        Json(CreateUserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
        }),
    ))
}

pub async fn login_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginUserPayload>,
) -> Result<(StatusCode, Json<LoginUserResponse>), AppError> {
    payload.validate().map_err(AppError::validation)?;

    let user_option = UserService::find_user_by_email(&state.db, &payload.email)
        .await
        .map_err(|e| AppError::internal(e))?;

    if let None = user_option {
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    let user = user_option.unwrap();

    let is_password_valid = hash::verify_password(&payload.password, &user.password);

    if !is_password_valid {
        return Err(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    let token = create_token(&state.jwt_config, user.id).map_err(AppError::internal)?;

    Ok((StatusCode::OK, Json(LoginUserResponse { token })))
}
