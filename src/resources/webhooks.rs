//! Webhook subscriptions.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, Page};

/// Webhooks resource handle.
#[derive(Debug, Clone)]
pub struct Webhooks {
    client: Sendry,
}

impl Webhooks {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Register a webhook endpoint.
    pub async fn create(&self, params: WebhookParams) -> Result<Webhook, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/webhooks", &[], Some(&params)),
            )
            .await
    }

    /// Retrieve a webhook.
    pub async fn get(&self, id: &str) -> Result<Webhook, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, &format!("/v1/webhooks/{id}"), &[], None),
            )
            .await
    }

    /// List webhooks.
    pub async fn list(&self) -> Result<Page<Webhook>, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/webhooks", &[], None),
            )
            .await
    }

    /// Remove a webhook.
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        self.client
            .request_unit(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/webhooks/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Parameters for creating a webhook.
#[derive(Debug, Clone, Serialize)]
pub struct WebhookParams {
    /// HTTPS URL to POST events to.
    pub url: String,
    /// Event types to subscribe to, e.g. `["email.delivered"]`.
    pub events: Vec<String>,
}

/// Webhook record returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct Webhook {
    /// Webhook id.
    pub id: String,
    /// Endpoint URL.
    pub url: String,
    /// Subscribed events.
    pub events: Vec<String>,
    /// Signing secret (only returned on create).
    #[serde(default)]
    pub secret: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
}
