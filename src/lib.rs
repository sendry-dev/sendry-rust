//! Official Rust client for the [Sendry](https://sendry.online) email API.
//!
//! ```no_run
//! use sendry::{Sendry, SendEmail};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let client = Sendry::new(std::env::var("SENDRY_API_KEY")?);
//!
//! let resp = client.emails().send(SendEmail {
//!     from:    "hello@yourdomain.com".into(),
//!     to:      vec!["user@example.com".into()],
//!     subject: "Welcome".into(),
//!     html:    Some("<p>Hi.</p>".into()),
//!     ..Default::default()
//! }).await?;
//!
//! println!("sent: {}", resp.id);
//! # Ok(()) }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![allow(clippy::module_name_repetitions)]

mod client;
mod error;
mod resources;
mod webhook;

pub use client::{Sendry, SendryBuilder};
pub use error::Error;
pub use resources::{
    analytics::{
        Analytics, AnalyticsBucket, AnalyticsParams, AnalyticsResponse, AnalyticsSummary,
        BenchmarkBucket, BenchmarkParams, BenchmarkResponse, BreakdownItem, BreakdownParams,
        BreakdownResponse, CohortBucket, CohortParams, CohortResponse, ComparisonChanges,
        ComparisonPeriodStats, ComparisonResponse, ExportParams, LogEvent, LogsParams,
    },
    api_keys::{ApiKey, ApiKeyCreated, ApiKeys, CreateApiKey},
    audiences::{Audience, AudienceParams, Audiences},
    automations::{
        AbSplit, AddAutomationStep, Automation, AutomationRun, AutomationRunStep,
        AutomationRunStepList, AutomationRuns, AutomationStep, AutomationStepConfig,
        AutomationStepList, AutomationSteps, Automations, BranchCondition, CreateAutomation,
        CreateAutomationRun, ListAutomationRuns, ListAutomations, UpdateAutomation,
        UpdateAutomationStep,
    },
    billing::{Billing, BillingPlan, BillingUsage, CheckoutSession, CreateCheckout, CreatePortal,
        PortalSession},
    campaigns::{Campaign, CampaignParams, Campaigns},
    contacts::{Contact, ContactParams, Contacts},
    dedicated_ips::{
        AssignIp, DedicatedIp, DedicatedIpAssignment, DedicatedIps, IpAssignment,
        ProvisionDedicatedIp,
    },
    deliverability::{
        AuthenticationStatus, BlocklistAlertItem, BlocklistCheckBody, BlocklistCheckItem,
        BlocklistQuery, BlocklistResponse, BlocklistSummary, Deliverability, DeliverabilityReport,
        DeliverabilityReportQuery, InboxPlacementEstimate, ReportBlocklistStatus, ReportMetrics,
        ReportPeriod, ReportReputation, ReputationCurrent, ReputationDomain, ReputationFactors,
        ReputationHistoryQuery, ReputationQuery, ReputationResponse, ReputationSnapshot,
    },
    domains::{Domain, DomainParams, Domains},
    emails::{
        BatchEmail, BatchEmailItem, BatchResponse, CancelEmailResponse, Email, EmailResponse,
        Emails, SendEmail, SendMarketingEmail,
    },
    events::{Events, IngestEvent, IngestedEvent, ListEvents},
    inbound::{Inbound, InboundConfig, InboundEmail, InboundEmailAttachment, UpdateInboundConfig},
    notification_preferences::{
        NotificationPreferences, NotificationPreferencesResource, UpdateNotificationPreferences,
    },
    organizations::{
        BrandingSettings, Organization, Organizations, UpdateBranding, UpdateOrganization,
    },
    regions::{
        OrgRegionSettings, Region, RegionAnalyticsItem, RegionAnalyticsParams,
        RegionAnalyticsResponse, RegionList, Regions, UpdateDomainRegion, UpdateOrgRegion,
    },
    status::{
        AffectedComponent, GetLatencyParams, Incident, IncidentUpdate, LatencyHourBucket,
        LatencyStats, SlaSummary, Status, StatusComponent, SystemStatus,
    },
    suppression::{AddSuppression, Suppression, SuppressionEntry},
    team::{InviteTeamMember, ListTeamResponse, Team, TeamMember, TeamSeats, UpdateTeamMemberRole},
    templates::{
        RenderTemplate, RenderTemplateResponse, Template, TemplateParams, TemplateStarter,
        TemplateStarterList, Templates, UpdateTemplate,
    },
    test_emails::{TestEmail, TestEmailSummary, TestEmails},
    unsubscribes::{
        BatchUnsubscribe, BatchUnsubscribeResponse, CreateUnsubscribe, ListUnsubscribes,
        UnsubscribeEntry, Unsubscribes,
    },
    webhooks::{Webhook, WebhookParams, Webhooks},
};
pub use webhook::verify_webhook_signature;

/// SDK version, set by Cargo at compile time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Common pagination wrapper returned by every `list` endpoint.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Page<T> {
    /// The items returned for this page.
    pub data: Vec<T>,
    /// True if more pages exist beyond `next_cursor`.
    pub has_more: bool,
    /// Opaque cursor for the next page, or `None` if at the end.
    pub next_cursor: Option<String>,
}

/// Cursor pagination query parameters.
#[derive(Debug, Clone, Default)]
pub struct PaginationParams {
    /// Max items per page (server-capped).
    pub limit: Option<u32>,
    /// Opaque cursor from a previous page.
    pub cursor: Option<String>,
}

impl PaginationParams {
    /// Serialise into `(name, value)` query pairs.
    #[must_use]
    pub fn to_query(&self) -> Vec<(&'static str, String)> {
        let mut q = Vec::new();
        if let Some(l) = self.limit {
            q.push(("limit", l.to_string()));
        }
        if let Some(c) = &self.cursor {
            q.push(("cursor", c.clone()));
        }
        q
    }
}

/// Simple deletion acknowledgement returned by many `DELETE` endpoints.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct DeleteResponse {
    /// Always `true` on success.
    pub deleted: bool,
}
