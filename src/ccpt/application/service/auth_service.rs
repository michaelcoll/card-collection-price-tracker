use crate::application::error::AppError;
use crate::domain::user::User;
use async_trait::async_trait;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GoogleClaims {
    sub: String,
    email: String,
    name: Option<String>,
    iss: String,
    aud: String,
    exp: usize,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AuthService: Send + Sync {
    async fn validate_google_token(&self, token: &str) -> Result<User, AppError>;
}

pub struct GoogleAuthService {
    jwks: JwkSet,
    client_id: String,
}

impl GoogleAuthService {
    /// Creates a new GoogleAuthService instance
    ///
    /// # Arguments
    /// * `client_id` - The Google OAuth2 client ID
    /// * `jwks_url` - The URL to fetch the JWKS from. If None, uses GOOGLE_JWKS_URL env var or default
    pub async fn new(client_id: String, jwks_url: Option<&str>) -> Result<Self, AppError> {
        let url = if let Some(url) = jwks_url {
            url.to_string()
        } else {
            std::env::var("GOOGLE_JWKS_URL")
                .unwrap_or_else(|_| "https://www.googleapis.com/oauth2/v3/certs".to_string())
        };

        let jwks: JwkSet = reqwest::get(&url)
            .await
            .map_err(|e| AppError::CallError(format!("Failed to fetch Google JWKS: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::CallError(format!("Failed to parse Google JWKS: {}", e)))?;

        Ok(Self { jwks, client_id })
    }

    /// Finds a JWK (JSON Web Key) by its key ID in the JWKS set
    ///
    /// # Arguments
    /// * `kid` - The key ID to search for
    ///
    /// # Returns
    /// `Some(&Jwk)` if a key with the matching ID is found, `None` otherwise
    fn find_jwk(&self, kid: &str) -> Option<&jsonwebtoken::jwk::Jwk> {
        self.jwks
            .keys
            .iter()
            .find(|k| k.common.key_id.as_deref() == Some(kid))
    }
}

#[async_trait]
impl AuthService for GoogleAuthService {
    async fn validate_google_token(&self, token: &str) -> Result<User, AppError> {
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

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.client_id]);
        validation.set_issuer(&["https://accounts.google.com", "accounts.google.com"]);

        let token_data =
            decode::<GoogleClaims>(token, &decoding_key, &validation).map_err(|e| {
                AppError::AuthenticationError(format!("Token validation failed: {}", e))
            })?;

        Ok(User::new(
            token_data.claims.sub,
            token_data.claims.email,
            token_data.claims.name,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn google_claims_deserialization() {
        let json = r#"{
            "sub": "123456789",
            "email": "test@example.com",
            "name": "Test User",
            "iss": "https://accounts.google.com",
            "aud": "client-id",
            "exp": 1234567890
        }"#;

        let claims: GoogleClaims = serde_json::from_str(json).unwrap();
        assert_eq!(claims.sub, "123456789");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.name, Some("Test User".to_string()));
    }

    #[test]
    fn google_claims_deserialization_without_name() {
        let json = r#"{
            "sub": "987654321",
            "email": "noname@example.com",
            "iss": "https://accounts.google.com",
            "aud": "client-id",
            "exp": 1234567890
        }"#;

        let claims: GoogleClaims = serde_json::from_str(json).unwrap();
        assert_eq!(claims.sub, "987654321");
        assert_eq!(claims.email, "noname@example.com");
        assert_eq!(claims.name, None);
    }

    #[test]
    fn creates_service_with_client_id() {
        let jwks_json = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "test-key-id",
                    "n": "test-n-value",
                    "e": "AQAB",
                    "alg": "RS256",
                    "use": "sig"
                }
            ]
        }"#;
        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        assert_eq!(service.client_id, "test-client-id");
        assert_eq!(service.jwks.keys.len(), 1);
    }

    #[test]
    fn stores_empty_jwks_set() {
        let jwks_json = r#"{ "keys": [] }"#;
        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        assert!(service.jwks.keys.is_empty());
    }

    #[test]
    fn finds_jwk_by_kid_when_present() {
        let jwks_json = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "key-1",
                    "n": "test-n",
                    "e": "AQAB",
                    "alg": "RS256",
                    "use": "sig"
                },
                {
                    "kty": "RSA",
                    "kid": "key-2",
                    "n": "test-n-2",
                    "e": "AQAB",
                    "alg": "RS256",
                    "use": "sig"
                }
            ]
        }"#;

        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let result = service.find_jwk("key-1");
        assert!(result.is_some());
        assert_eq!(result.unwrap().common.key_id.as_deref(), Some("key-1"));
    }

    #[test]
    fn returns_none_when_jwk_kid_not_found() {
        let jwks_json = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "key-1",
                    "n": "test-n",
                    "e": "AQAB",
                    "alg": "RS256",
                    "use": "sig"
                }
            ]
        }"#;

        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let result = service.find_jwk("non-existent-key");
        assert!(result.is_none());
    }

    #[test]
    fn returns_none_when_jwks_empty() {
        let jwks_json = r#"{
            "keys": []
        }"#;

        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let result = service.find_jwk("any-key");
        assert!(result.is_none());
    }

    // Tests for GoogleAuthService::new
    #[tokio::test]
    async fn new_successfully_creates_service_with_valid_jwks() {
        let server = MockServer::start().await;
        let jwks_response = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "test-key",
                    "n": "test-n",
                    "e": "AQAB",
                    "alg": "RS256",
                    "use": "sig"
                }
            ]
        }"#;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(jwks_response))
            .mount(&server)
            .await;

        let service = GoogleAuthService::new("test-client-id".to_string(), Some(&server.uri()))
            .await
            .unwrap();

        assert_eq!(service.client_id, "test-client-id");
        assert_eq!(service.jwks.keys.len(), 1);
        assert!(service.find_jwk("test-key").is_some());
    }

    #[tokio::test]
    async fn new_initializes_service_with_multiple_keys() {
        let server = MockServer::start().await;
        let jwks_json = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "key-1",
                    "n": "test-n-1",
                    "e": "AQAB",
                    "alg": "RS256",
                    "use": "sig"
                },
                {
                    "kty": "RSA",
                    "kid": "key-2",
                    "n": "test-n-2",
                    "e": "AQAB",
                    "alg": "RS256",
                    "use": "sig"
                }
            ]
        }"#;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(jwks_json))
            .mount(&server)
            .await;

        let service = GoogleAuthService::new("my-app".to_string(), Some(&server.uri()))
            .await
            .unwrap();

        assert_eq!(service.client_id, "my-app");
        assert_eq!(service.jwks.keys.len(), 2);
        assert!(service.find_jwk("key-1").is_some());
        assert!(service.find_jwk("key-2").is_some());
    }

    // Tests for JWK decoding errors
    #[tokio::test]
    async fn rejects_token_with_invalid_jwk_format() {
        // Use a valid RSA JWK structure, but with incomplete values
        let jwks_json = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "test-key",
                    "n": "incomplete_n_value",
                    "e": "AQAB",
                    "alg": "RS256",
                    "use": "sig"
                }
            ]
        }"#;

        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let token_with_invalid_jwk = "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Qta2V5IiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjMiLCJlbWFpbCI6InRlc3RAZXhhbXBsZS5jb20iLCJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhdWQiOiJ0ZXN0LWNsaWVudC1pZCIsImV4cCI6OTk5OTk5OTk5OX0.signature";

        let result = service.validate_google_token(token_with_invalid_jwk).await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(msg)) => {
                assert!(msg.contains("Invalid JWK") || msg.contains("Token validation failed"));
            }
            _ => panic!("Expected AuthenticationError for invalid JWK format"),
        }
    }

    #[tokio::test]
    async fn rejects_token_when_jwk_conversion_fails() {
        // Create an EC key which cannot be used for RS256 validation
        let jwks_json = r#"{
            "keys": [
                {
                    "kty": "EC",
                    "kid": "test-key",
                    "crv": "P-256",
                    "x": "xnK82U4TiifQvt1fYd8E9kI-X4pALyVX3jIaHJKz6vc",
                    "y": "3pj3S3Y21R9fDgYX-xSB_WLcXl8hCvA0-xR9oEQOXEk",
                    "alg": "ES256",
                    "use": "sig"
                }
            ]
        }"#;

        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        // Token uses RS256 but JWK is EC/ES256
        let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Qta2V5IiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjMiLCJlbWFpbCI6InRlc3RAZXhhbXBsZS5jb20iLCJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhdWQiOiJ0ZXN0LWNsaWVudC1pZCIsImV4cCI6OTk5OTk5OTk5OX0.signature";

        let result = service.validate_google_token(token).await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(msg)) => {
                assert!(
                    msg.contains("Invalid JWK")
                        || msg.contains("Token validation failed")
                        || msg.contains("Couldn't convert")
                );
            }
            _ => panic!("Expected AuthenticationError for JWK conversion failure"),
        }
    }

    // Tests for token decoding errors
    #[tokio::test]
    async fn rejects_token_with_missing_kid_in_header() {
        let jwks_json = r#"{
            "keys": []
        }"#;
        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let token_without_kid =
            "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.signature";

        let result = service.validate_google_token(token_without_kid).await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(msg)) => {
                assert!(msg.contains("Token missing kid"));
            }
            _ => panic!("Expected AuthenticationError with missing kid message"),
        }
    }

    #[tokio::test]
    async fn rejects_token_with_unknown_kid() {
        let jwks_json = r#"{
            "keys": []
        }"#;
        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let token_with_unknown_kid = "eyJhbGciOiJSUzI1NiIsImtpZCI6InVua25vd24ta2V5IiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjMiLCJlbWFpbCI6InRlc3RAZXhhbXBsZS5jb20iLCJuYW1lIjoiVGVzdCIsImlzcyI6Imh0dHBzOi8vYWNjb3VudHMuZ29vZ2xlLmNvbSIsImF1ZCI6InRlc3QtY2xpZW50LWlkIiwiZXhwIjo5OTk5OTk5OTk5fQ.signature";

        let result = service.validate_google_token(token_with_unknown_kid).await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(msg)) => {
                assert!(msg.contains("Unknown key ID"));
            }
            _ => panic!("Expected AuthenticationError with unknown key ID message"),
        }
    }

    #[tokio::test]
    async fn rejects_malformed_token() {
        let jwks_json = r#"{
            "keys": []
        }"#;
        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let invalid_token = "not-a-valid-token";

        let result = service.validate_google_token(invalid_token).await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(msg)) => {
                assert!(msg.contains("Invalid token header"));
            }
            _ => panic!("Expected AuthenticationError for invalid header"),
        }
    }

    #[tokio::test]
    async fn rejects_token_with_invalid_base64() {
        let jwks_json = r#"{
            "keys": []
        }"#;
        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let token_with_invalid_base64 = "invalid!!!.payload!!!.signature!!!";

        let result = service
            .validate_google_token(token_with_invalid_base64)
            .await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(msg)) => {
                assert!(msg.contains("Invalid token header"));
            }
            _ => panic!("Expected AuthenticationError for invalid base64"),
        }
    }

    #[tokio::test]
    async fn rejects_token_with_wrong_algorithm() {
        let jwks_json = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "test-key",
                    "n": "test-n",
                    "e": "AQAB",
                    "alg": "RS256",
                    "use": "sig"
                }
            ]
        }"#;

        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        // Token with HS256 algorithm instead of RS256
        let token_wrong_algo = "eyJhbGciOiJIUzI1NiIsImtpZCI6InRlc3Qta2V5IiwidHlwIjoiSldUIn0.eyJzdWIiOiIxMjMiLCJlbWFpbCI6InRlc3RAZXhhbXBsZS5jb20iLCJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhdWQiOiJ0ZXN0LWNsaWVudC1pZCIsImV4cCI6OTk5OTk5OTk5OX0.signature";

        let result = service.validate_google_token(token_wrong_algo).await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(_msg)) => {
                // Error is expected for wrong algorithm
            }
            _ => panic!("Expected AuthenticationError for wrong algorithm"),
        }
    }

    #[tokio::test]
    async fn rejects_token_with_empty_string() {
        let jwks_json = r#"{
            "keys": []
        }"#;
        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let result = service.validate_google_token("").await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(msg)) => {
                assert!(msg.contains("Invalid token header"));
            }
            _ => panic!("Expected AuthenticationError for empty token"),
        }
    }

    #[tokio::test]
    async fn rejects_token_with_incomplete_jwt_parts() {
        let jwks_json = r#"{
            "keys": []
        }"#;
        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        // JWT should have 3 parts separated by dots, this only has 2
        let incomplete_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Qta2V5In0.eyJzdWIiOiIxMjMifQ";

        let result = service.validate_google_token(incomplete_token).await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(msg)) => {
                assert!(msg.contains("Invalid token header"));
            }
            _ => panic!("Expected AuthenticationError for incomplete JWT"),
        }
    }

    // Additional tests for token validation errors
    #[tokio::test]
    async fn rejects_token_with_corrupted_payload() {
        let jwks_json = r#"{
            "keys": []
        }"#;
        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        // Valid header format but corrupted base64 payload
        let corrupted_token =
            "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Qta2V5In0.!!!invalid_base64!!!.signature";

        let result = service.validate_google_token(corrupted_token).await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(_msg)) => {
                // Error is expected for corrupted payload
            }
            _ => panic!("Expected AuthenticationError for corrupted payload"),
        }
    }

    #[tokio::test]
    async fn new_handles_multiple_keys_in_jwks() {
        let server = MockServer::start().await;
        let jwks_json = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "rsa-key-1",
                    "n": "test-n-1",
                    "e": "AQAB"
                },
                {
                    "kty": "RSA",
                    "kid": "rsa-key-2",
                    "n": "test-n-2",
                    "e": "AQAB"
                },
                {
                    "kty": "RSA",
                    "kid": "rsa-key-3",
                    "n": "test-n-3",
                    "e": "AQAB"
                }
            ]
        }"#;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(jwks_json))
            .mount(&server)
            .await;

        let service = GoogleAuthService::new("test-client-id".to_string(), Some(&server.uri()))
            .await
            .unwrap();

        assert_eq!(service.jwks.keys.len(), 3);
        assert!(service.find_jwk("rsa-key-1").is_some());
        assert!(service.find_jwk("rsa-key-2").is_some());
        assert!(service.find_jwk("rsa-key-3").is_some());
        assert!(service.find_jwk("rsa-key-4").is_none());
    }

    #[tokio::test]
    async fn new_returns_error_when_http_request_fails() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
            .mount(&server)
            .await;

        let result = GoogleAuthService::new("test-client".to_string(), Some(&server.uri())).await;

        assert!(result.is_err());
        match result {
            Err(AppError::CallError(msg)) => {
                assert!(msg.contains("Failed to parse Google JWKS"));
            }
            _ => panic!("Expected CallError when response is not JSON"),
        }
    }

    #[tokio::test]
    async fn new_returns_error_when_response_is_not_json() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&server)
            .await;

        let result = GoogleAuthService::new("test-client".to_string(), Some(&server.uri())).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn new_returns_error_on_http_404() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
            .mount(&server)
            .await;

        let result = GoogleAuthService::new("test-client".to_string(), Some(&server.uri())).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn new_successfully_parses_jwks_with_all_fields() {
        let server = MockServer::start().await;
        let complete_jwks = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "complete-key",
                    "use": "sig",
                    "alg": "RS256",
                    "n": "n_value_here",
                    "e": "AQAB",
                    "n5c": ["cert1", "cert2"]
                }
            ]
        }"#;

        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string(complete_jwks))
            .mount(&server)
            .await;

        let service = GoogleAuthService::new("complete-test".to_string(), Some(&server.uri()))
            .await
            .unwrap();

        assert_eq!(service.client_id, "complete-test");
        assert_eq!(service.jwks.keys.len(), 1);
        assert!(service.find_jwk("complete-key").is_some());
    }

    #[tokio::test]
    async fn rejects_token_with_special_characters_in_payload() {
        let jwks_json = r#"{
            "keys": []
        }"#;
        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let token_with_special_chars =
            "eyJhbGciOiJSUzI1NiIsImtpZCI6InRlc3Qta2V5In0.😀😀😀😀😀.signature";

        let result = service
            .validate_google_token(token_with_special_chars)
            .await;
        assert!(result.is_err());

        match result {
            Err(AppError::AuthenticationError(_msg)) => {
                // Error is expected for special characters
            }
            _ => panic!("Expected AuthenticationError for special characters"),
        }
    }

    #[tokio::test]
    async fn finds_correct_jwk_among_many_keys() {
        let jwks_json = r#"{
            "keys": [
                {
                    "kty": "RSA",
                    "kid": "wrong-key-1",
                    "n": "test-n",
                    "e": "AQAB"
                },
                {
                    "kty": "RSA",
                    "kid": "correct-key",
                    "n": "correct-n",
                    "e": "AQAB"
                },
                {
                    "kty": "RSA",
                    "kid": "wrong-key-2",
                    "n": "test-n",
                    "e": "AQAB"
                }
            ]
        }"#;

        let jwks: JwkSet = serde_json::from_str(jwks_json).unwrap();
        let service = GoogleAuthService {
            jwks,
            client_id: "test-client-id".to_string(),
        };

        let result = service.find_jwk("correct-key");
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().common.key_id.as_deref(),
            Some("correct-key")
        );
    }
}
