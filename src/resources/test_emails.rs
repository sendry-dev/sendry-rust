//! Captured test-mode emails.

use reqwest::Method;
use serde::Deserialize;

use crate::{client::Sendry, error::Error, Page, PaginationParams};

/// Test emails resource handle.
#[derive(Debug, Clone)]
pub struct TestEmails {
    client: Sendry,
}

impl TestEmails {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// List captured test emails.
    pub async fn list(
        &self,
        params: PaginationParams,
    ) -> Result<Page<TestEmailSummary>, Error> {
        let q = params.to_query();
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/test-emails", &q, None),
            )
            .await
    }

    /// Get a specific test email by id.
    pub async fn get(&self, id: &str) -> Result<TestEmail, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/test-emails/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Test email summary (for list responses).
#[derive(Debug, Clone, Deserialize)]
pub struct TestEmailSummary {
    /// Test email id.
    pub id: String,
    /// From address.
    pub from: String,
    /// To addresses.
    pub to: Vec<String>,
    /// Subject.
    pub subject: String,
    /// Message type.
    pub message_type: String,
    /// Created.
    pub created_at: String,
}

/// Full test email.
#[derive(Debug, Clone, Deserialize)]
pub struct TestEmail {
    /// Test email id.
    pub id: String,
    /// From.
    pub from: String,
    /// To.
    pub to: Vec<String>,
    /// CC.
    pub cc: Option<Vec<String>>,
    /// Subject.
    pub subject: String,
    /// HTML body.
    pub html: Option<String>,
    /// Plain-text body.
    pub text: Option<String>,
    /// Message type.
    pub message_type: String,
    /// Created.
    pub created_at: String,
}
