//! Contacts (subscribers).

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, Page};

/// Contacts resource handle.
#[derive(Debug, Clone)]
pub struct Contacts {
    client: Sendry,
}

impl Contacts {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Create or update a contact (matched by `email`).
    pub async fn upsert(&self, params: ContactParams) -> Result<Contact, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/contacts", &[], Some(&params)),
            )
            .await
    }

    /// Retrieve a contact.
    pub async fn get(&self, id: &str) -> Result<Contact, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, &format!("/v1/contacts/{id}"), &[], None),
            )
            .await
    }

    /// List contacts.
    pub async fn list(&self, audience_id: Option<&str>) -> Result<Page<Contact>, Error> {
        let q: Vec<(&str, String)> = audience_id
            .map(|a| vec![("audience_id", a.to_string())])
            .unwrap_or_default();
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/contacts", &q, None),
            )
            .await
    }

    /// Delete a contact.
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        self.client
            .request_unit(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/contacts/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Parameters for upserting a contact.
#[derive(Debug, Clone, Serialize)]
pub struct ContactParams {
    /// Email address (matching key).
    pub email: String,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Optional audience to add this contact to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
}

/// Contact record returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct Contact {
    /// Contact id.
    pub id: String,
    /// Email address.
    pub email: String,
    /// Whether the contact has unsubscribed.
    pub unsubscribed: bool,
    /// Creation timestamp.
    pub created_at: String,
}
