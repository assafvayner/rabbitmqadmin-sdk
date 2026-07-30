//! Integration tests for the queues endpoints:
//! `list_queues()`, `list_queues_paged()`, `list_queues_in_vhost()`,
//! `get_queue()`, `declare_queue()`, `delete_queue()`, `purge_queue()`,
//! `queue_action()`, `get_messages()`.

mod common;

use rabbitmqadmin_sdk::types::queue::{AckMode, PayloadEncoding, QueueAction, QueueDeclareOptions};
use rabbitmqadmin_sdk::{Client, Error, PaginationQuery};
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn list_queues_returns_all_queues() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/queues"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "q1",
                "vhost": "/",
                "durable": true,
                "auto_delete": false,
                "state": "running",
                "messages": 42,
                "messages_ready": 40,
                "messages_unacknowledged": 2,
                "consumers": 3,
                "memory": 131_072
            },
            {
                "name": "q2",
                "vhost": "vh",
                "durable": false,
                "auto_delete": true,
                "exclusive": true,
                "arguments": {"x-max-length": 10},
                "message_stats": {"publish": 7, "deliver_get": 5, "ack": 5}
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let queues = c.list_queues().await.unwrap();

    assert_eq!(queues.len(), 2);
    assert_eq!(queues[0].name, "q1");
    assert_eq!(queues[0].vhost, "/");
    assert!(queues[0].durable);
    assert!(!queues[0].auto_delete);
    assert_eq!(queues[0].state.as_deref(), Some("running"));
    assert_eq!(queues[0].messages, Some(42));
    assert_eq!(queues[0].consumers, Some(3));

    assert_eq!(queues[1].name, "q2");
    assert!(queues[1].exclusive);
    assert_eq!(queues[1].messages, None);
    let stats = queues[1].message_stats.as_ref().expect("message_stats");
    assert_eq!(stats.publish, Some(7));
}

#[tokio::test]
async fn list_queues_paged_sends_pagination_params() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/queues"))
        .and(query_param("pagination", "true"))
        .and(query_param("page", "2"))
        .and(query_param("page_size", "50"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "filtered_count": 51,
            "item_count": 1,
            "items": [{
                "name": "q51",
                "vhost": "/",
                "durable": true,
                "auto_delete": false,
                "messages": 0
            }],
            "page": 2,
            "page_count": 2,
            "page_size": 50,
            "total_count": 51
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let q = PaginationQuery {
        page: Some(2),
        page_size: Some(50),
        ..Default::default()
    };
    let page = c.list_queues_paged(&q).await.unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, "q51");
    assert_eq!(page.total_count, 51);
    assert_eq!(page.filtered_count, 51);
    assert_eq!(page.page, 2);
    assert_eq!(page.page_size, 50);
}

#[tokio::test]
async fn list_queues_in_vhost_encodes_vhost() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        // The default vhost "/" must be percent-encoded as %2F.
        .and(path("/api/queues/%2F"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "q1",
                "vhost": "/",
                "durable": true,
                "auto_delete": false
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let queues = c.list_queues_in_vhost("/").await.unwrap();
    assert_eq!(queues.len(), 1);
    assert_eq!(queues[0].name, "q1");
}

#[tokio::test]
async fn get_queue_uses_ctx_on_404() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/queues/%2F/q1"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.get_queue("/", "q1").await.unwrap_err();
    match err {
        Error::NotFound(msg) => {
            assert!(
                msg.contains("queue 'q1'"),
                "expected context in message, got: {msg}"
            );
            assert!(
                msg.contains("vhost '/'"),
                "expected vhost context in message, got: {msg}"
            );
        }
        other => panic!("expected Error::NotFound, got {other:?}"),
    }
}

#[tokio::test]
async fn declare_queue_puts_body() {
    let srv = common::server().await;
    Mock::given(method("PUT"))
        .and(path("/api/queues/%2F/q1"))
        .and(body_json(serde_json::json!({
            "durable": true,
            "auto_delete": false,
            "arguments": {"x-max-length": 10}
        })))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(201).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let mut args = serde_json::Map::new();
    args.insert("x-max-length".into(), serde_json::json!(10));
    let opts = QueueDeclareOptions {
        durable: true,
        auto_delete: false,
        arguments: args,
    };
    c.declare_queue("/", "q1", &opts).await.unwrap();
}

#[tokio::test]
async fn delete_queue_hits_encoded_path() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        .and(path("/api/queues/%2F/q1"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.delete_queue("/", "q1").await.unwrap();
}

#[tokio::test]
async fn purge_queue_deletes_contents() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        .and(path("/api/queues/%2F/q1/contents"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.purge_queue("/", "q1").await.unwrap();
}

#[tokio::test]
async fn queue_action_posts_sync() {
    let srv = common::server().await;
    Mock::given(method("POST"))
        .and(path("/api/queues/%2F/q1/actions"))
        .and(body_json(serde_json::json!({"action": "sync"})))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.queue_action("/", "q1", QueueAction::Sync).await.unwrap();
}

#[tokio::test]
async fn get_messages_posts_options() {
    let srv = common::server().await;
    Mock::given(method("POST"))
        .and(path("/api/queues/%2F/q1/get"))
        .and(body_json(serde_json::json!({
            "count": 2,
            "ackmode": "ack_requeue_false",
            "encoding": "auto"
        })))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "payload": "hello",
                "payload_encoding": "string",
                "message_count": 4,
                "exchange": "amq.direct",
                "routing_key": "q1",
                "properties": {"delivery_mode": 1},
                "redelivered": false
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let msgs = c
        .get_messages(
            "/",
            "q1",
            2,
            AckMode::AckRequeueFalse,
            PayloadEncoding::Auto,
        )
        .await
        .unwrap();

    assert_eq!(msgs.len(), 1);
    assert_eq!(msgs[0].payload, "hello");
    assert_eq!(msgs[0].message_count, 4);
    assert_eq!(msgs[0].exchange.as_deref(), Some("amq.direct"));
    assert_eq!(msgs[0].routing_key.as_deref(), Some("q1"));
    assert_eq!(msgs[0].redelivered, Some(false));
}
