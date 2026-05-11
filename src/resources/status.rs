//! Sendry system status and incidents.

use reqwest::Method;
use serde::Deserialize;

use crate::{client::Sendry, error::Error, Page, PaginationParams};

/// Status resource handle.
#[derive(Debug, Clone)]
pub struct Status {
    client: Sendry,
}

impl Status {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Current operational status.
    pub async fn get_current(&self) -> Result<SystemStatus, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/status", &[], None),
            )
            .await
    }

    /// Paginated incident history.
    pub async fn get_history(&self, params: PaginationParams) -> Result<Page<Incident>, Error> {
        let q = params.to_query();
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/status/history", &q, None),
            )
            .await
    }

    /// Hourly P50/P95/P99 latency rollups.
    pub async fn get_latency(&self, params: GetLatencyParams) -> Result<LatencyStats, Error> {
        let q = params.to_query();
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/status/latency", &q, None),
            )
            .await
    }
}

/// One incident update.
#[derive(Debug, Clone, Deserialize)]
pub struct IncidentUpdate {
    /// Update id.
    pub id: String,
    /// Status.
    pub status: String,
    /// Message.
    pub message: String,
    /// Timestamp.
    pub created_at: String,
}

/// One affected component.
#[derive(Debug, Clone, Deserialize)]
pub struct AffectedComponent {
    /// Component id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// URL slug.
    pub slug: String,
}

/// One incident.
#[derive(Debug, Clone, Deserialize)]
pub struct Incident {
    /// Incident id.
    pub id: String,
    /// Title.
    pub title: String,
    /// Status.
    pub status: String,
    /// Impact.
    pub impact: String,
    /// Start time.
    pub starts_at: Option<String>,
    /// End time.
    pub ends_at: Option<String>,
    /// Resolved time.
    pub resolved_at: Option<String>,
    /// Created.
    pub created_at: String,
    /// Updated.
    pub updated_at: String,
    /// Updates.
    pub updates: Vec<IncidentUpdate>,
    /// Affected components.
    pub affected_components: Vec<AffectedComponent>,
}

/// One status component.
#[derive(Debug, Clone, Deserialize)]
pub struct StatusComponent {
    /// Component id.
    pub id: String,
    /// Name.
    pub name: String,
    /// Description.
    pub description: Option<String>,
    /// Group.
    pub group: Option<String>,
    /// URL slug.
    pub slug: String,
    /// Status.
    pub status: String,
    /// 90-day uptime.
    pub uptime_90d: f64,
    /// SLA target.
    pub sla_target: f64,
    /// Whether SLA was met.
    pub sla_met: bool,
}

/// SLA summary.
#[derive(Debug, Clone, Deserialize)]
pub struct SlaSummary {
    /// SLA target.
    pub target: f64,
    /// Current uptime.
    pub current_uptime: f64,
    /// Whether SLA was met.
    pub sla_met: bool,
}

/// System status snapshot.
#[derive(Debug, Clone, Deserialize)]
pub struct SystemStatus {
    /// Overall status.
    pub status: String,
    /// Per-component status.
    pub components: Vec<StatusComponent>,
    /// Active incidents.
    pub active_incidents: Vec<Incident>,
    /// SLA summary.
    pub sla_summary: SlaSummary,
}

/// One hour of latency data.
#[derive(Debug, Clone, Deserialize)]
pub struct LatencyHourBucket {
    /// Hour timestamp.
    pub hour: String,
    /// P50 ms.
    pub p50_ms: Option<f64>,
    /// P95 ms.
    pub p95_ms: Option<f64>,
    /// P99 ms.
    pub p99_ms: Option<f64>,
    /// Sample count.
    pub sample_count: u64,
    /// Percentage that met the target.
    pub target_met_pct: f64,
}

/// Latency stats.
#[derive(Debug, Clone, Deserialize)]
pub struct LatencyStats {
    /// Component name.
    pub component: String,
    /// Target ms.
    pub target_ms: f64,
    /// Current P50.
    pub current_p50_ms: Option<f64>,
    /// Whether the target was met.
    pub target_met: bool,
    /// Per-hour data.
    pub hourly: Vec<LatencyHourBucket>,
}

/// Parameters for [`Status::get_latency`].
#[derive(Debug, Clone, Default)]
pub struct GetLatencyParams {
    /// Component slug (default `api-gateway`).
    pub component: Option<String>,
    /// Window in hours (1–168, default 24).
    pub hours: Option<u32>,
}

impl GetLatencyParams {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(v) = &self.component {
            q.push(("component", v.clone()));
        }
        if let Some(v) = self.hours {
            q.push(("hours", v.to_string()));
        }
        q
    }
}
