//! Unsubscribe list management.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, DeleteResponse, Page};

/// Unsubscribes resource handle.
#[derive(Debug, Clone)]
pub struct Unsubscribes {
    client: Sendry,
}

impl Unsubscribes {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// List unsubscribed addresses.
    pub async fn list(&self, params: ListUnsubscribes) -> Result<Page<UnsubscribeEntry>, Error> {
        let q = params.to_query();
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/unsubscribes", &q, None),
            )
            .await
    }

    /// Add a single email.
    pub async fn create(&self, params: CreateUnsubscribe) -> Result<UnsubscribeEntry, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/unsubscribes", &[], Some(&params)),
            )
            .await
    }

    /// Add up to 1000 emails in one request.
    pub async fn create_batch(
        &self,
        params: BatchUnsubscribe,
    ) -> Result<BatchUnsubscribeResponse, Error> {
        self.client
            .request(self.client.build(
                Method::POST,
                "/v1/unsubscribes/batch",
                &[],
                Some(&params),
            ))
            .await
    }

    /// Retrieve a single unsubscribe by id.
    pub async fn get(&self, id: &str) -> Result<UnsubscribeEntry, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/unsubscribes/{id}"),
                &[],
                None,
            ))
            .await
    }

    /// Remove an unsubscribe by id.
    pub async fn remove(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/unsubscribes/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Filters for [`Unsubscribes::list`].
#[derive(Debug, Clone, Default)]
pub struct ListUnsubscribes {
    /// Page size.
    pub limit: Option<u32>,
    /// Cursor.
    pub cursor: Option<String>,
    /// Filter by email.
    pub email: Option<String>,
    /// Filter by list id.
    pub list_id: Option<String>,
}

impl ListUnsubscribes {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(v) = self.limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = &self.cursor {
            q.push(("cursor", v.clone()));
        }
        if let Some(v) = &self.email {
            q.push(("email", v.clone()));
        }
        if let Some(v) = &self.list_id {
            q.push(("list_id", v.clone()));
        }
        q
    }
}

/// Parameters for [`Unsubscribes::create`].
#[derive(Debug, Clone, Serialize)]
pub struct CreateUnsubscribe {
    /// Email.
    pub email: String,
    /// Optional list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// Optional human-readable reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Parameters for [`Unsubscribes::create_batch`].
#[derive(Debug, Clone, Serialize)]
pub struct BatchUnsubscribe {
    /// Up to 1000 emails.
    pub emails: Vec<String>,
    /// Optional list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// Optional reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Response from [`Unsubscribes::create_batch`].
#[derive(Debug, Clone, Deserialize)]
pub struct BatchUnsubscribeResponse {
    /// Number of new rows inserted (idempotent).
    pub inserted: u32,
}

/// One unsubscribe record.
#[derive(Debug, Clone, Deserialize)]
pub struct UnsubscribeEntry {
    /// Row id.
    pub id: String,
    /// Email.
    pub email: String,
    /// List id (may be null = global).
    pub list_id: Option<String>,
    /// Reason.
    pub reason: Option<String>,
    /// Created timestamp.
    pub created_at: String,
}
