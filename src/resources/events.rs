//! Events — external signals that can trigger automations.

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{client::Sendry, error::Error, Page};

/// Events resource handle.
#[derive(Debug, Clone)]
pub struct Events {
    client: Sendry,
}

impl Events {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Ingest a new event.
    pub async fn ingest(&self, params: IngestEvent) -> Result<IngestedEvent, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/events", &[], Some(&params)),
            )
            .await
    }

    /// List ingested events.
    pub async fn list(&self, params: ListEvents) -> Result<Page<IngestedEvent>, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(Method::GET, "/v1/events", &q, None))
            .await
    }

    /// Get an event by id.
    pub async fn get(&self, id: &str) -> Result<IngestedEvent, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/events/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Parameters for [`Events::ingest`].
#[derive(Debug, Clone, Serialize)]
pub struct IngestEvent {
    /// Event name.
    pub name: String,
    /// Caller-provided idempotency id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Associated contact email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    /// Associated contact id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// Arbitrary payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
}

/// Filters for [`Events::list`].
#[derive(Debug, Clone, Default)]
pub struct ListEvents {
    /// Page size.
    pub limit: Option<u32>,
    /// Cursor.
    pub cursor: Option<String>,
    /// Filter by name.
    pub name: Option<String>,
}

impl ListEvents {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(v) = self.limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = &self.cursor {
            q.push(("cursor", v.clone()));
        }
        if let Some(v) = &self.name {
            q.push(("name", v.clone()));
        }
        q
    }
}

/// Ingested event record.
#[derive(Debug, Clone, Deserialize)]
pub struct IngestedEvent {
    /// Internal id.
    pub id: String,
    /// External / caller-supplied id.
    pub external_id: Option<String>,
    /// Event name.
    pub name: String,
    /// Contact email.
    pub contact_email: Option<String>,
    /// Contact id.
    pub contact_id: Option<String>,
    /// Payload.
    pub payload: Value,
    /// Received timestamp.
    pub received_at: String,
    /// Processed timestamp.
    pub processed_at: Option<String>,
    /// Number of automation runs triggered.
    pub triggered_runs: u32,
    /// Whether this was deduplicated.
    #[serde(default)]
    pub deduped: Option<bool>,
}
