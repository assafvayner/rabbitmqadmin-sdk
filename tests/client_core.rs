//! Integration tests for the core `Client`: auth header, URL handling,
//! JSON deserialization, and error mapping.

mod common;

use rabbitmqadmin_sdk::{Client, Error};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn whoami_deserializes_json_and_sends_auth() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/whoami"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "guest",
            "tags": "administrator"
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let who = c.whoami().await.unwrap();
    assert_eq!(who.name, "guest");
    assert_eq!(who.tags, "administrator");
}

#[tokio::test]
async fn not_found_maps_to_error_not_found() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/whoami"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.whoami().await.unwrap_err();
    match err {
        Error::NotFound(body) => assert!(body.contains("Not Found")),
        other => panic!("expected Error::NotFound, got {other:?}"),
    }
}

#[tokio::test]
async fn unauthorized_maps_to_error_api() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/whoami"))
        .respond_with(ResponseTemplate::new(401).set_body_string("not authorised"))
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "wrong-password").unwrap();
    let err = c.whoami().await.unwrap_err();
    match err {
        Error::Api { status, reason } => {
            assert_eq!(status, 401);
            assert_eq!(reason, "not authorised");
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn base_url_normalization_avoids_double_api() {
    for suffix in ["", "/", "/api", "/api/"] {
        let srv = common::server().await;
        Mock::given(method("GET"))
            // Must hit /api/whoami exactly — never /api/api/whoami.
            .and(path("/api/whoami"))
            .and(common::guest_auth())
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "guest",
                "tags": "administrator"
            })))
            .expect(1)
            .mount(&srv)
            .await;

        let c = Client::new(&format!("{}{suffix}", srv.uri()), "guest", "guest").unwrap();
        let who = c.whoami().await.unwrap();
        assert_eq!(who.name, "guest", "suffix {suffix:?}");
    }
}
