//! Error types returned by every fallible client call.

use std::time::Duration;

use thiserror::Error;

/// All errors this crate can return.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP 401 — the API key was missing or invalid.
    #[error("authentication failed: {0}")]
    Authentication(String),

    /// HTTP 404 — the requested resource doesn't exist.
    #[error("not found: {0}")]
    NotFound(String),

    /// HTTP 422 — request body or parameters failed validation.
    #[error("validation failed: {message}")]
    Validation {
        /// Server-side error code, e.g. `validation_error`.
        code: String,
        /// Human-readable message.
        message: String,
        /// Optional field-level error details from the API.
        details: Option<serde_json::Value>,
    },

    /// HTTP 429 — rate limit exceeded.
    #[error("rate limited: {message} (retry after {retry_after:?})")]
    RateLimit {
        /// Server-side error code.
        code: String,
        /// Human-readable message.
        message: String,
        /// Suggested wait time from the `Retry-After` header.
        retry_after: Option<Duration>,
    },

    /// Any other 4xx / 5xx API error.
    #[error("api error {status}: {message} (code={code})")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Server-side error code.
        code: String,
        /// Human-readable message.
        message: String,
        /// Optional details from the API.
        details: Option<serde_json::Value>,
    },

    /// Network-level failure — DNS, connection, timeout, etc. No HTTP response.
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON decode failure on a response body.
    #[error("decode error: {0}")]
    Decode(#[from] serde_json::Error),
}

impl Error {
    /// Returns `true` if a retry could plausibly succeed.
    pub fn is_retryable(&self) -> bool {
        match self {
            Error::Network(_) | Error::RateLimit { .. } => true,
            Error::Api { status, .. } => *status >= 500,
            _ => false,
        }
    }
}
