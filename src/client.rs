//! Top-level [`Sendry`] client and builder.

use std::sync::Arc;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::error::Error;
use crate::resources::{
    analytics::Analytics, api_keys::ApiKeys, audiences::Audiences, automations::Automations,
    billing::Billing, campaigns::Campaigns, contacts::Contacts, dedicated_ips::DedicatedIps,
    deliverability::Deliverability, domains::Domains, emails::Emails, events::Events,
    inbound::Inbound, notification_preferences::NotificationPreferencesResource,
    organizations::Organizations, regions::Regions, status::Status, suppression::Suppression,
    team::Team, templates::Templates, test_emails::TestEmails, unsubscribes::Unsubscribes,
    webhooks::Webhooks,
};
use crate::VERSION;

const DEFAULT_BASE_URL: &str = "https://api.sendry.online";
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const DEFAULT_RETRIES: u8 = 2;

/// Async client for the Sendry API.
///
/// Cheap to clone — all internal state lives behind an `Arc`.
#[derive(Debug, Clone)]
pub struct Sendry {
    pub(crate) inner: Arc<Inner>,
}

#[derive(Debug)]
pub(crate) struct Inner {
    pub(crate) http: Client,
    pub(crate) base_url: String,
    pub(crate) retries: u8,
}

/// Builder for customising a [`Sendry`] client.
#[derive(Debug, Default)]
pub struct SendryBuilder {
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    retries: Option<u8>,
}

impl SendryBuilder {
    /// API key — required.
    #[must_use]
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Override the base URL (default `https://api.sendry.online`).
    #[must_use]
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Per-request timeout (default `30s`).
    #[must_use]
    pub fn timeout(mut self, d: Duration) -> Self {
        self.timeout = Some(d);
        self
    }

    /// Max retry attempts on 5xx / network errors (default `2`).
    #[must_use]
    pub fn max_retries(mut self, n: u8) -> Self {
        self.retries = Some(n);
        self
    }

    /// Construct the [`Sendry`] client.
    pub fn build(self) -> Result<Sendry, Error> {
        let api_key = self
            .api_key
            .ok_or_else(|| Error::Authentication("api_key is required".into()))?;

        let mut headers = HeaderMap::new();
        let auth = HeaderValue::from_str(&format!("Bearer {api_key}"))
            .map_err(|_| Error::Authentication("invalid api_key bytes".into()))?;
        headers.insert(AUTHORIZATION, auth);
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&format!("sendry-rust/{VERSION}")).unwrap(),
        );

        let http = Client::builder()
            .default_headers(headers)
            .timeout(self.timeout.unwrap_or(DEFAULT_TIMEOUT))
            .build()
            .map_err(Error::Network)?;

        Ok(Sendry {
            inner: Arc::new(Inner {
                http,
                base_url: self
                    .base_url
                    .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
                    .trim_end_matches('/')
                    .to_string(),
                retries: self.retries.unwrap_or(DEFAULT_RETRIES),
            }),
        })
    }
}

impl Sendry {
    /// Convenience constructor — equivalent to `Sendry::builder().api_key(k).build()`.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::builder()
            .api_key(api_key)
            .build()
            .expect("api key required")
    }

    /// Returns a builder for advanced configuration.
    #[must_use]
    pub fn builder() -> SendryBuilder {
        SendryBuilder::default()
    }

    /// Emails resource.
    #[must_use]
    pub fn emails(&self) -> Emails {
        Emails::new(self.clone())
    }

    /// Domains resource.
    #[must_use]
    pub fn domains(&self) -> Domains {
        Domains::new(self.clone())
    }

    /// Templates resource.
    #[must_use]
    pub fn templates(&self) -> Templates {
        Templates::new(self.clone())
    }

    /// Contacts resource.
    #[must_use]
    pub fn contacts(&self) -> Contacts {
        Contacts::new(self.clone())
    }

    /// Audiences resource.
    #[must_use]
    pub fn audiences(&self) -> Audiences {
        Audiences::new(self.clone())
    }

    /// Campaigns resource.
    #[must_use]
    pub fn campaigns(&self) -> Campaigns {
        Campaigns::new(self.clone())
    }

    /// Webhooks resource.
    #[must_use]
    pub fn webhooks(&self) -> Webhooks {
        Webhooks::new(self.clone())
    }

    /// API keys resource.
    #[must_use]
    pub fn api_keys(&self) -> ApiKeys {
        ApiKeys::new(self.clone())
    }

    /// Analytics resource.
    #[must_use]
    pub fn analytics(&self) -> Analytics {
        Analytics::new(self.clone())
    }

    /// Suppression list resource.
    #[must_use]
    pub fn suppression(&self) -> Suppression {
        Suppression::new(self.clone())
    }

    /// Unsubscribes resource.
    #[must_use]
    pub fn unsubscribes(&self) -> Unsubscribes {
        Unsubscribes::new(self.clone())
    }

    /// Billing resource.
    #[must_use]
    pub fn billing(&self) -> Billing {
        Billing::new(self.clone())
    }

    /// Team resource.
    #[must_use]
    pub fn team(&self) -> Team {
        Team::new(self.clone())
    }

    /// Dedicated IPs resource.
    #[must_use]
    pub fn dedicated_ips(&self) -> DedicatedIps {
        DedicatedIps::new(self.clone())
    }

    /// Deliverability resource.
    #[must_use]
    pub fn deliverability(&self) -> Deliverability {
        Deliverability::new(self.clone())
    }

    /// Inbound resource.
    #[must_use]
    pub fn inbound(&self) -> Inbound {
        Inbound::new(self.clone())
    }

    /// Notification preferences resource.
    #[must_use]
    pub fn notification_preferences(&self) -> NotificationPreferencesResource {
        NotificationPreferencesResource::new(self.clone())
    }

    /// Organizations resource.
    #[must_use]
    pub fn organizations(&self) -> Organizations {
        Organizations::new(self.clone())
    }

    /// Regions resource.
    #[must_use]
    pub fn regions(&self) -> Regions {
        Regions::new(self.clone())
    }

    /// Status resource.
    #[must_use]
    pub fn status(&self) -> Status {
        Status::new(self.clone())
    }

    /// Test emails resource.
    #[must_use]
    pub fn test_emails(&self) -> TestEmails {
        TestEmails::new(self.clone())
    }

    /// Automations resource.
    #[must_use]
    pub fn automations(&self) -> Automations {
        Automations::new(self.clone())
    }

    /// Events resource.
    #[must_use]
    pub fn events(&self) -> Events {
        Events::new(self.clone())
    }

    // ──────────────────────────────────────────────────────────────────
    // Internal HTTP helpers used by resource modules.
    // ──────────────────────────────────────────────────────────────────

    pub(crate) async fn request<R>(&self, req: RequestBuilder) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        let resp = self.send_with_retries(req).await?;
        let status = resp.status();

        if status == StatusCode::NO_CONTENT {
            // Caller asked for a typed body but the API returned none.
            // Use a JSON null and let the caller's type system reject it
            // if the type isn't nullable.
            return serde_json::from_value(Value::Null).map_err(Error::Decode);
        }

        let bytes = resp.bytes().await.map_err(Error::Network)?;
        serde_json::from_slice::<R>(&bytes).map_err(Error::Decode)
    }

    pub(crate) async fn request_unit(&self, req: RequestBuilder) -> Result<(), Error> {
        let resp = self.send_with_retries(req).await?;
        if resp.status().is_success() {
            // Drain the body.
            let _ = resp.bytes().await;
            return Ok(());
        }
        // Should never happen — send_with_retries surfaces errors first.
        Err(Self::error_from_status(resp.status(), Value::Null))
    }

    pub(crate) fn build<B>(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, String)],
        body: Option<&B>,
    ) -> RequestBuilder
    where
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.inner.base_url, path);
        let mut req = self.inner.http.request(method, url);
        if !query.is_empty() {
            req = req.query(query);
        }
        if let Some(b) = body {
            req = req.json(b);
        }
        req
    }

    async fn send_with_retries(&self, req: RequestBuilder) -> Result<reqwest::Response, Error> {
        let max_attempts = u32::from(self.inner.retries) + 1;
        let mut last_error: Option<Error> = None;

        for attempt in 0..max_attempts {
            if attempt > 0 {
                let delay = backoff(attempt - 1);
                tokio::time::sleep(delay).await;
            }

            let cloned = req
                .try_clone()
                .expect("request body is JSON / static so try_clone always succeeds");

            match cloned.send().await {
                Ok(resp) if resp.status().is_success() => return Ok(resp),
                Ok(resp) => {
                    let status = resp.status();
                    let bytes = resp.bytes().await.map_err(Error::Network)?;
                    let body: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
                    let err = Self::error_from_status(status, body);
                    if !err.is_retryable() {
                        return Err(err);
                    }
                    last_error = Some(err);
                }
                Err(e) => {
                    last_error = Some(Error::Network(e));
                }
            }
        }

        Err(last_error.expect("loop runs at least once"))
    }

    fn error_from_status(status: StatusCode, body: Value) -> Error {
        let err = body.get("error");
        let code = err
            .and_then(|e| e.get("code"))
            .and_then(Value::as_str)
            .unwrap_or("unknown_error")
            .to_string();
        let message = err
            .and_then(|e| e.get("message"))
            .and_then(Value::as_str)
            .unwrap_or_else(|| status.canonical_reason().unwrap_or("error"))
            .to_string();
        let details = err.and_then(|e| e.get("details")).cloned();

        match status.as_u16() {
            401 => Error::Authentication(message),
            404 => Error::NotFound(message),
            422 => Error::Validation {
                code,
                message,
                details,
            },
            429 => Error::RateLimit {
                code,
                message,
                retry_after: None,
            },
            s => Error::Api {
                status: s,
                code,
                message,
                details,
            },
        }
    }
}

fn backoff(attempt: u32) -> Duration {
    let ms = 200u64.saturating_mul(1u64 << attempt.min(5));
    Duration::from_millis(ms.min(5_000))
}
