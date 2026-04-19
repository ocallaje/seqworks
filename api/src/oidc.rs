use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    ClientId, ClientSecret, IssuerUrl, RedirectUrl,
};

pub async fn build_oidc_client() -> CoreClient {
    let issuer = IssuerUrl::new(
        std::env::var("KEYCLOAK_ISSUER_URL")
            // e.g. "https://keycloak.example.com/realms/my-realm"
            .expect("KEYCLOAK_ISSUER_URL must be set"),
    )
    .unwrap();

    let metadata = CoreProviderMetadata::discover_async(issuer, async_http_client)
        .await
        .expect("Failed to discover OIDC provider metadata");

    CoreClient::from_provider_metadata(
        metadata,
        ClientId::new(std::env::var("KEYCLOAK_CLIENT_ID").expect("KEYCLOAK_CLIENT_ID must be set")),
        Some(ClientSecret::new(
            std::env::var("KEYCLOAK_CLIENT_SECRET").expect("KEYCLOAK_CLIENT_SECRET must be set"),
        )),
    )
    .set_redirect_uri(
        RedirectUrl::new(
            std::env::var("KEYCLOAK_REDIRECT_URI")
                // e.g. "http://localhost:8080/api/auth/callback"
                .expect("KEYCLOAK_REDIRECT_URI must be set"),
        )
        .unwrap(),
    )
}