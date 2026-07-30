//! Integration tests for the vhosts, users, and permissions endpoints:
//! `list_vhosts()`, `get_vhost()`, `create_vhost()`, `delete_vhost()`,
//! `list_vhost_permissions()`, `list_users()`, `get_user()`,
//! `create_user()`, `delete_user()`, `list_user_permissions()`,
//! `set_permission()`, `delete_permission()`.

mod common;

use rabbitmqadmin_sdk::types::user::UserCreate;
use rabbitmqadmin_sdk::{Client, Error};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, ResponseTemplate};

#[tokio::test]
async fn list_vhosts_returns_all() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/vhosts"))
        .and(common::guest_auth())
        // First entry mirrors the real RabbitMQ 4.3.4 GET /api/vhosts
        // payload, including the 4.x-only fields default_queue_type,
        // protected_from_deletion, tracing, and metadata.
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "/",
                "description": "Default virtual host",
                "metadata": {
                    "description": "Default virtual host",
                    "tags": []
                },
                "tags": [],
                "default_queue_type": "classic",
                "protected_from_deletion": false,
                "tracing": false,
                "cluster_state": {"rabbit@localhost": "running"},
                "messages": 3,
                "messages_ready": 2,
                "messages_unacknowledged": 1
            },
            {
                "name": "prod",
                "description": "Production",
                "tags": ["dc1", "critical"],
                "cluster_state": {}
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let vhosts = c.list_vhosts().await.unwrap();

    assert_eq!(vhosts.len(), 2);
    assert_eq!(vhosts[0].name, "/");
    assert_eq!(vhosts[0].messages, Some(3));
    assert_eq!(vhosts[0].default_queue_type.as_deref(), Some("classic"));
    assert_eq!(vhosts[0].protected_from_deletion, Some(false));
    assert_eq!(vhosts[0].tracing, Some(false));
    assert!(
        vhosts[0].metadata.is_some(),
        "4.x metadata object is captured"
    );
    assert_eq!(vhosts[1].name, "prod");
    // 3.12 brokers don't emit the 4.x fields: they stay None.
    assert_eq!(vhosts[1].default_queue_type, None);
    assert_eq!(vhosts[1].protected_from_deletion, None);
    assert_eq!(vhosts[1].tracing, None);
    assert_eq!(vhosts[1].metadata, None);
    assert_eq!(
        vhosts[1].tags.as_deref(),
        Some(vec!["dc1".to_string(), "critical".to_string()].as_slice())
    );
}

#[tokio::test]
async fn get_vhost_404_ctx() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/vhosts/nope"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.get_vhost("nope").await.unwrap_err();
    match err {
        Error::NotFound(msg) => {
            assert!(
                msg.contains("vhost 'nope'"),
                "expected vhost context in message, got: {msg}"
            );
        }
        other => panic!("expected Error::NotFound, got {other:?}"),
    }
}

#[tokio::test]
async fn create_vhost_puts_empty_body() {
    let srv = common::server().await;
    Mock::given(method("PUT"))
        .and(path("/api/vhosts/prod"))
        .and(body_json(serde_json::json!({})))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(201).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.create_vhost("prod").await.unwrap();
}

#[tokio::test]
async fn delete_vhost_encoded() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        // The default vhost "/" must be percent-encoded as %2F.
        .and(path("/api/vhosts/%2F"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.delete_vhost("/").await.unwrap();
}

#[tokio::test]
async fn list_vhost_permissions() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/vhosts/%2F/permissions"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "user": "guest",
                "vhost": "/",
                "configure": ".*",
                "write": ".*",
                "read": ".*"
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let perms = c.list_vhost_permissions("/").await.unwrap();
    assert_eq!(perms.len(), 1);
    assert_eq!(perms[0].user, "guest");
    assert_eq!(perms[0].vhost, "/");
    assert_eq!(perms[0].configure, ".*");
    assert_eq!(perms[0].write, ".*");
    assert_eq!(perms[0].read, ".*");
}

#[tokio::test]
async fn list_users_deserializes_4x_payload() {
    let srv = common::server().await;
    // Real RabbitMQ 4.x GET /api/users payload: tags is a JSON array,
    // and password_hash / limits fields may be present.
    Mock::given(method("GET"))
        .and(path("/api/users"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "guest",
                "password_hash": "BASE64HASH==",
                "hashing_algorithm": "rabbit_password_hashing_sha256",
                "tags": ["administrator"],
                "limits": {}
            },
            {
                "name": "app",
                "tags": ["management", "policymaker"]
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let users = c.list_users().await.unwrap();

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "guest");
    assert_eq!(users[0].tags, vec!["administrator"]);
    assert_eq!(
        users[0].hashing_algorithm.as_deref(),
        Some("rabbit_password_hashing_sha256")
    );
    assert_eq!(users[0].password_hash.as_deref(), Some("BASE64HASH=="));
    assert!(users[0].limits.is_some());
    assert_eq!(users[1].name, "app");
    assert_eq!(users[1].tags, vec!["management", "policymaker"]);
    assert_eq!(users[1].hashing_algorithm, None);
    assert_eq!(users[1].password_hash, None);
    assert_eq!(users[1].limits, None);
}

#[tokio::test]
async fn list_users_accepts_3x_tags_as_comma_separated_string() {
    let srv = common::server().await;
    // RabbitMQ 3.12 shape: tags is a comma-separated string.
    Mock::given(method("GET"))
        .and(path("/api/users"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "name": "guest",
                "tags": "administrator",
                "hashing_algorithm": "rabbit_password_hashing_sha256"
            },
            {
                "name": "app",
                "tags": "management,policymaker"
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let users = c.list_users().await.unwrap();

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].tags, vec!["administrator"]);
    assert_eq!(users[1].tags, vec!["management", "policymaker"]);
}

#[tokio::test]
async fn create_user_puts_password_and_tags() {
    let srv = common::server().await;
    Mock::given(method("PUT"))
        .and(path("/api/users/app"))
        .and(body_json(serde_json::json!({
            "password": "s3cret",
            "tags": "management"
        })))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(201).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.create_user("app", &UserCreate::new("s3cret", "management"))
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_user_encoded() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        // Spaces in user names must be percent-encoded as %20.
        .and(path("/api/users/app%20svc"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(204).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.delete_user("app svc").await.unwrap();
}

#[tokio::test]
async fn list_user_permissions() {
    let srv = common::server().await;
    Mock::given(method("GET"))
        .and(path("/api/users/guest/permissions"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "user": "guest",
                "vhost": "/",
                "configure": ".*",
                "write": ".*",
                "read": ".*"
            }
        ])))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let perms = c.list_user_permissions("guest").await.unwrap();
    assert_eq!(perms.len(), 1);
    assert_eq!(perms[0].user, "guest");
    assert_eq!(perms[0].vhost, "/");
    assert_eq!(perms[0].configure, ".*");
}

#[tokio::test]
async fn set_permission_puts_regexes() {
    let srv = common::server().await;
    Mock::given(method("PUT"))
        .and(path("/api/permissions/%2F/guest"))
        .and(body_json(serde_json::json!({
            "configure": ".*",
            "write": ".*",
            "read": ".*"
        })))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(201).set_body_string(""))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    c.set_permission("/", "guest", ".*", ".*", ".*")
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_permission_404_ctx() {
    let srv = common::server().await;
    Mock::given(method("DELETE"))
        .and(path("/api/permissions/%2F/guest"))
        .and(common::guest_auth())
        .respond_with(ResponseTemplate::new(404).set_body_string("Object Not Found"))
        .expect(1)
        .mount(&srv)
        .await;

    let c = Client::new(&srv.uri(), "guest", "guest").unwrap();
    let err = c.delete_permission("/", "guest").await.unwrap_err();
    match err {
        Error::NotFound(msg) => {
            assert!(
                msg.contains("permission for user 'guest'"),
                "expected permission context in message, got: {msg}"
            );
            assert!(
                msg.contains("in vhost '/'"),
                "expected vhost context in message, got: {msg}"
            );
        }
        other => panic!("expected Error::NotFound, got {other:?}"),
    }
}
