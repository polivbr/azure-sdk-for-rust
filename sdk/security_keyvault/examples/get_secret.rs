use azure_identity::{ClientSecretCredential, TokenCredentialOptions};
use azure_security_keyvault::SecretClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let secret_name = env::var("SECRET_NAME").expect("Missing SECRET_NAME environment variable.");

    let creds = std::sync::Arc::new(ClientSecretCredential::new(
        azure_core::new_http_client(),
        tenant_id,
        client_id,
        client_secret,
        TokenCredentialOptions::default(),
    ));
    let client = SecretClient::new(&keyvault_url, creds)?;

    let secret = client.get(secret_name).into_future().await?;
    dbg!(secret.value);

    Ok(())
}
