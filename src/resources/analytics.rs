//! Aggregated email analytics, event logs, cohorts, benchmarks, and exports.

use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{client::Sendry, error::Error, Page};

/// Analytics resource handle.
#[derive(Debug, Clone)]
pub struct Analytics {
    client: Sendry,
}

impl Analytics {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Aggregated stats (summary + timeseries).
    pub async fn stats(&self, params: AnalyticsParams) -> Result<AnalyticsResponse, Error> {
        let q = params.to_query();
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/analytics", &q, None),
            )
            .await
    }

    /// Event logs (paginated).
    pub async fn logs(&self, params: LogsParams) -> Result<Page<LogEvent>, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(Method::GET, "/v1/logs", &q, None))
            .await
    }

    /// Cohort analysis.
    pub async fn cohorts(&self, params: CohortParams) -> Result<CohortResponse, Error> {
        let q = params.to_query();
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/analytics/cohorts", &q, None),
            )
            .await
    }

    /// Cross-org benchmarks.
    pub async fn benchmarks(&self, params: BenchmarkParams) -> Result<BenchmarkResponse, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/analytics/benchmarks",
                &q,
                None,
            ))
            .await
    }

    /// Opt in or out of benchmark sharing.
    pub async fn toggle_benchmark_opt_in(&self, opt_in: bool) -> Result<Value, Error> {
        #[derive(Serialize)]
        struct Body {
            opt_in: bool,
        }
        self.client
            .request(self.client.build(
                Method::POST,
                "/v1/analytics/benchmarks/opt-in",
                &[],
                Some(&Body { opt_in }),
            ))
            .await
    }

    /// Per-domain or per-template breakdowns.
    pub async fn breakdowns(&self, params: BreakdownParams) -> Result<BreakdownResponse, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/analytics/breakdowns",
                &q,
                None,
            ))
            .await
    }

    /// Compare metrics against the previous equivalent period.
    pub async fn comparison(&self, params: AnalyticsParams) -> Result<ComparisonResponse, Error> {
        let q = params.to_query();
        self.client
            .request(self.client.build::<()>(
                Method::GET,
                "/v1/analytics/comparison",
                &q,
                None,
            ))
            .await
    }

    /// Export analytics data as CSV or JSON (raw JSON value).
    pub async fn export(&self, params: ExportParams) -> Result<Value, Error> {
        let q = params.to_query();
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/analytics/export", &q, None),
            )
            .await
    }
}

/// Stats query parameters.
#[derive(Debug, Clone, Default)]
pub struct AnalyticsParams {
    /// ISO date (inclusive lower bound).
    pub from: String,
    /// ISO date (inclusive upper bound).
    pub to: String,
    /// `hour`, `day`, `week`, or `month`.
    pub granularity: Option<String>,
    /// Filter by SES tag.
    pub tag: Option<String>,
    /// Filter by domain.
    pub domain: Option<String>,
}

impl AnalyticsParams {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = vec![("from", self.from.clone()), ("to", self.to.clone())];
        if let Some(v) = &self.granularity {
            q.push(("granularity", v.clone()));
        }
        if let Some(v) = &self.tag {
            q.push(("tag", v.clone()));
        }
        if let Some(v) = &self.domain {
            q.push(("domain", v.clone()));
        }
        q
    }
}

/// Per-bucket counts.
#[derive(Debug, Clone, Deserialize)]
pub struct AnalyticsBucket {
    /// Bucket date.
    pub date: String,
    /// Sent count.
    pub sent: u64,
    /// Delivered count.
    pub delivered: u64,
    /// Opened count.
    pub opened: u64,
    /// Clicked count.
    pub clicked: u64,
    /// Bounced count.
    pub bounced: u64,
    /// Complained count.
    pub complained: u64,
}

/// Summary totals + rates.
#[derive(Debug, Clone, Deserialize)]
pub struct AnalyticsSummary {
    /// Sent count.
    pub sent: u64,
    /// Delivered count.
    pub delivered: u64,
    /// Opened count.
    pub opened: u64,
    /// Clicked count.
    pub clicked: u64,
    /// Bounced count.
    pub bounced: u64,
    /// Complained count.
    pub complained: u64,
    /// Delivered / sent.
    pub delivery_rate: f64,
    /// Opened / delivered.
    pub open_rate: f64,
    /// Clicked / delivered.
    pub click_rate: f64,
    /// Bounced / sent.
    pub bounce_rate: f64,
    /// Complained / delivered.
    pub complaint_rate: f64,
}

/// Response from [`Analytics::stats`].
#[derive(Debug, Clone, Deserialize)]
pub struct AnalyticsResponse {
    /// Period totals.
    pub summary: AnalyticsSummary,
    /// Per-bucket data.
    pub timeseries: Vec<AnalyticsBucket>,
}

/// Filters for [`Analytics::logs`].
#[derive(Debug, Clone, Default)]
pub struct LogsParams {
    /// Page size.
    pub limit: Option<u32>,
    /// Cursor from a previous page.
    pub cursor: Option<String>,
    /// Filter by email id.
    pub email_id: Option<String>,
    /// Filter by event type.
    pub event_type: Option<String>,
    /// Filter by recipient.
    pub to: Option<String>,
    /// Lower bound date.
    pub from_date: Option<String>,
    /// Upper bound date.
    pub to_date: Option<String>,
}

impl LogsParams {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(v) = self.limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = &self.cursor {
            q.push(("cursor", v.clone()));
        }
        if let Some(v) = &self.email_id {
            q.push(("email_id", v.clone()));
        }
        if let Some(v) = &self.event_type {
            q.push(("type", v.clone()));
        }
        if let Some(v) = &self.to {
            q.push(("to", v.clone()));
        }
        if let Some(v) = &self.from_date {
            q.push(("from_date", v.clone()));
        }
        if let Some(v) = &self.to_date {
            q.push(("to_date", v.clone()));
        }
        q
    }
}

/// One log line.
#[derive(Debug, Clone, Deserialize)]
pub struct LogEvent {
    /// Event id.
    pub id: String,
    /// Email id.
    pub email_id: String,
    /// Event type.
    #[serde(rename = "type")]
    pub event_type: String,
    /// Recipient address.
    pub recipient: String,
    /// Optional metadata blob.
    pub metadata: Option<Value>,
    /// Event timestamp.
    pub created_at: String,
}

/// Cohort query parameters.
#[derive(Debug, Clone, Default)]
pub struct CohortParams {
    /// ISO date.
    pub from: String,
    /// ISO date.
    pub to: String,
    /// `day`, `week`, or `month`.
    pub granularity: Option<String>,
    /// `open_rate`, `click_rate`, or `delivery_rate`.
    pub metric: Option<String>,
}

impl CohortParams {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = vec![("from", self.from.clone()), ("to", self.to.clone())];
        if let Some(v) = &self.granularity {
            q.push(("granularity", v.clone()));
        }
        if let Some(v) = &self.metric {
            q.push(("metric", v.clone()));
        }
        q
    }
}

/// One cohort bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct CohortBucket {
    /// Cohort date.
    pub cohort_date: String,
    /// Period offset.
    pub period_offset: i32,
    /// Total sent.
    pub total_sent: u64,
    /// Metric value.
    pub metric_value: f64,
}

/// Cohort response.
#[derive(Debug, Clone, Deserialize)]
pub struct CohortResponse {
    /// Buckets.
    pub cohorts: Vec<CohortBucket>,
    /// Echoed granularity.
    pub granularity: String,
    /// Echoed metric.
    pub metric: String,
}

/// Benchmark query parameters.
#[derive(Debug, Clone, Default)]
pub struct BenchmarkParams {
    /// ISO date.
    pub from: String,
    /// ISO date.
    pub to: String,
    /// `day`, `week`, or `month`.
    pub granularity: Option<String>,
}

impl BenchmarkParams {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = vec![("from", self.from.clone()), ("to", self.to.clone())];
        if let Some(v) = &self.granularity {
            q.push(("granularity", v.clone()));
        }
        q
    }
}

/// Per-day benchmark bucket.
#[derive(Debug, Clone, Deserialize)]
pub struct BenchmarkBucket {
    /// Bucket date.
    pub date: String,
    /// Your delivery rate.
    pub your_delivery_rate: f64,
    /// Your open rate.
    pub your_open_rate: f64,
    /// Your click rate.
    pub your_click_rate: f64,
    /// Average delivery rate.
    pub avg_delivery_rate: f64,
    /// Average open rate.
    pub avg_open_rate: f64,
    /// Average click rate.
    pub avg_click_rate: f64,
    /// P50 delivery rate.
    pub p50_delivery_rate: f64,
    /// P50 open rate.
    pub p50_open_rate: f64,
    /// P50 click rate.
    pub p50_click_rate: f64,
    /// P75 delivery rate.
    pub p75_delivery_rate: f64,
    /// P75 open rate.
    pub p75_open_rate: f64,
    /// P75 click rate.
    pub p75_click_rate: f64,
}

/// Benchmark response.
#[derive(Debug, Clone, Deserialize)]
pub struct BenchmarkResponse {
    /// Per-bucket data.
    pub data: Vec<BenchmarkBucket>,
    /// Whether your org has opted into the program.
    pub benchmark_opt_in: bool,
    /// Number of orgs in the dataset.
    pub org_count: u32,
}

/// Breakdown query parameters.
#[derive(Debug, Clone, Default)]
pub struct BreakdownParams {
    /// ISO date.
    pub from: String,
    /// ISO date.
    pub to: String,
    /// `domain` or `template`.
    pub group_by: String,
    /// Optional row limit.
    pub limit: Option<u32>,
}

impl BreakdownParams {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = vec![
            ("from", self.from.clone()),
            ("to", self.to.clone()),
            ("group_by", self.group_by.clone()),
        ];
        if let Some(v) = self.limit {
            q.push(("limit", v.to_string()));
        }
        q
    }
}

/// One breakdown row.
#[derive(Debug, Clone, Deserialize)]
pub struct BreakdownItem {
    /// Group id.
    pub id: Option<String>,
    /// Group name.
    pub name: Option<String>,
    /// Sent count.
    pub sent: u64,
    /// Delivered count.
    pub delivered: u64,
    /// Opened count.
    pub opened: u64,
    /// Clicked count.
    pub clicked: u64,
    /// Bounced count.
    pub bounced: u64,
    /// Complained count.
    pub complained: u64,
    /// Delivery rate.
    pub delivery_rate: f64,
    /// Open rate.
    pub open_rate: f64,
    /// Click rate.
    pub click_rate: f64,
}

/// Breakdown response.
#[derive(Debug, Clone, Deserialize)]
pub struct BreakdownResponse {
    /// Rows.
    pub data: Vec<BreakdownItem>,
    /// Echoed group dimension.
    pub group_by: String,
}

/// Comparison period stats.
#[derive(Debug, Clone, Deserialize)]
pub struct ComparisonPeriodStats {
    /// Sent count.
    pub sent: u64,
    /// Delivered count.
    pub delivered: u64,
    /// Opened count.
    pub opened: u64,
    /// Clicked count.
    pub clicked: u64,
    /// Bounced count.
    pub bounced: u64,
    /// Complained count.
    pub complained: u64,
    /// Delivery rate.
    pub delivery_rate: f64,
    /// Open rate.
    pub open_rate: f64,
    /// Click rate.
    pub click_rate: f64,
    /// Bounce rate.
    pub bounce_rate: f64,
    /// Complaint rate.
    pub complaint_rate: f64,
}

/// Period-over-period change percentages and deltas.
#[derive(Debug, Clone, Deserialize)]
pub struct ComparisonChanges {
    /// Percent change in sent count.
    pub sent_pct: f64,
    /// Percent change in delivered count.
    pub delivered_pct: f64,
    /// Percent change in opened count.
    pub opened_pct: f64,
    /// Percent change in clicked count.
    pub clicked_pct: f64,
    /// Percent change in bounced count.
    pub bounced_pct: f64,
    /// Percent change in complained count.
    pub complained_pct: f64,
    /// Delivery rate delta (pp).
    pub delivery_rate_delta: f64,
    /// Open rate delta (pp).
    pub open_rate_delta: f64,
    /// Click rate delta (pp).
    pub click_rate_delta: f64,
    /// Bounce rate delta (pp).
    pub bounce_rate_delta: f64,
    /// Complaint rate delta (pp).
    pub complaint_rate_delta: f64,
}

/// Comparison response.
#[derive(Debug, Clone, Deserialize)]
pub struct ComparisonResponse {
    /// Current period totals.
    pub current: ComparisonPeriodStats,
    /// Previous period totals.
    pub previous: ComparisonPeriodStats,
    /// Deltas.
    pub changes: ComparisonChanges,
}

/// Export parameters.
#[derive(Debug, Clone, Default)]
pub struct ExportParams {
    /// ISO date.
    pub from: String,
    /// ISO date.
    pub to: String,
    /// `hour`, `day`, `week`, or `month`.
    pub granularity: Option<String>,
    /// `csv` or `json`.
    pub format: Option<String>,
    /// Domain filter.
    pub domain: Option<String>,
}

impl ExportParams {
    fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = vec![("from", self.from.clone()), ("to", self.to.clone())];
        if let Some(v) = &self.granularity {
            q.push(("granularity", v.clone()));
        }
        if let Some(v) = &self.format {
            q.push(("format", v.clone()));
        }
        if let Some(v) = &self.domain {
            q.push(("domain", v.clone()));
        }
        q
    }
}
