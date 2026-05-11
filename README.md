# Sendry Rust SDK

The official Rust crate for the [Sendry](https://sendry.online) email API.

[![Crates.io](https://img.shields.io/crates/v/sendry.svg)](https://crates.io/crates/sendry)
[![docs.rs](https://docs.rs/sendry/badge.svg)](https://docs.rs/sendry)
[![License](https://img.shields.io/crates/l/sendry.svg)](LICENSE)

## Installation

```bash
cargo add sendry
```

Or add to `Cargo.toml`:

```toml
[dependencies]
sendry = "0.1"
tokio = { version = "1", features = ["full"] }
```

Requires Rust 1.75+. Async only — built on `reqwest` + `tokio`.

## Quick start

```rust
use sendry::{Sendry, SendEmail};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Sendry::new(std::env::var("SENDRY_API_KEY")?);

    let resp = client.emails().send(SendEmail {
        from:    "hello@yourdomain.com".into(),
        to:      vec!["user@example.com".into()],
        subject: "Welcome".into(),
        html:    Some("<p>Thanks for signing up.</p>".into()),
        text:    Some("Thanks for signing up.".into()),
        ..Default::default()
    }).await?;

    println!("sent: {}", resp.id);
    Ok(())
}
```

## Configuration

```rust
use std::time::Duration;

let client = Sendry::builder()
    .api_key(std::env::var("SENDRY_API_KEY")?)
    .base_url("https://api.sendry.online")
    .timeout(Duration::from_secs(30))
    .max_retries(2)
    .build()?;
```

## Resources

```rust
client.emails()      // send, send_batch, get, list
client.domains()     // create, verify, get, list, delete
client.templates()   // create, get, list, delete
client.contacts()    // upsert, get, list, delete
client.audiences()   // create, get, list, delete
client.campaigns()   // create, get, list, send
client.webhooks()    // create, get, list, delete
```

## Error handling

```rust
use sendry::Error;

match client.emails().send(req).await {
    Ok(resp) => println!("sent: {}", resp.id),
    Err(Error::Authentication(msg)) => eprintln!("auth: {msg}"),
    Err(Error::Validation { details, .. }) => eprintln!("validation: {details:?}"),
    Err(Error::RateLimit { retry_after, .. }) => {
        tokio::time::sleep(retry_after.unwrap_or_default()).await;
    }
    Err(Error::Api { status, message, .. }) => eprintln!("API {status}: {message}"),
    Err(Error::Network(e)) => eprintln!("network: {e}"),
    Err(e) => eprintln!("{e}"),
}
```

`Error::is_retryable()` returns `true` for `Network`, `RateLimit`, and `Api`
errors with 5xx status.

## Webhook signature verification

```rust
use sendry::verify_webhook_signature;

let valid = verify_webhook_signature(
    request_body.as_bytes(),
    request.headers().get("x-sendry-signature").unwrap().to_str()?,
    &webhook_secret,
);

if !valid {
    return Err(StatusCode::UNAUTHORIZED.into());
}
```

Constant-time comparison via the [`subtle`](https://crates.io/crates/subtle) crate.

## Feature flags

| Flag | Default | Description |
|---|---|---|
| `native-tls` | yes | Use the system TLS stack |
| `rustls`     | no  | Use pure-Rust TLS (no system deps) |

```toml
[dependencies]
sendry = { version = "0.1", default-features = false, features = ["rustls"] }
```

## License

MIT — see [LICENSE](LICENSE).
