//! Audiences (contact lists).

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, Page};

/// Audiences resource handle.
#[derive(Debug, Clone)]
pub struct Audiences {
    client: Sendry,
}

impl Audiences {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Create an audience.
    pub async fn create(&self, params: AudienceParams) -> Result<Audience, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/audiences", &[], Some(&params)),
            )
            .await
    }

    /// Retrieve an audience.
    pub async fn get(&self, id: &str) -> Result<Audience, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/audiences/{id}"),
                &[],
                None,
            ))
            .await
    }

    /// List audiences.
    pub async fn list(&self) -> Result<Page<Audience>, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/audiences", &[], None),
            )
            .await
    }

    /// Delete an audience.
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        self.client
            .request_unit(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/audiences/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Parameters for creating an audience.
#[derive(Debug, Clone, Serialize)]
pub struct AudienceParams {
    /// Display name.
    pub name: String,
}

/// Audience record returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct Audience {
    /// Audience id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Subscriber count.
    pub contact_count: u32,
    /// Creation timestamp.
    pub created_at: String,
}
