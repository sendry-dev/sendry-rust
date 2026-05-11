//! SES sending regions and data residency.

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{client::Sendry, error::Error};

/// Regions resource handle.
#[derive(Debug, Clone)]
pub struct Regions {
    client: Sendry,
}

impl Regions {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// List available regions.
    pub async fn list(&self) -> Result<RegionList, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/regions", &[], None),
            )
            .await
    }

    /// Get org-level region settings.
    pub async fn get_org_settings(&self) -> Result<OrgRegionSettings, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/organizations/me/region",
                &[],
                None,
            ))
            .await
    }

    /// Update org-level region settings.
    pub async fn update_org_settings(
        &self,
        params: UpdateOrgRegion,
    ) -> Result<OrgRegionSettings, Error> {
        self.client
            .request(self.client.build(
                Method::PATCH,
                "/v1/organizations/me/region",
                &[],
                Some(&params),
            ))
            .await
    }

    /// Override the region for a specific domain. Pass `None` for `region` to clear.
    pub async fn set_domain_region(
        &self,
        domain_id: &str,
        params: UpdateDomainRegion,
    ) -> Result<Value, Error> {
        self.client
            .request(self.client.build(
                Method::PATCH,
                &format!("/v1/domains/{domain_id}/region"),
                &[],
                Some(&params),
            ))
            .await
    }

    /// Per-region send-count breakdown for a date range.
    pub async fn get_region_analytics(
        &self,
        params: RegionAnalyticsParams,
    ) -> Result<RegionAnalyticsResponse, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/analytics/regions",
                &q,
                None,
            ))
            .await
    }
}

/// One region.
#[derive(Debug, Clone, Deserialize)]
pub struct Region {
    /// SES region code (e.g. `us-east-1`).
    pub region_code: String,
    /// Display name.
    pub display_name: String,
    /// Whether this is the org default.
    pub is_default: bool,
}

/// Wrapper returned by [`Regions::list`].
#[derive(Debug, Clone, Deserialize)]
pub struct RegionList {
    /// Regions.
    pub data: Vec<Region>,
}

/// Org region settings.
#[derive(Debug, Clone, Deserialize)]
pub struct OrgRegionSettings {
    /// Default region (may be null).
    pub default_region: Option<String>,
    /// Data residency: `none`, `eu`, `us`, or `ap`.
    pub data_residency: String,
}

/// Parameters for [`Regions::update_org_settings`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateOrgRegion {
    /// Default region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_region: Option<String>,
    /// Data residency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_residency: Option<String>,
}

/// Parameters for [`Regions::set_domain_region`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateDomainRegion {
    /// Region code, or `None` to clear the override.
    pub region: Option<String>,
}

/// Parameters for [`Regions::get_region_analytics`].
#[derive(Debug, Clone, Default)]
pub struct RegionAnalyticsParams {
    /// ISO date.
    pub from: String,
    /// ISO date.
    pub to: String,
}

impl RegionAnalyticsParams {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        vec![("from", self.from.clone()), ("to", self.to.clone())]
    }
}

/// One row of region analytics.
#[derive(Debug, Clone, Deserialize)]
pub struct RegionAnalyticsItem {
    /// Region code.
    pub region: String,
    /// Send count.
    pub count: u64,
    /// Share of total.
    pub percentage: f64,
}

/// Region analytics response.
#[derive(Debug, Clone, Deserialize)]
pub struct RegionAnalyticsResponse {
    /// Rows.
    pub data: Vec<RegionAnalyticsItem>,
}
