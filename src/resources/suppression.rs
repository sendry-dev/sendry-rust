//! Suppression list (hard-bounced / complained addresses we won't send to).

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, DeleteResponse, Page, PaginationParams};

/// Suppression resource handle.
#[derive(Debug, Clone)]
pub struct Suppression {
    client: Sendry,
}

impl Suppression {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// List suppressed addresses.
    pub async fn list(&self, params: PaginationParams) -> Result<Page<SuppressionEntry>, Error> {
        let q = params.to_query();
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/suppression", &q, None),
            )
            .await
    }

    /// Add an address to the suppression list.
    pub async fn add(&self, params: AddSuppression) -> Result<SuppressionEntry, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/suppression", &[], Some(&params)),
            )
            .await
    }

    /// Remove an address.
    pub async fn remove(&self, email: &str) -> Result<DeleteResponse, Error> {
        let encoded = urlencode(email);
        self.client
            .request(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/suppression/{encoded}"),
                &[],
                None,
            ))
            .await
    }
}

/// Parameters for [`Suppression::add`].
#[derive(Debug, Clone, Serialize)]
pub struct AddSuppression {
    /// Email address.
    pub email: String,
    /// Reason: `hard_bounce`, `complaint`, `unsubscribe`, or `manual`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// One suppression row.
#[derive(Debug, Clone, Deserialize)]
pub struct SuppressionEntry {
    /// Email address.
    pub email: String,
    /// Reason it was added.
    pub reason: String,
    /// When added.
    pub created_at: String,
}

fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
