use sendry::{Error, SendEmail, Sendry};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn client(server: &MockServer) -> Sendry {
    Sendry::builder()
        .api_key("sn_live_test")
        .base_url(server.uri())
        .max_retries(0)
        .build()
        .unwrap()
}

#[tokio::test]
async fn sends_email_with_auth_header() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/emails"))
        .and(header("Authorization", "Bearer sn_live_test"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({ "id": "em_123", "status": "queued" })),
        )
        .mount(&server)
        .await;

    let client = client(&server).await;
    let resp = client
        .emails()
        .send(SendEmail {
            from: "a@b.com".into(),
            to: vec!["c@d.com".into()],
            subject: "hi".into(),
            html: Some("<p>hi</p>".into()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(resp.id, "em_123");
    assert_eq!(resp.status, "queued");
}

#[tokio::test]
async fn maps_401_to_authentication_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/emails"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "error": { "code": "invalid_api_key", "message": "Bad key" }
        })))
        .mount(&server)
        .await;

    let client = client(&server).await;
    let err = client
        .emails()
        .send(SendEmail {
            from: "a@b.com".into(),
            to: vec!["c@d.com".into()],
            subject: "x".into(),
            html: Some("y".into()),
            ..Default::default()
        })
        .await
        .unwrap_err();

    assert!(matches!(err, Error::Authentication(_)));
}

#[tokio::test]
async fn maps_422_to_validation_error_with_details() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/emails"))
        .respond_with(ResponseTemplate::new(422).set_body_json(json!({
            "error": {
                "code": "validation_error",
                "message": "bad",
                "details": { "from": "required" }
            }
        })))
        .mount(&server)
        .await;

    let client = client(&server).await;
    let err = client
        .emails()
        .send(SendEmail::default())
        .await
        .unwrap_err();

    match err {
        Error::Validation { details, .. } => {
            assert_eq!(
                details.unwrap().get("from").and_then(|v| v.as_str()),
                Some("required"),
            );
        }
        other => panic!("expected Validation, got {other:?}"),
    }
}
