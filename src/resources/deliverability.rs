//! Deliverability — reputation, blocklists, full reports.

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{client::Sendry, error::Error};

/// Deliverability resource handle.
#[derive(Debug, Clone)]
pub struct Deliverability {
    client: Sendry,
}

impl Deliverability {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Reputation overview for the org or a specific domain.
    pub async fn get_reputation(
        &self,
        params: ReputationQuery,
    ) -> Result<ReputationResponse, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/deliverability/reputation",
                &q,
                None,
            ))
            .await
    }

    /// Per-domain reputation history (raw value, schema varies).
    pub async fn get_reputation_history(
        &self,
        domain_id: &str,
        params: ReputationHistoryQuery,
    ) -> Result<Value, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                &format!("/v1/deliverability/reputation/{domain_id}/history"),
                &q,
                None,
            ))
            .await
    }

    /// Blocklist status.
    pub async fn get_blocklist(&self, params: BlocklistQuery) -> Result<BlocklistResponse, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/deliverability/blocklist",
                &q,
                None,
            ))
            .await
    }

    /// On-demand blocklist check.
    pub async fn run_blocklist_check(&self, params: BlocklistCheckBody) -> Result<Value, Error> {
        self.client
            .request(self.client.build(
                Method::POST,
                "/v1/deliverability/blocklist/check",
                &[],
                Some(&params),
            ))
            .await
    }

    /// Dismiss a blocklist alert.
    pub async fn dismiss_alert(&self, alert_id: &str) -> Result<Value, Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            status: &'a str,
        }
        self.client
            .request(self.client.build(
                Method::PATCH,
                &format!("/v1/deliverability/blocklist/alerts/{alert_id}"),
                &[],
                Some(&Body { status: "dismissed" }),
            ))
            .await
    }

    /// Full deliverability report.
    pub async fn get_report(
        &self,
        params: DeliverabilityReportQuery,
    ) -> Result<DeliverabilityReport, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/deliverability/report",
                &q,
                None,
            ))
            .await
    }
}

/// Reputation query parameters.
#[derive(Debug, Clone, Default)]
pub struct ReputationQuery {
    /// Optional domain filter.
    pub domain_id: Option<String>,
    /// Window in days (1–90).
    pub days: Option<u32>,
}

impl ReputationQuery {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(v) = &self.domain_id {
            q.push(("domain_id", v.clone()));
        }
        if let Some(v) = self.days {
            q.push(("days", v.to_string()));
        }
        q
    }
}

/// Reputation history query parameters.
#[derive(Debug, Clone, Default)]
pub struct ReputationHistoryQuery {
    /// ISO date.
    pub from: Option<String>,
    /// ISO date.
    pub to: Option<String>,
}

impl ReputationHistoryQuery {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(v) = &self.from {
            q.push(("from", v.clone()));
        }
        if let Some(v) = &self.to {
            q.push(("to", v.clone()));
        }
        q
    }
}

/// One reputation snapshot.
#[derive(Debug, Clone, Deserialize)]
pub struct ReputationSnapshot {
    /// Date.
    pub date: String,
    /// Sent.
    pub total_sent: u64,
    /// Delivered.
    pub total_delivered: u64,
    /// Bounced.
    pub total_bounced: u64,
    /// Complained.
    pub total_complained: u64,
    /// Opened.
    pub total_opened: u64,
    /// Clicked.
    pub total_clicked: u64,
    /// Delivery rate.
    pub delivery_rate: f64,
    /// Bounce rate.
    pub bounce_rate: f64,
    /// Complaint rate.
    pub complaint_rate: f64,
    /// Open rate.
    pub open_rate: f64,
    /// Click rate.
    pub click_rate: f64,
    /// Reputation score 0–100.
    pub reputation_score: f64,
}

/// Reputation factors.
#[derive(Debug, Clone, Deserialize)]
pub struct ReputationFactors {
    /// Bounce rate.
    #[serde(rename = "bounceRate")]
    pub bounce_rate: f64,
    /// Complaint rate.
    #[serde(rename = "complaintRate")]
    pub complaint_rate: f64,
    /// Delivery rate.
    #[serde(rename = "deliveryRate")]
    pub delivery_rate: f64,
    /// Engagement rate.
    #[serde(rename = "engagementRate")]
    pub engagement_rate: f64,
}

/// Current reputation.
#[derive(Debug, Clone, Deserialize)]
pub struct ReputationCurrent {
    /// Score 0–100.
    pub score: f64,
    /// Bucketed rating.
    pub rating: String,
    /// Contributing factors.
    pub factors: ReputationFactors,
    /// Recommendations.
    pub recommendations: Vec<String>,
}

/// Per-domain reputation.
#[derive(Debug, Clone, Deserialize)]
pub struct ReputationDomain {
    /// Domain id.
    pub domain_id: String,
    /// Domain name.
    pub domain_name: String,
    /// Score 0–100.
    pub score: f64,
    /// Rating.
    pub rating: String,
}

/// Reputation response.
#[derive(Debug, Clone, Deserialize)]
pub struct ReputationResponse {
    /// Current snapshot.
    pub current: ReputationCurrent,
    /// History.
    pub history: Vec<ReputationSnapshot>,
    /// Per-domain breakdown.
    pub domains: Vec<ReputationDomain>,
}

/// Blocklist query parameters.
#[derive(Debug, Clone, Default)]
pub struct BlocklistQuery {
    /// Target filter.
    pub target: Option<String>,
    /// `domain` or `ip`.
    pub target_type: Option<String>,
}

impl BlocklistQuery {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(v) = &self.target {
            q.push(("target", v.clone()));
        }
        if let Some(v) = &self.target_type {
            q.push(("target_type", v.clone()));
        }
        q
    }
}

/// Body for [`Deliverability::run_blocklist_check`].
#[derive(Debug, Clone, Serialize)]
pub struct BlocklistCheckBody {
    /// Target (domain or IP).
    pub target: String,
    /// `domain` or `ip`.
    pub target_type: String,
}

/// One blocklist check.
#[derive(Debug, Clone, Deserialize)]
pub struct BlocklistCheckItem {
    /// Row id.
    pub id: String,
    /// Target.
    pub target: String,
    /// `domain` or `ip`.
    pub target_type: String,
    /// Provider (Spamhaus, etc).
    pub provider: String,
    /// Whether listed.
    pub listed: bool,
    /// Listing reason if listed.
    pub listing_reason: Option<String>,
    /// Response time.
    pub response_time_ms: Option<u32>,
    /// Check timestamp.
    pub checked_at: String,
}

/// One blocklist alert.
#[derive(Debug, Clone, Deserialize)]
pub struct BlocklistAlertItem {
    /// Alert id.
    pub id: String,
    /// Target.
    pub target: String,
    /// `domain` or `ip`.
    pub target_type: String,
    /// Provider.
    pub provider: String,
    /// Alert status.
    pub status: String,
    /// Listing date.
    pub listed_at: String,
    /// Resolved date if any.
    pub resolved_at: Option<String>,
}

/// Summary counts.
#[derive(Debug, Clone, Deserialize)]
pub struct BlocklistSummary {
    /// Total targets.
    pub total_targets: u32,
    /// Listed count.
    pub listed_count: u32,
    /// Clean count.
    pub clean_count: u32,
}

/// Blocklist response.
#[derive(Debug, Clone, Deserialize)]
pub struct BlocklistResponse {
    /// Per-provider checks.
    pub checks: Vec<BlocklistCheckItem>,
    /// Open alerts.
    pub alerts: Vec<BlocklistAlertItem>,
    /// Summary counts.
    pub summary: BlocklistSummary,
}

/// Report query parameters.
#[derive(Debug, Clone, Default)]
pub struct DeliverabilityReportQuery {
    /// Optional domain filter.
    pub domain_id: Option<String>,
    /// Window in days (7–90).
    pub days: Option<u32>,
}

impl DeliverabilityReportQuery {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(v) = &self.domain_id {
            q.push(("domain_id", v.clone()));
        }
        if let Some(v) = self.days {
            q.push(("days", v.to_string()));
        }
        q
    }
}

/// Report period.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportPeriod {
    /// ISO date.
    pub from: String,
    /// ISO date.
    pub to: String,
    /// Days.
    pub days: u32,
}

/// Report aggregate metrics.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportMetrics {
    /// Sent count.
    pub total_sent: u64,
    /// Delivered count.
    pub total_delivered: u64,
    /// Bounced count.
    pub total_bounced: u64,
    /// Complained count.
    pub total_complained: u64,
    /// Delivery rate.
    pub delivery_rate: f64,
    /// Bounce rate.
    pub bounce_rate: f64,
    /// Complaint rate.
    pub complaint_rate: f64,
    /// Open rate.
    pub open_rate: f64,
    /// Click rate.
    pub click_rate: f64,
}

/// Report reputation summary.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportReputation {
    /// Score 0–100.
    pub score: f64,
    /// Rating.
    pub rating: String,
    /// `up`, `down`, or `stable`.
    pub trend: String,
}

/// Blocklist status block.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportBlocklistStatus {
    /// Number of lists checked.
    pub total_lists_checked: u32,
    /// Active listings.
    pub active_listings: u32,
    /// Whether currently clean.
    pub clean: bool,
}

/// Inbox placement estimate.
#[derive(Debug, Clone, Deserialize)]
pub struct InboxPlacementEstimate {
    /// Inbox percentage.
    pub inbox_pct: f64,
    /// Spam folder percentage.
    pub spam_pct: f64,
    /// Missing percentage.
    pub missing_pct: f64,
}

/// Authentication setup.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthenticationStatus {
    /// SPF.
    pub spf: bool,
    /// DKIM.
    pub dkim: bool,
    /// DMARC.
    pub dmarc: bool,
    /// BIMI.
    pub bimi: bool,
}

/// Full deliverability report.
#[derive(Debug, Clone, Deserialize)]
pub struct DeliverabilityReport {
    /// Period.
    pub period: ReportPeriod,
    /// Aggregate metrics.
    pub metrics: ReportMetrics,
    /// Reputation summary.
    pub reputation: ReportReputation,
    /// Blocklist summary.
    pub blocklist_status: ReportBlocklistStatus,
    /// Inbox placement.
    pub inbox_placement_estimate: InboxPlacementEstimate,
    /// Recommendations.
    pub recommendations: Vec<String>,
    /// Authentication.
    pub authentication: AuthenticationStatus,
}
