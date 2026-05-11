//! API key management.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, DeleteResponse, Page};

/// API keys resource handle.
#[derive(Debug, Clone)]
pub struct ApiKeys {
    client: Sendry,
}

impl ApiKeys {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Create a new API key. The plaintext `key` is only returned once.
    pub async fn create(&self, params: CreateApiKey) -> Result<ApiKeyCreated, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/api-keys", &[], Some(&params)),
            )
            .await
    }

    /// List API keys (values are masked, only prefix is shown).
    pub async fn list(&self) -> Result<Page<ApiKey>, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/api-keys", &[], None),
            )
            .await
    }

    /// Revoke (delete) an API key.
    pub async fn remove(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/api-keys/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Parameters for creating an API key.
#[derive(Debug, Clone, Serialize)]
pub struct CreateApiKey {
    /// Display name.
    pub name: String,
    /// Scope: `full_access`, `sending_access`, or `read_only`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// API key record (key value is masked).
#[derive(Debug, Clone, Deserialize)]
pub struct ApiKey {
    /// API key id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Scope name.
    pub scope: String,
    /// Visible prefix (first few characters).
    pub key_prefix: String,
    /// Timestamp of last use, if any.
    pub last_used_at: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
}

/// Response from [`ApiKeys::create`]. Includes the full plaintext key — store it now.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiKeyCreated {
    /// API key id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Scope name.
    pub scope: String,
    /// The full plaintext API key. Only returned once.
    pub key: String,
    /// Creation timestamp.
    pub created_at: String,
}
