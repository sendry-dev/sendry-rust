//! Per-user notification preferences.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error};

/// Notification preferences resource handle.
#[derive(Debug, Clone)]
pub struct NotificationPreferencesResource {
    client: Sendry,
}

impl NotificationPreferencesResource {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Get the current user's notification preferences.
    pub async fn get(&self) -> Result<NotificationPreferences, Error> {
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/notification-preferences",
                &[],
                None,
            ))
            .await
    }

    /// Update the current user's notification preferences.
    pub async fn update(
        &self,
        params: UpdateNotificationPreferences,
    ) -> Result<NotificationPreferences, Error> {
        self.client
            .request(self.client.build(
                Method::PUT,
                "/v1/notification-preferences",
                &[],
                Some(&params),
            ))
            .await
    }
}

/// Notification preferences record.
#[derive(Debug, Clone, Deserialize)]
pub struct NotificationPreferences {
    /// Row id.
    pub id: String,
    /// Bounce alerts.
    pub bounce_alerts: bool,
    /// Complaint alerts.
    pub complaint_alerts: bool,
    /// Delivery failure alerts.
    pub delivery_failures: bool,
    /// Domain issues.
    pub domain_issues: bool,
    /// Daily summary email.
    pub daily_summary: bool,
    /// Weekly digest email.
    pub weekly_digest: bool,
    /// Monthly report.
    pub monthly_report: bool,
    /// All events firehose.
    pub all_events: bool,
    /// Delivery events.
    pub delivery_events: bool,
    /// Engagement events.
    pub engagement_events: bool,
    /// Compliance events.
    pub compliance_events: bool,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

/// Parameters for [`NotificationPreferencesResource::update`]. All fields are optional.
///
/// Field names use camelCase to match the server contract.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateNotificationPreferences {
    /// Bounce alerts.
    #[serde(rename = "bounceAlerts", skip_serializing_if = "Option::is_none")]
    pub bounce_alerts: Option<bool>,
    /// Complaint alerts.
    #[serde(rename = "complaintAlerts", skip_serializing_if = "Option::is_none")]
    pub complaint_alerts: Option<bool>,
    /// Delivery failure alerts.
    #[serde(rename = "deliveryFailures", skip_serializing_if = "Option::is_none")]
    pub delivery_failures: Option<bool>,
    /// Domain issues.
    #[serde(rename = "domainIssues", skip_serializing_if = "Option::is_none")]
    pub domain_issues: Option<bool>,
    /// Daily summary email.
    #[serde(rename = "dailySummary", skip_serializing_if = "Option::is_none")]
    pub daily_summary: Option<bool>,
    /// Weekly digest email.
    #[serde(rename = "weeklyDigest", skip_serializing_if = "Option::is_none")]
    pub weekly_digest: Option<bool>,
    /// Monthly report.
    #[serde(rename = "monthlyReport", skip_serializing_if = "Option::is_none")]
    pub monthly_report: Option<bool>,
    /// All events firehose.
    #[serde(rename = "allEvents", skip_serializing_if = "Option::is_none")]
    pub all_events: Option<bool>,
    /// Delivery events.
    #[serde(rename = "deliveryEvents", skip_serializing_if = "Option::is_none")]
    pub delivery_events: Option<bool>,
    /// Engagement events.
    #[serde(rename = "engagementEvents", skip_serializing_if = "Option::is_none")]
    pub engagement_events: Option<bool>,
    /// Compliance events.
    #[serde(rename = "complianceEvents", skip_serializing_if = "Option::is_none")]
    pub compliance_events: Option<bool>,
}
