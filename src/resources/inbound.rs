//! Inbound email handling.

use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{client::Sendry, error::Error, Page, PaginationParams};

/// Inbound resource handle.
#[derive(Debug, Clone)]
pub struct Inbound {
    client: Sendry,
}

impl Inbound {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// List received inbound emails.
    pub async fn list(&self, params: PaginationParams) -> Result<Page<InboundEmail>, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(Method::GET, "/v1/inbound", &q, None))
            .await
    }

    /// Retrieve a single inbound email.
    pub async fn get(&self, id: &str) -> Result<InboundEmail, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/inbound/{id}"),
                &[],
                None,
            ))
            .await
    }

    /// Get the inbound forwarding configuration.
    pub async fn get_config(&self) -> Result<InboundConfig, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/inbound/config",
                &[],
                None,
            ))
            .await
    }

    /// Update the inbound forwarding configuration.
    pub async fn update_config(
        &self,
        params: UpdateInboundConfig,
    ) -> Result<InboundConfig, Error> {
        self.client
            .request(self.client.build(
                Method::PUT,
                "/v1/inbound/config",
                &[],
                Some(&params),
            ))
            .await
    }
}

/// One inbound email attachment.
#[derive(Debug, Clone, Deserialize)]
pub struct InboundEmailAttachment {
    /// File name.
    pub filename: String,
    /// MIME type.
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// Size in bytes.
    pub size: u64,
    /// Optional inline content id.
    #[serde(rename = "contentId", default)]
    pub content_id: Option<String>,
}

/// Inbound email record.
#[derive(Debug, Clone, Deserialize)]
pub struct InboundEmail {
    /// Inbound email id.
    pub id: String,
    /// From address.
    pub from: String,
    /// To addresses.
    pub to: Vec<String>,
    /// CC addresses.
    #[serde(default)]
    pub cc: Vec<String>,
    /// Subject.
    pub subject: Option<String>,
    /// Plain-text body.
    pub text: Option<String>,
    /// HTML body.
    pub html: Option<String>,
    /// Raw headers.
    pub headers: Option<HashMap<String, String>>,
    /// Attachments.
    #[serde(default)]
    pub attachments: Vec<InboundEmailAttachment>,
    /// Whether webhook delivery succeeded.
    pub webhook_delivered: bool,
    /// Received timestamp.
    pub created_at: String,
}

/// Inbound forwarding configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct InboundConfig {
    /// Forwarding URL.
    pub url: Option<String>,
    /// HMAC signing secret.
    pub secret: Option<String>,
}

/// Parameters for [`Inbound::update_config`].
#[derive(Debug, Clone, Serialize)]
pub struct UpdateInboundConfig {
    /// Forwarding URL (or `None` to disable).
    pub url: Option<String>,
    /// Optional HMAC signing secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
