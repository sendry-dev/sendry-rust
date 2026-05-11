# Changelog

All notable changes to the `sendry` crate are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-05-11

### Added

- Initial public release.
- `Sendry` async client (built on `reqwest` + `tokio`).
- `SendryBuilder` for advanced configuration (timeout, retries, base URL).
- Resource handles: `Emails`, `Domains`, `Templates`, `Contacts`,
  `Audiences`, `Campaigns`, `Webhooks`.
- Strongly typed `Error` enum: `Authentication`, `NotFound`, `Validation`,
  `RateLimit`, `Api`, `Network`, `Decode`. `Error::is_retryable` helper.
- `verify_webhook_signature` — constant-time HMAC-SHA256 (via `subtle`).
- Automatic retries on 5xx / network errors with exponential backoff.
- Feature flags: `native-tls` (default), `rustls`.
