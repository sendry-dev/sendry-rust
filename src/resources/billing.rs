//! Billing — plan, usage, Stripe checkout / portal.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{client::Sendry, error::Error};

/// Billing resource handle.
#[derive(Debug, Clone)]
pub struct Billing {
    client: Sendry,
}

impl Billing {
    pub(crate) fn new(client: Sendry) -> Self {
        Self { client }
    }

    /// Current plan and subscription status.
    pub async fn get_plan(&self) -> Result<BillingPlan, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/billing/plan", &[], None),
            )
            .await
    }

    /// Period-to-date usage.
    pub async fn get_usage(&self) -> Result<BillingUsage, Error> {
        self.client
            .request(
                self.client
                    .build::<()>(Method::GET, "/v1/billing/usage", &[], None),
            )
            .await
    }

    /// Create a Stripe Checkout session for an upgrade.
    pub async fn create_checkout(
        &self,
        params: CreateCheckout,
    ) -> Result<CheckoutSession, Error> {
        self.client
            .request(self.client.build(
                Method::POST,
                "/v1/billing/checkout",
                &[],
                Some(&params),
            ))
            .await
    }

    /// Create a Stripe Billing Portal session for self-service management.
    pub async fn create_portal(&self, params: CreatePortal) -> Result<PortalSession, Error> {
        self.client
            .request(self.client.build(
                Method::POST,
                "/v1/billing/portal",
                &[],
                Some(&params),
            ))
            .await
    }
}

/// Plan info.
#[derive(Debug, Clone, Deserialize)]
pub struct BillingPlan {
    /// Plan name (e.g. `pro`).
    pub plan: String,
    /// Whether a Stripe subscription exists.
    #[serde(rename = "hasSubscription")]
    pub has_subscription: bool,
    /// `monthly` or `annual`.
    #[serde(rename = "billingPeriod")]
    pub billing_period: String,
}

/// Period usage.
#[derive(Debug, Clone, Deserialize)]
pub struct BillingUsage {
    /// Emails sent this period.
    pub emails_sent_this_period: u64,
    /// Plan limit.
    pub plan_limit: u64,
    /// Overage count.
    pub overage_count: u64,
    /// Per-email overage rate, if any.
    pub overage_rate: Option<f64>,
    /// Period end ISO date.
    pub period_end: Option<String>,
}

/// Parameters for [`Billing::create_checkout`].
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateCheckout {
    /// Target plan.
    pub plan: String,
    /// `monthly` or `annual`.
    #[serde(rename = "billingPeriod", skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<String>,
    /// Redirect on success.
    #[serde(rename = "successUrl", skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    /// Redirect on cancel.
    #[serde(rename = "cancelUrl", skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
}

/// Parameters for [`Billing::create_portal`].
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreatePortal {
    /// Optional return URL.
    #[serde(rename = "returnUrl", skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

/// Stripe Checkout session.
#[derive(Debug, Clone, Deserialize)]
pub struct CheckoutSession {
    /// Hosted Stripe URL.
    pub url: String,
}

/// Stripe Billing Portal session.
#[derive(Debug, Clone, Deserialize)]
pub struct PortalSession {
    /// Hosted Stripe URL.
    pub url: String,
}
