use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{self, SaltString, rand_core::OsRng},
};

use crate::model::User;

pub fn verify_password(user: &User, password: String) -> password_hash::Result<()> {
    let argon = Argon2::default();
    let hash = PasswordHash::new(&user.password_hash)?;
    argon.verify_password(password.as_bytes(), &hash)
}

pub fn hash_password(password: String) -> password_hash::Result<String> {
    let argon = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    argon
        .hash_password(password.as_bytes(), &salt)
        .map(|p| p.to_string())
}
