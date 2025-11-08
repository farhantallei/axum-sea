use sea_orm::DatabaseConnection;

use crate::utils::auth::JwtConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub jwt_config: JwtConfig,
}
