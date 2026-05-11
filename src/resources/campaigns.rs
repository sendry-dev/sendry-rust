//! Broadcast campaigns.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error, Page};

/// Campaigns resource handle.
#[derive(Debug, Clone)]
pub struct Campaigns {
    client: Sendry,
}

impl Campaigns {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Create a draft campaign.
    pub async fn create(&self, params: CampaignParams) -> Result<Campaign, Error> {
        self.client
            .request(
                self.client
                    .build(Method::POST, "/v1/campaigns", &[], Some(&params)),
            )
            .await
    }

    /// Retrieve a campaign.
    pub async fn get(&self, id: &str) -> Result<Campaign, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/campaigns/{id}"),
                &[],
                None,
            ))
            .await
    }

    /// List campaigns.
    pub async fn list(&self) -> Result<Page<Campaign>, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/campaigns", &[], None),
            )
            .await
    }

    /// Send a campaign immediately.
    pub async fn send(&self, id: &str) -> Result<Campaign, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::POST,
                &format!("/v1/campaigns/{id}/send"),
                &[],
                None,
            ))
            .await
    }
}

/// Parameters for creating a campaign.
#[derive(Debug, Clone, Serialize)]
pub struct CampaignParams {
    /// Display name.
    pub name: String,
    /// Subject line.
    pub subject: String,
    /// Sender address.
    pub from: String,
    /// Audience to send to.
    pub audience_id: String,
    /// HTML body.
    pub html: String,
    /// Plain-text body.
    pub text: String,
}

/// Campaign record returned by the API.
#[derive(Debug, Clone, Deserialize)]
pub struct Campaign {
    /// Campaign id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// `draft`, `sending`, `sent`, `paused`, `cancelled`.
    pub status: String,
    /// Count of emails sent so far.
    pub sent_count: u32,
    /// Total recipients.
    pub total_recipients: u32,
    /// Creation timestamp.
    pub created_at: String,
}
