//! Email templates.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, Page};

/// Templates resource handle.
#[derive(Debug, Clone)]
pub struct Templates {
    client: Sendry,
}

impl Templates {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Create a template.
    pub async fn create(&self, params: TemplateParams) -> Result<Template, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/templates", &[], Some(&params)),
            )
            .await
    }

    /// Retrieve a template.
    pub async fn get(&self, id: &str) -> Result<Template, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/templates/{id}"),
                &[],
                None,
            ))
            .await
    }

    /// List templates.
    pub async fn list(&self) -> Result<Page<Template>, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/templates", &[], None),
            )
            .await
    }

    /// Delete a template.
    pub async fn delete(&self, id: &str) -> Result<(), Error> {
        self.client
            .request_unit(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/templates/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Parameters for creating or updating a template.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateParams {
    /// Display name.
    pub name: String,
    /// Subject line (supports `{{variable}}` substitution).
    pub subject: String,
    /// HTML body.
    pub html: String,
    /// Engine — `"html"` or `"react"`.
    pub engine: String,
}

/// Template record returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct Template {
    /// Template id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Subject.
    pub subject: String,
    /// Creation timestamp.
    pub created_at: String,
}
