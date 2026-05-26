use crate::application::error::AppError;
use crate::domain::user::User;
use async_trait::async_trait;
use colored::Colorize;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ClerkClaims {
    sub: String,
    email: Option<String>,
    exp: usize,
    azp: Option<String>,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AuthService: Send + Sync {
    async fn validate_token(&self, token: &str) -> Result<User, AppError>;
}

pub struct ClerkAuthService {
    jwks: JwkSet,
    frontend_api_url: String,
}

impl ClerkAuthService {
    /// Creates a new ClerkAuthService instance
    ///
    /// # Arguments
    /// * `frontend_api_url` - The Clerk Frontend API URL (e.g. `https://musical-pup-67.clerk.accounts.dev`)
    /// * `jwks_url` - The URL to fetch the JWKS from. If None, defaults to
    ///   `{frontend_api_url}/.well-known/jwks.json`
    pub async fn new(frontend_api_url: String, jwks_url: Option<&str>) -> Result<Self, AppError> {
        let url = if let Some(url) = jwks_url {
            url.to_string()
        } else {
            format!(
                "{}/.well-known/jwks.json",
                frontend_api_url.trim_end_matches('/')
            )
        };

        println!("{} Fetching JWKS from URL: {}", "ℹ".yellow().bold(), url);

        let jwks: JwkSet = reqwest::get(&url)
            .await
            .map_err(|e| AppError::CallError(format!("Failed to fetch Clerk JWKS: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::CallError(format!("Failed to parse Clerk JWKS: {}", e)))?;

        Ok(Self {
            jwks,
            frontend_api_url,
        })
    }

    /// Finds a JWK (JSON Web Key) by its key ID in the JWKS set
    fn find_jwk(&self, kid: &str) -> Option<&jsonwebtoken::jwk::Jwk> {
        self.jwks
            .keys
            .iter()
            .find(|k| k.common.key_id.as_deref() == Some(kid))
    }
}

#[async_trait]
impl AuthService for ClerkAuthService {
    async fn validate_token(&self, token: &str) -> Result<User, AppError> {
        let header = decode_header(token)
            .map_err(|e| AppError::AuthenticationError(format!("Invalid token header: {}", e)))?;

        let kid = header
            .kid
            .ok_or_else(|| AppError::AuthenticationError("Token missing kid".to_string()))?;

        let jwk = self
            .find_jwk(&kid)
            .ok_or_else(|| AppError::AuthenticationError("Unknown key ID".to_string()))?;

        let decoding_key = DecodingKey::from_jwk(jwk)
            .map_err(|e| AppError::AuthenticationError(format!("Invalid JWK: {}", e)))?;

        // Use the algorithm specified in the token header (RS256 or ES256 depending on Clerk config)
        let mut validation = Validation::new(header.alg);
        validation.set_issuer(&[&self.frontend_api_url]);
        // Clerk uses `azp` rather than `aud` for session tokens — disable audience validation
        validation.validate_aud = false;

        let token_data = decode::<ClerkClaims>(token, &decoding_key, &validation).map_err(|e| {
            AppError::AuthenticationError(format!("Token validation failed: {}", e))
        })?;

        Ok(User::new(
            token_data.claims.sub.clone(),
            token_data.claims.email.unwrap_or(token_data.claims.sub),
            None, // Clerk JWT does not include name
        ))
    }
}

#[cfg(test)]
#[path = "auth_service_tests.rs"]
mod tests;
