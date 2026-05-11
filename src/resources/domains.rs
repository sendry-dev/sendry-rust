//! Sending-domain registration and verification.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, Page};

/// Domains resource handle.
#[derive(Debug, Clone)]
pub struct Domains {
    client: Sendry,
}

impl Domains {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Register a new sending domain.
    pub async fn create(&self, params: DomainParams) -> Result<Domain, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/domains", &[], Some(&params)),
            )
            .await
    }

    /// Trigger DNS verification.
    pub async fn verify(&self, id: &str) -> Result<Domain, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::POST,
                &format!("/v1/domains/{id}/verify"),
                &[],
                None,
            ))
            .await
    }

    /// Retrieve a domain.
    pub async fn get(&self, id: &str) -> Result<Domain, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, &format!("/v1/domains/{id}"), &[], None),
            )
            .await
    }

    /// List domains.
    pub async fn list(&self) -> Result<Page<Domain>, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/domains", &[], None),
            )
            .await
    }

    /// Remove a domain.
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        self.client
            .request_unit(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/domains/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Parameters for creating a domain.
#[derive(Debug, Clone, Serialize)]
pub struct DomainParams {
    /// The domain name, e.g. `mail.example.com`.
    pub name: String,
    /// Optional friendly region — defaults to your org default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Domain record returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct Domain {
    /// Domain id.
    pub id: String,
    /// Domain name.
    pub name: String,
    /// Verification status (`pending`, `verified`, `failed`).
    pub status: String,
    /// Creation timestamp.
    pub created_at: String,
}
