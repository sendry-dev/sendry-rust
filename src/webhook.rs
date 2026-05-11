//! Webhook signature verification.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

/// Verify a Sendry webhook signature using HMAC-SHA256 with constant-time
/// comparison.
///
/// Returns `true` if the signature is valid.
///
/// # Example
///
/// ```no_run
/// use sendry::verify_webhook_signature;
///
/// let valid = verify_webhook_signature(
///     b"raw request body",
///     "5dcce8c8e91d28b80c0b7eb6c9c4f4a0c5e0...", // x-sendry-signature header
///     "whsec_your_webhook_secret",
/// );
/// assert!(!valid); // dummy signature
/// ```
pub fn verify_webhook_signature(payload: &[u8], signature: &str, secret: &str) -> bool {
    let sig_hex = signature.strip_prefix("sha256=").unwrap_or(signature);
    let Ok(sig_bytes) = hex::decode(sig_hex) else {
        return false;
    };

    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(payload);
    let expected = mac.finalize().into_bytes();

    expected.as_slice().ct_eq(sig_bytes.as_slice()).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hmac::Mac;

    fn sign(body: &[u8], secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body);
        hex::encode(mac.finalize().into_bytes())
    }

    #[test]
    fn valid_signature() {
        let body = b"hello";
        let sig = sign(body, "secret");
        assert!(verify_webhook_signature(body, &sig, "secret"));
        assert!(verify_webhook_signature(
            body,
            &format!("sha256={sig}"),
            "secret"
        ));
    }

    #[test]
    fn rejects_bad_signature() {
        assert!(!verify_webhook_signature(b"hello", "deadbeef", "secret"));
        assert!(!verify_webhook_signature(b"hello", "not-hex", "secret"));
    }

    #[test]
    fn rejects_wrong_secret() {
        let sig = sign(b"hello", "secret");
        assert!(!verify_webhook_signature(b"hello", &sig, "other"));
    }
}
