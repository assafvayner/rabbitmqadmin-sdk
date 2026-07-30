//! Integration tests for the exchanges endpoints:
//! `list_exchanges()`, `list_exchanges_paged()`, `list_exchanges_in_vhost()`,
//! `get_exchange()`, `declare_exchange()`, `delete_exchange()`,
//! `publish_to_exchange()`, `list_exchange_bindings_source()`.

mod common;

use rabbitmqadmin_sdk::types::exchange::{ExchangeDeclareOptions, PublishMessage};
use rabbitmqadmin_sdk::{Client, Error, PaginationQuery};
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn list_exchanges_returns_all() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/exchanges"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "amq.direct",
                "vhost": "/",
                "type": "direct",
                "durable": true,
                "auto_delete": false,
                "internal": true,
                "arguments": {}
            },
            {
                "name": "events",
                "vhost": "vh",
                "type": "topic",
                "durable": false,
                "auto_delete": true,
                "internal": false,
                "arguments": {"alternate-exchange": "amq.fanout"},
                "message_stats": {"publish": 12}
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let exchanges = c.list_exchanges().await.unwrap();

    assert_eq!(exchanges.len(), 2);
    assert_eq!(exchanges[0].name, "amq.direct");
    assert_eq!(exchanges[0].vhost, "/");
    // The JSON "type" key must deserialize into `type_`.
    assert_eq!(exchanges[0].type_, "direct");
    assert!(exchanges[0].durable);
    assert!(exchanges[0].internal);

    assert_eq!(exchanges[1].type_, "topic");
    assert!(!exchanges[1].durable);
    assert!(exchanges[1].auto_delete);
    let stats = exchanges[1].message_stats.as_ref().expect("message_stats");
    assert_eq!(stats.publish, Some(12));
}

#[tokio::test]
async fn list_exchanges_paged_sends_params() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/exchanges"))
        .and(query_param("pagination", "true"))
        .and(query_param("page", "1"))
        .and(query_param("page_size", "10"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "filtered_count": 1,
            "item_count": 1,
            "items": [{
                "name": "amq.fanout",
                "vhost": "/",
                "type": "fanout",
                "durable": true,
                "auto_delete": false,
                "internal": false,
                "arguments": {}
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
    let page = c.list_exchanges_paged(&q).await.unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].name, "amq.fanout");
    assert_eq!(page.total_count, 1);
    assert_eq!(page.page, 1);
    assert_eq!(page.page_size, 10);
}

#[tokio::test]
async fn list_exchanges_in_vhost_encodes() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        // The default vhost "/" must be percent-encoded as %2F.
        .and(path("/api/exchanges/%2F"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "amq.direct",
                "vhost": "/",
                "type": "direct",
                "durable": true,
                "auto_delete": false,
                "internal": false,
                "arguments": {}
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let exchanges = c.list_exchanges_in_vhost("/").await.unwrap();
    assert_eq!(exchanges.len(), 1);
    assert_eq!(exchanges[0].name, "amq.direct");
}

#[tokio::test]
async fn get_exchange_404_has_ctx() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/exchanges/%2F/missing"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.get_exchange("/", "missing").await.unwrap_err();
    match err {
        Error::NotFound(msg) => {
            assert!(
                msg.contains("exchange 'missing'"),
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
async fn declare_exchange_puts_body() {
    let srv = common::server().await;
    Mock::given(method("PUT"))
        .and(path("/api/exchanges/%2F/events"))
        .and(body_json(serde_json::json!({
            "type": "fanout",
            "durable": true,
            "auto_delete": false,
            "internal": false,
            "arguments": {}
        })))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(201).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let opts = ExchangeDeclareOptions {
        type_: "fanout".to_owned(),
        durable: true,
        auto_delete: false,
        internal: false,
        arguments: serde_json::Map::new(),
    };
    c.declare_exchange("/", "events", &opts).await.unwrap();
}

#[tokio::test]
async fn delete_exchange_encoded_path() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        // Both vhost "/" and the space in "my ex" must be percent-encoded.
        .and(path("/api/exchanges/%2F/my%20ex"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.delete_exchange("/", "my ex").await.unwrap();
}

#[tokio::test]
async fn publish_to_exchange_posts_and_reads_routed() {
    let srv = common::server().await;
    Mock::given(method("POST"))
        .and(path("/api/exchanges/%2F/amq.direct/publish"))
        .and(body_json(serde_json::json!({
            "properties": {},
            "routing_key": "rk",
            "payload": "hello",
            "payload_encoding": "string"
        })))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "routed": true
        })))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let msg = PublishMessage::new("rk", "hello");
    let result = c
        .publish_to_exchange("/", "amq.direct", &msg)
        .await
        .unwrap();

    assert!(result.routed);
}

#[tokio::test]
async fn list_exchange_bindings_source_returns_array() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/exchanges/%2F/amq.direct/bindings/source"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "source": "amq.direct",
                "vhost": "/",
                "destination": "q1",
                "destination_type": "queue",
                "routing_key": "q1",
                "arguments": {},
                "properties_key": "q1"
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let bindings = c
        .list_exchange_bindings_source("/", "amq.direct")
        .await
        .unwrap();

    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].source, "amq.direct");
    assert_eq!(bindings[0].vhost, "/");
    assert_eq!(bindings[0].destination, "q1");
    assert_eq!(bindings[0].destination_type, "queue");
    assert_eq!(bindings[0].routing_key, "q1");
    assert_eq!(bindings[0].properties_key.as_deref(), Some("q1"));
}
