use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultCertificateBaseIdentifierAttributedRaw {
    pub enabled: bool,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub exp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub nbf: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultCertificateBaseIdentifierRaw {
    pub id: String,
    #[allow(unused)]
    pub x5t: String,
    pub attributes: KeyVaultCertificateBaseIdentifierAttributedRaw,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificatesResponse {
    pub value: Vec<KeyVaultCertificateBaseIdentifierRaw>,
    #[serde(rename = "nextLink")]
    pub next_link: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponse {
    pub kid: String,
    pub sid: String,
    pub x5t: String,
    pub cer: String,
    pub id: String,
    pub attributes: KeyVaultGetCertificateResponseAttributes,
    pub policy: KeyVaultGetCertificateResponsePolicy,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponseAttributes {
    pub enabled: bool,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub exp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(with = "ts_seconds_option")]
    pub nbf: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated: DateTime<Utc>,
    #[serde(rename = "recoveryLevel")]
    #[allow(unused)]
    pub recovery_level: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct KeyVaultGetCertificateResponsePolicy {
    pub id: String,
    pub key_props: KeyVaultGetCertificateResponsePolicyKeyProperties,
    pub secret_props: KeyVaultGetCertificateResponsePolicySecretProperties,
    pub x509_props: KeyVaultGetCertificateResponsePolicyX509Properties,
    pub issuer: KeyVaultGetCertificateResponsePolicyIssuer,
    pub attributes: KeyVaultGetCertificateResponsePolicyAttributes,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyKeyProperties {
    pub exportable: bool,
    pub kty: String,
    pub key_size: u64,
    pub reuse_key: bool,
}

#[derive(Deserialize, Debug)]
pub(crate) struct KeyVaultGetCertificateResponsePolicySecretProperties {
    #[serde(rename = "contentType")]
    pub content_type: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyX509Properties {
    pub subject: String,
    pub validity_months: u64,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyIssuer {
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct KeyVaultGetCertificateResponsePolicyAttributes {
    pub enabled: bool,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct CertificateBackupResponse {
    pub value: String,
}

#[derive(Debug)]
pub struct KeyVaultCertificate {
    pub key_id: String,
    pub secret_id: String,
    pub x5t: String,
    pub cer: String,
    pub content_type: String,
    pub properties: CertificateProperties,
}

#[derive(Debug)]
pub struct CertificateProperties {
    pub id: String,
    pub name: String,
    pub version: String,
    pub not_before: Option<DateTime<Utc>>,
    pub expires_on: Option<DateTime<Utc>>,
    pub created_on: DateTime<Utc>,
    pub updated_on: DateTime<Utc>,
    pub enabled: bool,
}
