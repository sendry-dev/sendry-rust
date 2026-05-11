//! Organisation profile and branding settings.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error};

/// Organizations resource handle.
#[derive(Debug, Clone)]
pub struct Organizations {
    client: Sendry,
}

impl Organizations {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Get the current organisation.
    pub async fn get_current(&self) -> Result<Organization, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/organizations/me",
                &[],
                None,
            ))
            .await
    }

    /// Update the current organisation's name.
    pub async fn update(&self, params: UpdateOrganization) -> Result<Organization, Error> {
        self.client
            .request(self.client.build(
                Method::PATCH,
                "/v1/organizations/me",
                &[],
                Some(&params),
            ))
            .await
    }

    /// Get branding settings.
    pub async fn get_branding(&self) -> Result<BrandingSettings, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/organizations/me/branding",
                &[],
                None,
            ))
            .await
    }

    /// Update branding settings.
    pub async fn update_branding(
        &self,
        params: UpdateBranding,
    ) -> Result<BrandingSettings, Error> {
        self.client
            .request(self.client.build(
                Method::PATCH,
                "/v1/organizations/me/branding",
                &[],
                Some(&params),
            ))
            .await
    }
}

/// Organisation record.
#[derive(Debug, Clone, Deserialize)]
pub struct Organization {
    /// Org id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Plan.
    pub plan: String,
    /// Creation timestamp.
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

/// Parameters for [`Organizations::update`].
#[derive(Debug, Clone, Serialize)]
pub struct UpdateOrganization {
    /// New name.
    pub name: String,
}

/// Branding settings.
#[derive(Debug, Clone, Deserialize)]
pub struct BrandingSettings {
    /// Hex colour.
    pub brand_color: String,
    /// Logo URL.
    pub brand_logo: Option<String>,
    /// Unsubscribe page heading.
    pub unsubscribe_heading: Option<String>,
    /// Unsubscribe page message.
    pub unsubscribe_message: Option<String>,
    /// Post-unsubscribe redirect URL.
    pub unsubscribe_redirect_url: Option<String>,
}

/// Parameters for [`Organizations::update_branding`]. All fields optional.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateBranding {
    /// Hex colour.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_color: Option<String>,
    /// Logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_logo: Option<String>,
    /// Unsubscribe page heading.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_heading: Option<String>,
    /// Unsubscribe page message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_message: Option<String>,
    /// Post-unsubscribe redirect URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_redirect_url: Option<String>,
}
