//! Integration tests for the bindings endpoints:
//! `list_bindings()`, `list_bindings_paged()`, `list_bindings_in_vhost()`,
//! `list_queue_bindings()`, `list_bindings_between()`, `bind()`, `unbind()`.

mod common;

use rabbitmqadmin_sdk::{Client, Error, PaginationQuery};
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn list_bindings_returns_all() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/bindings"))
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
            },
            {
                "source": "events",
                "vhost": "vh",
                "destination": "q2",
                "destination_type": "queue",
                "routing_key": "rk~abc123",
                "arguments": {"x-match": "all"},
                "properties_key": "rk~abc123"
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let bindings = c.list_bindings().await.unwrap();

    assert_eq!(bindings.len(), 2);
    assert_eq!(bindings[0].source, "amq.direct");
    assert_eq!(bindings[0].vhost, "/");
    assert_eq!(bindings[0].destination, "q1");
    assert_eq!(bindings[0].destination_type, "queue");
    assert_eq!(bindings[0].routing_key, "q1");
    assert_eq!(bindings[0].properties_key.as_deref(), Some("q1"));

    assert_eq!(bindings[1].source, "events");
    assert_eq!(bindings[1].vhost, "vh");
    assert_eq!(bindings[1].routing_key, "rk~abc123");
    assert_eq!(bindings[1].arguments["x-match"], "all");
    assert_eq!(bindings[1].properties_key.as_deref(), Some("rk~abc123"));
}

#[tokio::test]
async fn list_bindings_paged_sends_params() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/bindings"))
        .and(query_param("pagination", "true"))
        .and(query_param("page", "1"))
        .and(query_param("page_size", "10"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "filtered_count": 1,
            "item_count": 1,
            "items": [{
                "source": "amq.direct",
                "vhost": "/",
                "destination": "q1",
                "destination_type": "queue",
                "routing_key": "q1",
                "arguments": {},
                "properties_key": "q1"
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
    let page = c.list_bindings_paged(&q).await.unwrap();

    assert_eq!(page.items.len(), 1);
    assert_eq!(page.items[0].source, "amq.direct");
    assert_eq!(page.total_count, 1);
    assert_eq!(page.page, 1);
    assert_eq!(page.page_size, 10);
}

#[tokio::test]
async fn list_bindings_in_vhost_encodes() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        // The default vhost "/" must be percent-encoded as %2F.
        .and(path("/api/bindings/%2F"))
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
    let bindings = c.list_bindings_in_vhost("/").await.unwrap();
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].source, "amq.direct");
}

#[tokio::test]
async fn list_queue_bindings_hits_queue_scoped_path() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/queues/%2F/q1/bindings"))
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
    let bindings = c.list_queue_bindings("/", "q1").await.unwrap();
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].destination, "q1");
    assert_eq!(bindings[0].source, "amq.direct");
}

#[tokio::test]
async fn list_bindings_between_hits_pair_path() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/bindings/%2F/e/amq.direct/q/q1"))
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
        .list_bindings_between("/", "amq.direct", "q1")
        .await
        .unwrap();
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].source, "amq.direct");
    assert_eq!(bindings[0].destination, "q1");
    assert_eq!(bindings[0].routing_key, "q1");
}

#[tokio::test]
async fn bind_posts_routing_key_and_args() {
    let srv = common::server().await;
    Mock::given(method("POST"))
        .and(path("/api/bindings/%2F/e/amq.direct/q/q1"))
        .and(body_json(serde_json::json!({
            "routing_key": "rk",
            "arguments": {"x-match": "all"}
        })))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(201).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.bind(
        "/",
        "amq.direct",
        "q1",
        "rk",
        serde_json::json!({"x-match": "all"}),
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn unbind_deletes_properties_key_path() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        // reqwest's Url::join normalizes '~' to '%7E' in the wire path.
        .and(path("/api/bindings/%2F/e/amq.direct/q/q1/rk%7Eabc123"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.unbind("/", "amq.direct", "q1", "rk~abc123")
        .await
        .unwrap();
}

#[tokio::test]
async fn unbind_404_has_ctx() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        .and(path("/api/bindings/%2F/e/amq.direct/q/q1/rk%7Eabc123"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c
        .unbind("/", "amq.direct", "q1", "rk~abc123")
        .await
        .unwrap_err();
    match err {
        Error::NotFound(msg) => {
            assert!(
                msg.contains("binding 'rk~abc123'"),
                "expected binding context in message, got: {msg}"
            );
            assert!(
                msg.contains("exchange 'amq.direct'"),
                "expected exchange context in message, got: {msg}"
            );
            assert!(
                msg.contains("queue 'q1'"),
                "expected queue context in message, got: {msg}"
            );
        }
        other => panic!("expected Error::NotFound, got {other:?}"),
    }
}
