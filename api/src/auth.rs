use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeycloakClaims {
    pub preferred_username: String,
    pub email: Option<String>,
    pub exp: usize,
    pub iss: String,
}

pub struct AuthUser {
    pub username: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".into()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid Authorization header".into()))?;

        // ⚠️ For now: skip signature validation
        let header = decode_header(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token header".into()))?;

        let key = DecodingKey::from_secret(&[]); // TEMP

        let validation = Validation::default();

        let claims = decode::<KeycloakClaims>(token, &key, &validation)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))?
            .claims;

        Ok(AuthUser {
            username: claims.preferred_username,
        })
    }
}
