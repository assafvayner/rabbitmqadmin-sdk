//! Integration tests for the cluster-level endpoints:
//! `export_definitions()`, `export_definitions_in_vhost()`,
//! `import_definitions()`, the six `health_check_*()` methods,
//! `list_alarms()`, and `rebalance_queues()`.

mod common;

use rabbitmqadmin_sdk::types::definitions::Definitions;
use rabbitmqadmin_sdk::{Client, Error};
use wiremock::matchers::{body_string_contains, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn export_definitions_deserializes() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/definitions"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "rabbit_version": "3.13.1",
            "users": [{
                "name": "guest",
                "password_hash": "BASE64HASH==",
                "hashing_algorithm": "rabbit_password_hashing_sha256",
                "tags": ["administrator"]
            }],
            "vhosts": [{"name": "/"}],
            "permissions": [{
                "user": "guest",
                "vhost": "/",
                "configure": ".*",
                "write": ".*",
                "read": ".*"
            }],
            "parameters": [],
            "global_parameters": [{"name": "cluster_name", "value": "rabbit@host"}],
            "policies": [{
                "name": "ha",
                "vhost": "/",
                "pattern": ".*",
                "apply-to": "queues",
                "definition": {"ha-mode": "all"},
                "priority": 0
            }],
            "queues": [{"name": "q1", "vhost": "/", "durable": true}],
            "exchanges": [{"name": "amq.direct", "vhost": "/", "type": "direct"}],
            "bindings": [{
                "source": "amq.direct",
                "vhost": "/",
                "destination": "q1",
                "destination_type": "queue",
                "routing_key": "rk",
                "arguments": {}
            }]
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let defs = c.export_definitions().await.unwrap();

    assert_eq!(defs.rabbit_version.as_deref(), Some("3.13.1"));
    // Typed where the SDK owns the schema:
    assert_eq!(defs.permissions[0].user, "guest");
    assert_eq!(defs.policies[0].apply_to, "queues");
    assert_eq!(defs.bindings[0].destination, "q1");
    // Value where export bodies contain server-generated fields that must
    // round-trip verbatim on import:
    assert_eq!(defs.users[0]["password_hash"], "BASE64HASH==");
    assert_eq!(defs.queues[0]["name"], "q1");
    assert_eq!(defs.exchanges[0]["type"], "direct");
}

#[tokio::test]
async fn export_definitions_in_vhost_returns_value() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/definitions/%2F"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "rabbit_version": "3.13.1",
            "vhosts": [{"name": "/"}],
            "queues": [{"name": "q1"}]
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let v = c.export_definitions_in_vhost("/").await.unwrap();

    assert_eq!(v["queues"][0]["name"], "q1");
}

#[tokio::test]
async fn import_definitions_posts_body() {
    let srv = common::server().await;
    Mock::given(method("POST"))
        .and(path("/api/definitions"))
        .and(body_string_contains("\"rabbit_version\""))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let defs: Definitions = serde_json::from_value(serde_json::json!({
        "rabbit_version": "3.13.1"
    }))
    .unwrap();
    c.import_definitions(&defs).await.unwrap();
}

#[tokio::test]
async fn health_check_alarms_ok() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/health/checks/alarms"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "ok"
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let h = c.health_check_alarms().await.unwrap();

    assert_eq!(h.status, "ok");
    assert!(h.reason.is_none());
}

#[tokio::test]
async fn health_check_alarms_failing_maps_to_api_503() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/health/checks/alarms"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(503).set_body_json(serde_json::json!({
            "status": "failed",
            "reason": "memory alarm on node rabbit@host",
            "alarms": [{"node": "rabbit@host", "resource": "memory"}]
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.health_check_alarms().await.unwrap_err();

    match err {
        Error::Api { status, reason } => {
            assert_eq!(status, 503);
            assert!(
                reason.contains("failed"),
                "error body should carry the failure JSON, got: {reason}"
            );
        }
        other => panic!("expected Error::Api(503), got {other:?}"),
    }
}

#[tokio::test]
async fn health_check_port_listener_hits_port_path() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/health/checks/port-listener/5672"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "ok"
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let h = c.health_check_port_listener(5672).await.unwrap();

    assert_eq!(h.status, "ok");
}

#[tokio::test]
async fn health_check_protocol_listener_encodes() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/health/checks/protocol-listener/amqp"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "ok"
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let h = c.health_check_protocol_listener("amqp").await.unwrap();

    assert_eq!(h.status, "ok");
}

#[tokio::test]
async fn health_check_quorum_critical_virtual_hosts_and_local_alarms() {
    let srv = common::server().await;
    for p in [
        "/api/health/checks/node-is-quorum-critical",
        "/api/health/checks/virtual-hosts",
        "/api/health/checks/local-alarms",
    ] {
        Mock::given(method("GET"))
            .and(path(p))
            .and(common::guest_auth())
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "ok"
            })))
            .expect(1)
            .mount(&srv)
            .await;
    }

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    assert_eq!(
        c.health_check_node_is_quorum_critical()
            .await
            .unwrap()
            .status,
        "ok"
    );
    assert_eq!(c.health_check_virtual_hosts().await.unwrap().status, "ok");
    assert_eq!(c.health_check_local_alarms().await.unwrap().status, "ok");
}

#[tokio::test]
async fn list_alarms_returns_values() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/alarms"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {"node": "rabbit@host", "resource": "memory"}
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let alarms = c.list_alarms().await.unwrap();

    assert_eq!(alarms.len(), 1);
    assert_eq!(alarms[0]["resource"], "memory");
}

#[tokio::test]
async fn rebalance_queues_posts_empty() {
    let srv = common::server().await;
    Mock::given(method("POST"))
        .and(path("/api/rebalance/queues"))
        .and(body_string_contains("{}"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.rebalance_queues().await.unwrap();
}
