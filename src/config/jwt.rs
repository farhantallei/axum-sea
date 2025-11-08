use anyhow::Result;
use std::env;

use crate::utils::auth::JwtConfig;

pub fn load_jwt_config() -> Result<JwtConfig> {
    let jwt_secret =
        env::var("JWT_SECRET").expect("JWT_SECRET must be set in environment variables");
    let jwt_config = JwtConfig {
        secret: jwt_secret,
        issuer: "my_app".to_string(),
        access_token_ttl_minutes: 60,
    };

    Ok(jwt_config)
}
