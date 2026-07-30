//! Integration tests for the connections, channels, and consumers
//! endpoints: `list_connections()`, `list_connections_paged()`,
//! `get_connection()`, `close_connection()`, `list_connection_channels()`,
//! `list_channels()`, `get_channel()`, `list_consumers()`,
//! `list_consumers_in_vhost()`.

mod common;

use rabbitmqadmin_sdk::{Client, Error, PaginationQuery};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

/// A realistic connection name as returned by the Management API.
const CONN_NAME: &str = "127.0.0.1:52341 -> 127.0.0.1:5672";
/// `encode_segment(CONN_NAME)` — space → %20, ':' → %3A, '>' → %3E.
const CONN_NAME_ENC: &str = "127.0.0.1%3A52341%20-%3E%20127.0.0.1%3A5672";

#[tokio::test]
async fn list_connections_returns_all() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/connections"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": CONN_NAME,
                "vhost": "/",
                "user": "guest",
                "state": "running",
                "protocol": "AMQP 0-9-1",
                "host": "127.0.0.1",
                "port": 5672,
                "peer_host": "127.0.0.1",
                "peer_port": 52341,
                "channels": 1,
                "connected_at": 1753891200000_u64,
                "recv_oct": 1024,
                "send_oct": 2048
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let conns = c.list_connections().await.unwrap();

    assert_eq!(conns.len(), 1);
    assert_eq!(conns[0].name, CONN_NAME);
    assert_eq!(conns[0].peer_port, Some(52341));
}

#[tokio::test]
async fn list_connections_paged_sends_params() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/connections"))
        .and(query_param("pagination", "true"))
        .and(query_param("page", "1"))
        .and(query_param("page_size", "10"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "filtered_count": 1,
            "item_count": 1,
            "items": [{
                "name": CONN_NAME,
                "vhost": "/",
                "user": "guest",
                "state": "running",
                "peer_port": 52341
            }],
            "page": 1,
            "page_count": 1,
            "page_size": 10,
            "total_count": 1
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let q = PaginationQuery {
        page: Some(1),
        page_size: Some(10),
        ..Default::default()
    };
    let page = c.list_connections_paged(&q).await.unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, CONN_NAME);
    assert_eq!(page.total_count, 1);
    assert_eq!(page.page, 1);
    assert_eq!(page.page_size, 10);
}

#[tokio::test]
async fn get_connection_encodes_complex_name() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path(format!("/api/connections/{CONN_NAME_ENC}")))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": CONN_NAME,
            "vhost": "/",
            "user": "guest",
            "state": "running",
            "peer_host": "127.0.0.1",
            "peer_port": 52341
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let conn = c.get_connection(CONN_NAME).await.unwrap();

    assert_eq!(conn.name, CONN_NAME);
    assert_eq!(conn.state.as_deref(), Some("running"));
}

#[tokio::test]
async fn close_connection_without_reason() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        .and(path(format!("/api/connections/{CONN_NAME_ENC}")))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.close_connection(CONN_NAME, None).await.unwrap();
}

#[tokio::test]
async fn close_connection_with_reason_sets_header() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        .and(path(format!("/api/connections/{CONN_NAME_ENC}")))
        .and(header("x-reason", "going away"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.close_connection(CONN_NAME, Some("going away"))
        .await
        .unwrap();
}

#[tokio::test]
async fn close_connection_404_ctx() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        .and(path(format!("/api/connections/{CONN_NAME_ENC}")))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.close_connection(CONN_NAME, None).await.unwrap_err();
    match err {
        Error::NotFound(msg) => {
            assert!(
                msg.contains("connection '"),
                "expected connection context in message, got: {msg}"
            );
        }
        other => panic!("expected Error::NotFound, got {other:?}"),
    }
}

#[tokio::test]
async fn list_connection_channels_returns_all() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path(format!("/api/connections/{CONN_NAME_ENC}/channels")))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "127.0.0.1:52341 -> 127.0.0.1:5672 (1)",
                "vhost": "/",
                "user": "guest",
                "number": 1,
                "node": "rabbit@host",
                "state": "running",
                "consumer_count": 2,
                "prefetch_count": 10,
                "messages_unacknowledged": 0,
                "messages_unconfirmed": 0
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let channels = c.list_connection_channels(CONN_NAME).await.unwrap();

    assert_eq!(channels.len(), 1);
    assert_eq!(channels[0].number, Some(1));
    assert_eq!(channels[0].consumer_count, Some(2));
}

#[tokio::test]
async fn list_channels_returns_all() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/channels"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "127.0.0.1:52341 -> 127.0.0.1:5672 (1)",
                "vhost": "/",
                "user": "guest",
                "number": 1,
                "node": "rabbit@host",
                "state": "running",
                "consumer_count": 1,
                "prefetch_count": 0,
                "messages_unacknowledged": 3,
                "messages_unconfirmed": 0,
                "message_stats": {"publish": 42, "ack": 39}
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let channels = c.list_channels().await.unwrap();

    assert_eq!(channels.len(), 1);
    assert_eq!(channels[0].messages_unacknowledged, Some(3));
    let stats = channels[0].message_stats.as_ref().expect("message_stats");
    assert_eq!(stats.publish, Some(42));
}

#[tokio::test]
async fn get_channel_404_ctx() {
    let srv = common::server().await;
    let ch_name = "127.0.0.1:52341 -> 127.0.0.1:5672 (1)";
    let ch_name_enc = "127.0.0.1%3A52341%20-%3E%20127.0.0.1%3A5672%20%281%29";
    Mock::given(method("GET"))
        .and(path(format!("/api/channels/{ch_name_enc}")))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.get_channel(ch_name).await.unwrap_err();
    match err {
        Error::NotFound(msg) => {
            assert!(
                msg.contains("channel '"),
                "expected channel context in message, got: {msg}"
            );
        }
        other => panic!("expected Error::NotFound, got {other:?}"),
    }
}

#[tokio::test]
async fn list_consumers_returns_all() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/consumers"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "consumer_tag": "amq.ctag-xyz",
                "queue": {"name": "jobs", "vhost": "/"},
                "channel_details": {"name": "127.0.0.1:52341 -> 127.0.0.1:5672 (1)"},
                "ack_required": true,
                "prefetch_count": 10,
                "active": true,
                "arguments": {}
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let consumers = c.list_consumers().await.unwrap();

    assert_eq!(consumers.len(), 1);
    assert_eq!(consumers[0].consumer_tag, "amq.ctag-xyz");
    assert_eq!(consumers[0].ack_required, Some(true));
}

#[tokio::test]
async fn list_consumers_in_vhost_encodes() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        // The default vhost "/" must be percent-encoded as %2F.
        .and(path("/api/consumers/%2F"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "consumer_tag": "amq.ctag-xyz",
                "queue": {"name": "jobs", "vhost": "/"},
                "ack_required": false,
                "active": false
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let consumers = c.list_consumers_in_vhost("/").await.unwrap();

    assert_eq!(consumers.len(), 1);
    assert_eq!(consumers[0].consumer_tag, "amq.ctag-xyz");
    assert_eq!(consumers[0].ack_required, Some(false));
}
