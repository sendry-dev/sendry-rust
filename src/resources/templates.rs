//! Email templates.

use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{client::Sendry, error::Error, DeleteResponse, Page};

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

    /// Update a template.
    pub async fn update(&self, id: &str, params: UpdateTemplate) -> Result<Template, Error> {
        self.client
            .request(self.client.build(
                Method::PUT,
                &format!("/v1/templates/{id}"),
                &[],
                Some(&params),
            ))
            .await
    }

    /// Render a saved template with the given variables.
    pub async fn render(
        &self,
        id: &str,
        params: RenderTemplate,
    ) -> Result<RenderTemplateResponse, Error> {
        self.client
            .request(self.client.build(
                Method::POST,
                &format!("/v1/templates/{id}/render"),
                &[],
                Some(&params),
            ))
            .await
    }

    /// List pre-built starter templates.
    pub async fn list_starters(&self) -> Result<TemplateStarterList, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/templates/starters",
                &[],
                None,
            ))
            .await
    }

    /// Delete a template.
    pub async fn delete(&self, id: &str) -> Result<DeleteResponse, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::DELETE,
                &format!("/v1/templates/{id}"),
                &[],
                None,
            ))
            .await
    }
}

/// Parameters for creating a template.
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

/// Parameters for [`Templates::update`]. All fields optional.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateTemplate {
    /// Display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Subject.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// HTML body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// Variable schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Value>,
}

/// Parameters for [`Templates::render`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct RenderTemplate {
    /// Variable values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, String>>,
}

/// Response from [`Templates::render`].
#[derive(Debug, Clone, Deserialize)]
pub struct RenderTemplateResponse {
    /// Rendered HTML.
    pub html: String,
    /// Rendered plain-text.
    pub text: String,
}

/// One starter template.
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateStarter {
    /// Starter id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Subject.
    pub subject: String,
    /// HTML body.
    pub html: String,
    /// Variable names.
    pub variables: Vec<String>,
    /// Engine.
    pub engine: String,
}

/// Wrapper for [`Templates::list_starters`].
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateStarterList {
    /// Starters.
    pub data: Vec<TemplateStarter>,
}

/// Template record returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct Template {
    /// Template id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Subject.
    pub subject: Option<String>,
    /// HTML body.
    pub html: Option<String>,
    /// Engine.
    #[serde(default)]
    pub engine: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
}
