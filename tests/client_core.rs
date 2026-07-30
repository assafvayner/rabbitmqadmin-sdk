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
        Error::NotFound(msg) => {
            assert!(
                msg.contains("whoami"),
                "expected 'whoami' context in message, got: {msg}"
            );
            assert!(msg.contains("Not Found"), "body preserved: {msg}");
        }
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

#[test]
fn base_url_without_scheme_is_rejected() {
    match Client::new("localhost:15672", "guest", "guest") {
        Err(Error::InvalidUrl(msg)) => {
            assert!(
                msg.contains("localhost:15672"),
                "message mentions the offending input: {msg}"
            );
            assert!(
                msg.contains("scheme"),
                "message mentions the scheme problem: {msg}"
            );
        }
        Err(other) => panic!("expected Error::InvalidUrl, got {other}"),
        Ok(_) => panic!("expected Error::InvalidUrl, but the client built successfully"),
    }
}

#[test]
fn base_url_with_non_http_scheme_is_rejected() {
    match Client::new("ftp://localhost:15672", "guest", "guest") {
        Err(Error::InvalidUrl(msg)) => {
            assert!(
                msg.contains("scheme"),
                "message mentions the scheme problem: {msg}"
            );
        }
        Err(other) => panic!("expected Error::InvalidUrl, got {other}"),
        Ok(_) => panic!("expected Error::InvalidUrl, but the client built successfully"),
    }
}

#[tokio::test]
async fn malformed_json_maps_to_error_deserialize() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/whoami"))
        .and(common::guest_auth())
        // WhoAmI.name must be a string; a number fails deserialization.
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": 42,
            "tags": "administrator"
        })))
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.whoami().await.unwrap_err();
    match err {
        Error::Deserialize { source, body } => {
            assert!(!source.to_string().is_empty());
            assert!(
                body.contains("\"name\":42"),
                "body kept for debugging: {body}"
            );
        }
        other => panic!("expected Error::Deserialize, got {other:?}"),
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
