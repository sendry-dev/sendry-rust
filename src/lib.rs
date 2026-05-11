//! Official Rust client for the [Sendry](https://sendry.online) email API.
//!
//! ```no_run
//! use sendry::{Sendry, SendEmail};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Sendry::new(std::env::var("SENDRY_API_KEY")?);
//!
//! let resp = client.emails().send(SendEmail {
//!     from:    "hello@yourdomain.com".into(),
//!     to:      vec!["user@example.com".into()],
//!     subject: "Welcome".into(),
//!     html:    Some("<p>Hi.</p>".into()),
//!     ..Default::default()
//! }).await?;
//!
//! println!("sent: {}", resp.id);
//! # Ok(()) }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![allow(clippy::module_name_repetitions)]

mod client;
mod error;
mod resources;
mod webhook;

pub use client::{Sendry, SendryBuilder};
pub use error::Error;
pub use resources::{
    audiences::{Audience, AudienceParams, Audiences},
    campaigns::{Campaign, CampaignParams, Campaigns},
    contacts::{Contact, ContactParams, Contacts},
    domains::{Domain, DomainParams, Domains},
    emails::{BatchEmail, BatchEmailItem, BatchResponse, Email, EmailResponse, Emails, SendEmail},
    templates::{Template, TemplateParams, Templates},
    webhooks::{Webhook, WebhookParams, Webhooks},
};
pub use webhook::verify_webhook_signature;

/// SDK version, set by Cargo at compile time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Common pagination wrapper returned by every `list` endpoint.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Page<T> {
    /// The items returned for this page.
    pub data: Vec<T>,
    /// True if more pages exist beyond `next_cursor`.
    pub has_more: bool,
    /// Opaque cursor for the next page, or `None` if at the end.
    pub next_cursor: Option<String>,
}
