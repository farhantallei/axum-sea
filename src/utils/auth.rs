use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub aud: Option<String>,
}

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub access_token_ttl_minutes: i64,
}

impl JwtConfig {
    pub fn encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(self.secret.as_bytes())
    }

    pub fn decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.secret.as_bytes())
    }

    pub fn validation(&self) -> Validation {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.leeway = 0;
        validation.set_issuer(&[self.issuer.clone()]);
        validation
    }
}

pub fn create_token(cfg: &JwtConfig, subject: i32) -> anyhow::Result<String> {
    let now = Utc::now();
    let exp = now + Duration::minutes(cfg.access_token_ttl_minutes);
    let claims = Claims {
        sub: subject,
        exp: exp.timestamp(),
        iat: now.timestamp(),
        iss: cfg.issuer.clone(),
        aud: None,
    };

    let token = encode(&Header::new(Algorithm::HS256), &claims, &cfg.encoding_key())?;
    Ok(token)
}

pub fn verify_token(cfg: &JwtConfig, token: &str) -> anyhow::Result<Claims> {
    let token_data = decode::<Claims>(token, &cfg.decoding_key(), &cfg.validation())?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> JwtConfig {
        JwtConfig {
            secret: "super-secret-for-tests-change-in-prod".to_string(),
            issuer: "test-issuer".to_string(),
            access_token_ttl_minutes: 1,
        }
    }

    #[test]
    fn test_create_and_verify_token() {
        let c = cfg();
        let token = create_token(&c, 123).unwrap();
        let claims = verify_token(&c, &token).unwrap();
        assert_eq!(claims.sub, 123);
        assert_eq!(claims.iss, "test-issuer");
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_verify_token_fails_with_wrong_secret() {
        let c1 = cfg();
        let token = create_token(&c1, 123).unwrap();

        let c2 = JwtConfig {
            secret: "different-secret".to_string(),
            issuer: "test-issuer".to_string(),
            access_token_ttl_minutes: 1,
        };

        assert!(verify_token(&c2, &token).is_err());
    }

    #[test]
    fn test_verify_token_fails_when_expired() {
        let c = cfg();
        let now = Utc::now();
        let claims = Claims {
            sub: 123,
            iat: now.timestamp(),
            exp: (now - Duration::seconds(1)).timestamp(),
            iss: c.issuer.clone(),
            aud: None,
        };

        let token = encode(&Header::new(Algorithm::HS256), &claims, &c.encoding_key()).unwrap();

        let res = verify_token(&c, &token);
        assert!(res.is_err());
    }
}
