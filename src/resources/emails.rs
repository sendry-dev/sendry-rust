//! Send and inspect emails.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, Page};

/// Emails resource handle.
#[derive(Debug, Clone)]
pub struct Emails {
    client: Sendry,
}

impl Emails {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Send a single email.
    pub async fn send(&self, params: SendEmail) -> Result<EmailResponse, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/emails", &[], Some(&params)),
            )
            .await
    }

    /// Send a batch of up to 100 emails in one request.
    pub async fn send_batch(&self, params: BatchEmail) -> Result<BatchResponse, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/emails/batch", &[], Some(&params)),
            )
            .await
    }

    /// Retrieve a single email by id.
    pub async fn get(&self, id: &str) -> Result<Email, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, &format!("/v1/emails/{id}"), &[], None),
            )
            .await
    }

    /// List emails (cursor-paginated).
    pub async fn list(&self, query: ListEmails) -> Result<Page<Email>, Error> {
        let q = query.to_query();
        self.client
            .request(self.client.build::<()>(Method::GET, "/v1/emails", &q, None))
            .await
    }
}

/// Parameters for [`Emails::send`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct SendEmail {
    /// The verified sender address. Required.
    pub from: String,
    /// Recipients.
    pub to: Vec<String>,
    /// Subject line.
    pub subject: String,
    /// HTML body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Plain-text body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Carbon copy recipients.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    /// Blind carbon copy recipients.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    /// Reply-To address.
    #[serde(skip_serializing_if = "Option::is_none", rename = "reply_to")]
    pub reply_to: Option<String>,
    /// Custom SMTP headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
    /// SES message tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// Inline attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// Template id to render (mutually exclusive with `html`/`text`).
    #[serde(skip_serializing_if = "Option::is_none", rename = "template_id")]
    pub template_id: Option<String>,
    /// Template variable substitutions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
}

/// Single SES message tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// Tag name.
    pub name: String,
    /// Tag value.
    pub value: String,
}

/// File attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// File name shown to the recipient.
    pub filename: String,
    /// Base64-encoded content.
    pub content: String,
    /// MIME type.
    #[serde(rename = "contentType")]
    pub content_type: String,
}

/// Response from [`Emails::send`].
#[derive(Debug, Clone, Deserialize)]
pub struct EmailResponse {
    /// Sendry-assigned id, e.g. `em_abc123`.
    pub id: String,
    /// `queued` initially; later moves to `sent`/`bounced`/etc.
    pub status: String,
}

/// Single email as returned by `get` / `list`.
#[derive(Debug, Clone, Deserialize)]
pub struct Email {
    /// Email id.
    pub id: String,
    /// From address.
    pub from: String,
    /// To addresses.
    pub to: Vec<String>,
    /// Subject line.
    pub subject: String,
    /// Current status.
    pub status: String,
    /// Send timestamp (ISO-8601).
    pub created_at: String,
}

/// Parameters for [`Emails::send_batch`].
#[derive(Debug, Clone, Serialize)]
pub struct BatchEmail {
    /// Shared sender address.
    pub from: String,
    /// Up to 100 emails.
    pub emails: Vec<BatchEmailItem>,
}

/// One item in a batch send.
#[derive(Debug, Clone, Default, Serialize)]
pub struct BatchEmailItem {
    /// Recipient.
    pub to: String,
    /// Subject.
    pub subject: String,
    /// HTML body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Plain-text body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Response from [`Emails::send_batch`].
#[derive(Debug, Clone, Deserialize)]
pub struct BatchResponse {
    /// Count of emails queued.
    pub sent: u32,
    /// Per-item failures (validation errors only).
    pub failed: Vec<BatchFailure>,
}

/// One failed item in a batch response.
#[derive(Debug, Clone, Deserialize)]
pub struct BatchFailure {
    /// Recipient that failed.
    pub to: String,
    /// Server-side error message.
    pub error: String,
}

/// Filters for [`Emails::list`].
#[derive(Debug, Clone, Default)]
pub struct ListEmails {
    /// Page size (max 100).
    pub limit: Option<u32>,
    /// Cursor from a previous page.
    pub cursor: Option<String>,
    /// Filter by status.
    pub status: Option<String>,
}

impl ListEmails {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(l) = self.limit {
            q.push(("limit", l.to_string()));
        }
        if let Some(c) = &self.cursor {
            q.push(("cursor", c.clone()));
        }
        if let Some(s) = &self.status {
            q.push(("status", s.clone()));
        }
        q
    }
}
