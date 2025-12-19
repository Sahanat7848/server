use crate::domain::repositories::brawlers::BrawlerRepository;
use crate::infrastructure::jwt::{
    self,
    authentication_model::LoginModel,
    jwt_model::{Claims, Passport},
};
use anyhow::{Result, anyhow};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordVerifier},
};
use chrono::{Duration, Utc};
use std::sync::Arc;

pub struct AuthenticationUseCase<T> {
    repository: Arc<T>,
}

impl<T> AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }

    pub async fn login(&self, model: LoginModel) -> Result<Passport> {
        let brawler = self
            .repository
            .find_by_username(model.username.clone())
            .await?;

        let parsed_hash = PasswordHash::new(&brawler.password)
            .map_err(|e| anyhow!("Invalid password hash: {}", e))?;

        Argon2::default()
            .verify_password(model.password.as_bytes(), &parsed_hash)
            .map_err(|_| anyhow!("Invalid password"))?;

        let now = Utc::now();
        let exp = (now + Duration::days(1)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claims = Claims {
            sub: brawler.id.to_string(),
            exp,
            iat,
        };

        let secret = crate::config::config_loader::get_user_secret()
            .map_err(|e| anyhow!("Failed to get secret: {}", e))?;
        let token = jwt::generate_token(secret, &claims)?;

        Ok(Passport { token })
    }
}
