use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{Algorithm, decode, decode_header, DecodingKey, Validation};
use serde::{Serialize, Deserialize};

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

        // Fetch Keycloak's public key from the JWKS endpoint
        let keycloak_url = std::env::var("KEYCLOAK_ISSUER_URL")
            .expect("KEYCLOAK_ISSUER_URL must be set");
        let jwks_url = format!("{}/protocol/openid-connect/certs", keycloak_url);

        let jwks: serde_json::Value = reqwest::get(&jwks_url)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch JWKS".into()))?
            .json()
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JWKS".into()))?;

        // Extract the first RSA public key
        let key = jwks["keys"][0]["x5c"][0]
            .as_str()
            .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "No public key found".into()))?;

        let pem = format!(
            "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----",
            key
        );

        let decoding_key = DecodingKey::from_rsa_pem(pem.as_bytes())
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Invalid public key".into()))?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[&keycloak_url]);
        validation.set_audience(&["account"]);

        let claims = decode::<KeycloakClaims>(token, &decoding_key, &validation)
            .map_err(|e| {
                println!("Token decode error: {}", e); 
                (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", e))
            })?
            .claims;

        Ok(AuthUser {
            username: claims.preferred_username,
        })
    }
}


use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    Json,
};
use openidconnect::{
    core::CoreClient,
    AuthorizationCode, CsrfToken, Nonce, Scope,
    OAuth2TokenResponse, TokenResponse,
    reqwest::async_http_client,
};
use openidconnect::core::CoreAuthenticationFlow;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

/// Shared app state — store CSRF tokens keyed by state value
/// In production, use a proper session store (e.g. tower-sessions + Redis)
pub type OidcStateStore = Arc<Mutex<HashMap<String, String>>>; // state -> nonce

#[derive(Clone)]
pub struct OidcAppState {
    pub client: Arc<CoreClient>,
    pub state_store: OidcStateStore,
}

/// GET /api/auth/oidc/login
/// Redirects the user to Keycloak's authorization endpoint
pub async fn oidc_login_handler(
    State(oidc): State<OidcAppState>,
) -> impl IntoResponse {
    let (auth_url, csrf_token, nonce) = oidc
        .client
        .authorize_url(
            CoreAuthenticationFlow::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    // Persist nonce keyed by CSRF state so we can verify it in the callback
    oidc.state_store
        .lock()
        .await
        .insert(csrf_token.secret().clone(), nonce.secret().clone());

    Redirect::temporary(auth_url.as_str())
}

/// Query params Keycloak sends back to the callback URL
#[derive(Debug, Deserialize)]
pub struct OidcCallbackParams {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize)]
pub struct AuthTokenResponse {
    pub access_token: String,
    pub id_token: Option<String>,
    pub expires_in: Option<u64>,
}

/// GET /api/auth/oidc/callback
/// Exchanges the authorization code for tokens and verifies the ID token
pub async fn oidc_callback_handler(
    State(oidc): State<OidcAppState>,
    Query(params): Query<OidcCallbackParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let nonce_secret = oidc
        .state_store
        .lock()
        .await
        .remove(&params.state)
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Invalid or expired CSRF state".to_string()))?;

    let token_response = oidc
        .client
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(async_http_client)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let id_token = token_response
        .id_token()
        .ok_or_else(|| (StatusCode::INTERNAL_SERVER_ERROR, "No ID token returned".to_string()))?;

    id_token
        .claims(&oidc.client.id_token_verifier(), &Nonce::new(nonce_secret))
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let access_token = token_response.access_token().secret().to_string();
    let refresh_token = token_response.refresh_token()
        .map(|t| t.secret().to_string())
        .unwrap_or_default();

    // Redirect to frontend with token in URL
    let frontend_url = std::env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
    let redirect = format!(
        "{}/dashboard.html?access_token={}&refresh_token={}",
        std::env::var("FRONTEND_URL").unwrap(),
        access_token,
        refresh_token
    );
    Ok(Redirect::temporary(&redirect))
}

pub async fn oidc_logout_handler(
    State(oidc): State<OidcAppState>,
    Json(body): Json<LogoutRequest>,
) -> impl IntoResponse {
    let keycloak_url = std::env::var("KEYCLOAK_ISSUER_URL").expect("KEYCLOAK_ISSUER_URL must be set");
    let client_id = std::env::var("KEYCLOAK_CLIENT_ID").expect("KEYCLOAK_CLIENT_ID must be set");
    let client_secret = std::env::var("KEYCLOAK_CLIENT_SECRET").expect("KEYCLOAK_CLIENT_SECRET must be set");

    let logout_url = format!("{}/protocol/openid-connect/logout", keycloak_url);

    let client = reqwest::Client::new();
    let res = client
        .post(&logout_url)
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("refresh_token", body.refresh_token.as_str()),
        ])
        .send()
        .await;

    match res {
        Ok(r) if r.status().is_success() => StatusCode::OK.into_response(),
        Ok(r) => (StatusCode::BAD_GATEWAY, format!("Keycloak error: {}", r.status())).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}


pub async fn validate_handler(
   auth: AuthUser,  // this already rejects invalid/expired tokens
) -> impl IntoResponse {
    StatusCode::OK
}
