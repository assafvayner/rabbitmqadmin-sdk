//! Types for the `/api/exchanges` endpoints.

use serde::Deserialize;
use serde_json::Value;

/// An exchange as returned by `GET /api/exchanges` and related endpoints.
#[derive(Debug, Clone, Deserialize)]
pub struct Exchange {
    /// Exchange name.
    pub name: String,
    /// Virtual host the exchange lives in.
    pub vhost: String,
    /// Exchange type (`"direct"`, `"fanout"`, `"topic"`, `"headers"`, ...).
    #[serde(rename = "type")]
    pub type_: String,
    /// Whether the exchange survives a broker restart.
    pub durable: bool,
    /// Whether the exchange is deleted when its last bound queue is unbound.
    pub auto_delete: bool,
    /// Whether the exchange is internal (cannot be published to directly).
    pub internal: bool,
    /// Exchange arguments (alternate-exchange, ...); open-ended map.
    #[serde(default)]
    pub arguments: Value,
    /// Message rate counters for this exchange.
    #[serde(default)]
    pub message_stats: Option<super::common::MessageStats>,
}

/// Request body for `PUT /api/exchanges/{vhost}/{name}` (exchange declaration).
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExchangeDeclareOptions {
    /// Exchange type (`"direct"`, `"fanout"`, `"topic"`, `"headers"`, ...).
    #[serde(rename = "type")]
    pub type_: String,
    /// Whether the exchange survives a broker restart.
    pub durable: bool,
    /// Whether the exchange is deleted when its last bound queue is unbound.
    pub auto_delete: bool,
    /// Whether the exchange is internal (cannot be published to directly).
    pub internal: bool,
    /// Optional exchange arguments.
    #[serde(default)]
    pub arguments: serde_json::Map<String, Value>,
}

/// Request body for `POST /api/exchanges/{vhost}/{name}/publish`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct PublishMessage {
    /// Message properties (open-ended map).
    pub properties: Value,
    /// Routing key the message is published with.
    pub routing_key: String,
    /// Message payload.
    pub payload: String,
    /// Encoding of `payload`: `"string"` or `"base64"`.
    pub payload_encoding: String,
}

impl PublishMessage {
    /// Create a new message with the given routing key and payload, empty
    /// properties, and `payload_encoding` of `"string"`.
    pub fn new(routing_key: impl Into<String>, payload: impl Into<String>) -> Self {
        Self {
            properties: serde_json::json!({}),
            routing_key: routing_key.into(),
            payload: payload.into(),
            payload_encoding: "string".to_owned(),
        }
    }
}

/// Response of `POST /api/exchanges/{vhost}/{name}/publish`.
#[derive(Debug, Clone, Deserialize)]
pub struct PublishResult {
    /// Whether the message was routed to at least one queue.
    pub routed: bool,
}
