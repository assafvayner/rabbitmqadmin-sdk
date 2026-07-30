//! Integration tests for the policies, operator policies, and runtime
//! parameters endpoints: `list_policies()`, `list_policies_in_vhost()`,
//! `get_policy()`, `set_policy()`, `delete_policy()`,
//! `list_operator_policies()`, `list_operator_policies_in_vhost()`,
//! `get_operator_policy()`, `set_operator_policy()`,
//! `delete_operator_policy()`, `list_parameters()`,
//! `list_parameters_in_vhost()`, `get_parameter()`, `set_parameter()`,
//! `delete_parameter()`.

mod common;

use rabbitmqadmin_sdk::{Client, Error};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn list_policies_returns_all() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/policies"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "ha-all",
                "vhost": "/",
                "pattern": "^ha\\.",
                "apply-to": "queues",
                "definition": {"ha-mode": "all"},
                "priority": 0
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let policies = c.list_policies().await.unwrap();

    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].name, "ha-all");
    assert_eq!(policies[0].vhost, "/");
    assert_eq!(policies[0].pattern, "^ha\\.");
    // The wire key is literally "apply-to" — serde must rename it.
    assert_eq!(policies[0].apply_to, "queues");
    assert_eq!(
        policies[0].definition,
        serde_json::json!({"ha-mode": "all"})
    );
    assert_eq!(policies[0].priority, 0);
}

#[tokio::test]
async fn list_policies_in_vhost() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/policies/%2F"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "ha-all",
                "vhost": "/",
                "pattern": "^ha\\.",
                "apply-to": "queues",
                "definition": {"ha-mode": "all"},
                "priority": 0
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let policies = c.list_policies_in_vhost("/").await.unwrap();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].name, "ha-all");
    assert_eq!(policies[0].apply_to, "queues");
}

#[tokio::test]
async fn get_policy_404_ctx() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/policies/%2F/ha-all"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.get_policy("/", "ha-all").await.unwrap_err();
    match err {
        Error::NotFound(msg) => {
            assert!(
                msg.contains("policy 'ha-all'"),
                "expected policy context in message, got: {msg}"
            );
            assert!(
                msg.contains("in vhost '/'"),
                "expected vhost context in message, got: {msg}"
            );
        }
        other => panic!("expected Error::NotFound, got {other:?}"),
    }
}

#[tokio::test]
async fn set_policy_puts_body() {
    let srv = common::server().await;
    Mock::given(method("PUT"))
        .and(path("/api/policies/%2F/ha-all"))
        .and(body_json(serde_json::json!({
            "pattern": "^ha\\.",
            "definition": {"ha-mode": "all"},
            "priority": 0,
            "apply-to": "queues"
        })))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(201).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.set_policy(
        "/",
        "ha-all",
        "^ha\\.",
        serde_json::json!({"ha-mode": "all"}),
        0,
        "queues",
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn delete_policy_encoded() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        .and(path("/api/policies/%2F/ha-all"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.delete_policy("/", "ha-all").await.unwrap();
}

#[tokio::test]
async fn list_operator_policies() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/operator-policies"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "qlimit",
                "vhost": "/",
                "pattern": "^q\\.",
                "apply-to": "queues",
                "definition": {"max-length": 1000},
                "priority": 10
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let policies = c.list_operator_policies().await.unwrap();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].name, "qlimit");
    assert_eq!(policies[0].priority, 10);
    assert_eq!(
        policies[0].definition,
        serde_json::json!({"max-length": 1000})
    );
}

#[tokio::test]
async fn list_operator_policies_in_vhost() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/operator-policies/%2F"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "qlimit",
                "vhost": "/",
                "pattern": "^q\\.",
                "apply-to": "queues",
                "definition": {"max-length": 1000},
                "priority": 10
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let policies = c.list_operator_policies_in_vhost("/").await.unwrap();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].name, "qlimit");
}

#[tokio::test]
async fn set_operator_policy_puts() {
    let srv = common::server().await;
    Mock::given(method("PUT"))
        .and(path("/api/operator-policies/%2F/qlimit"))
        .and(body_json(serde_json::json!({
            "pattern": "^q\\.",
            "definition": {"max-length": 1000},
            "priority": 10,
            "apply-to": "queues"
        })))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(201).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.set_operator_policy(
        "/",
        "qlimit",
        "^q\\.",
        serde_json::json!({"max-length": 1000}),
        10,
        "queues",
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn delete_operator_policy() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        .and(path("/api/operator-policies/%2F/qlimit"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.delete_operator_policy("/", "qlimit").await.unwrap();
}

#[tokio::test]
async fn list_parameters_all_and_by_component() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/parameters"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "component": "federation-upstream",
                "vhost": "/",
                "name": "up1",
                "value": {"uri": "amqp://remote"}
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/parameters/federation-upstream"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "component": "federation-upstream",
                "vhost": "/",
                "name": "up1",
                "value": {"uri": "amqp://remote"}
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();

    let all = c.list_parameters(None).await.unwrap();
    assert_eq!(all.len(), 1);
    assert_eq!(all[0]["name"], "up1");

    let fed = c
        .list_parameters(Some("federation-upstream"))
        .await
        .unwrap();
    assert_eq!(fed.len(), 1);
    assert_eq!(fed[0]["component"], "federation-upstream");
    assert_eq!(fed[0]["value"]["uri"], "amqp://remote");
}

#[tokio::test]
async fn set_parameter_wraps_value() {
    let srv = common::server().await;
    Mock::given(method("PUT"))
        .and(path("/api/parameters/federation-upstream/%2F/up1"))
        .and(body_json(serde_json::json!({
            "value": {"uri": "amqp://remote"}
        })))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(201).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.set_parameter(
        "federation-upstream",
        "/",
        "up1",
        serde_json::json!({"uri": "amqp://remote"}),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn get_parameter_404_ctx() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/parameters/federation-upstream/%2F/up1"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c
        .get_parameter("federation-upstream", "/", "up1")
        .await
        .unwrap_err();
    match err {
        Error::NotFound(msg) => {
            assert!(
                msg.contains("parameter 'up1'"),
                "expected parameter context in message, got: {msg}"
            );
            assert!(
                msg.contains("component 'federation-upstream'"),
                "expected component context in message, got: {msg}"
            );
        }
        other => panic!("expected Error::NotFound, got {other:?}"),
    }
}

#[tokio::test]
async fn delete_parameter_encoded() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        .and(path("/api/parameters/federation-upstream/%2F/up1"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.delete_parameter("federation-upstream", "/", "up1")
        .await
        .unwrap();
}
