//! Integration tests for the read-only monitoring endpoints:
//! `overview()`, `cluster_name()`, `list_nodes()`, `get_node()`.

mod common;

use rabbitmqadmin_sdk::{Client, Error};
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn overview_deserializes_realistic_payload() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/overview"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "management_version": "3.12.4",
            "rabbitmq_version": "3.12.4",
            "cluster_name": "rabbit@host",
            "erlang_version": "25.3.2.4",
            "object_totals": {
                "channels": 3,
                "connections": 2,
                "consumers": 5,
                "exchanges": 8,
                "queues": 12
            },
            "queue_totals": {
                "messages": 42,
                "messages_ready": 40,
                "messages_unacknowledged": 2
            },
            "message_stats": {
                "publish": 1234,
                "deliver_get": 5678,
                "ack": 5600,
                "redeliver": 3,
                "confirm": 1234,
                "publish_details": {"rate": 12.5}
            },
            "listeners": [{"node": "rabbit@host", "protocol": "amqp", "port": 5672}],
            "contexts": [{"node": "rabbit@host", "description": "RabbitMQ Management", "port": 15672}]
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let ov = c.overview().await.unwrap();

    assert_eq!(ov.rabbitmq_version.as_deref(), Some("3.12.4"));
    assert_eq!(ov.cluster_name.as_deref(), Some("rabbit@host"));
    assert_eq!(ov.object_totals.queues, 12);
    assert_eq!(ov.queue_totals.messages_ready, 40);
    let stats = ov.message_stats.expect("message_stats present");
    assert_eq!(stats.publish, Some(1234));
    assert_eq!(stats.redeliver, Some(3));
}

#[tokio::test]
async fn cluster_name_returns_unwrapped_name() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/cluster-name"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "rabbit@host"
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let name = c.cluster_name().await.unwrap();
    assert_eq!(name, "rabbit@host");
}

#[tokio::test]
async fn list_nodes_returns_all_nodes() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/nodes"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "rabbit@host1",
                "type": "disc",
                "running": true,
                "mem_used": 71_000_000_u64,
                "mem_limit": 6_800_000_000_u64,
                "mem_alarm": false,
                "disk_free": 50_000_000_000_u64,
                "disk_free_limit": 50_000_000_u64,
                "disk_free_alarm": false,
                "fd_used": 30,
                "fd_total": 1_048_576,
                "sockets_used": 5,
                "sockets_total": 943_629,
                "proc_used": 350,
                "proc_total": 1_048_576,
                "run_queue": 0,
                "uptime": 3_600_000,
                "os_pid": "12345",
                "rates_mode": "basic"
            },
            {
                "name": "rabbit@host2",
                "type": "disc",
                "running": true
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let nodes = c.list_nodes().await.unwrap();
    assert_eq!(nodes.len(), 2);
    assert_eq!(nodes[0].name, "rabbit@host1");
    assert_eq!(nodes[1].name, "rabbit@host2");
    // The wire key is literally "type" — serde must rename it to `node_type`.
    assert_eq!(nodes[0].node_type.as_deref(), Some("disc"));
    assert_eq!(nodes[1].node_type.as_deref(), Some("disc"));
    assert_eq!(nodes[0].mem_used, Some(71_000_000));
    assert_eq!(nodes[0].fd_total, Some(1_048_576));
    assert_eq!(nodes[1].mem_used, None);
}

/// Regression test for the `Node.node_type` deserialization bug: the
/// Management API returns this value under the JSON key `"type"`, not
/// `"node_type"`. Without `#[serde(rename = "type")]` this field silently
/// stayed `None` against every real broker.
#[tokio::test]
async fn get_node_populates_node_type_from_real_world_type_key() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/nodes/rabbit%40host1"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "rabbit@host1",
            "type": "disc",
            "running": true
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let node = c.get_node("rabbit@host1").await.unwrap();
    assert_eq!(node.node_type.as_deref(), Some("disc"));
}

#[tokio::test]
async fn get_node_encodes_name_in_path() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        // '@' must be percent-encoded as %40 in the path segment.
        .and(path("/api/nodes/rabbit%40host"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "rabbit@host",
            "type": "disc",
            "running": true,
            "mem_used": 71_000_000_u64,
            "disk_free_alarm": false,
            "run_queue": 0,
            "uptime": 86_400_000
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let node = c.get_node("rabbit@host").await.unwrap();
    assert_eq!(node.name, "rabbit@host");
    assert_eq!(node.node_type.as_deref(), Some("disc"));
    assert_eq!(node.running, Some(true));
    assert_eq!(node.uptime, Some(86_400_000));
}

#[tokio::test]
async fn get_node_not_found_maps_to_error_not_found() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/nodes/missing"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.get_node("missing").await.unwrap_err();
    match err {
        Error::NotFound(msg) => {
            assert!(
                msg.contains("node 'missing'"),
                "expected context in message, got: {msg}"
            );
            assert!(msg.contains("Not Found"), "body preserved: {msg}");
        }
        other => panic!("expected Error::NotFound, got {other:?}"),
    }
}
