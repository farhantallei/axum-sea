use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::{PasswordHash, SaltString, rand_core::OsRng};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Password hashing failed")
        .to_string()
}

pub fn verify_password(password: &str, hash_phc: &str) -> bool {
    let parsed = match PasswordHash::new(hash_phc) {
        Ok(phc) => phc,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "secure_password";
        let hash = hash_password(password);

        assert!(verify_password(password, &hash));
        assert!(!verify_password("wrong_password", &hash));
    }

    #[test]
    fn test_invalid_hash() {
        let password = "secure_password";
        let invalid_hash = "invalid_hash_string";
        assert!(!verify_password(password, invalid_hash));
    }
}
