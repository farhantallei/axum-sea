use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct GetUsersResponse {
    pub id: i32,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserPayload {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,

    #[validate(length(min = 6, max = 100))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub id: i32,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUserPayload {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginUserResponse {
    pub token: String,
}
