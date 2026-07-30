//! Types for the `/api/queues` endpoints.

use serde::Deserialize;
use serde_json::Value;

/// A queue as returned by `GET /api/queues` and related endpoints.
///
/// Monitoring fields (message counts, consumers, memory, ...) are absent
/// from the response when the queue has had no activity or the queue is
/// not running, so they are all optional.
#[derive(Debug, Clone, Deserialize)]
pub struct Queue {
    /// Queue name.
    pub name: String,
    /// Virtual host the queue lives in.
    pub vhost: String,
    /// Whether the queue survives a broker restart.
    pub durable: bool,
    /// Whether the queue is deleted when its last consumer unsubscribes.
    pub auto_delete: bool,
    /// Whether the queue is exclusive to its declaring connection.
    #[serde(default)]
    pub exclusive: bool,
    /// Queue arguments (`x-max-length`, `x-message-ttl`, ...); open-ended map.
    #[serde(default)]
    pub arguments: Value,
    /// Queue state (e.g. `"running"`).
    #[serde(default)]
    pub state: Option<String>,
    /// Total messages (ready + unacknowledged).
    #[serde(default)]
    pub messages: Option<u64>,
    /// Messages ready for delivery.
    #[serde(default)]
    pub messages_ready: Option<u64>,
    /// Messages delivered but not yet acknowledged.
    #[serde(default)]
    pub messages_unacknowledged: Option<u64>,
    /// Number of consumers.
    #[serde(default)]
    pub consumers: Option<u64>,
    /// Fraction of the time the queue can deliver messages to consumers.
    #[serde(default)]
    pub consumer_utilisation: Option<f64>,
    /// Bytes of memory used by the queue process.
    #[serde(default)]
    pub memory: Option<u64>,
    /// Message rate counters for this queue.
    #[serde(default)]
    pub message_stats: Option<super::common::MessageStats>,
}

/// Request body for `PUT /api/queues/{vhost}/{name}` (queue declaration).
#[derive(Debug, Clone, serde::Serialize, Default)]
pub struct QueueDeclareOptions {
    /// Whether the queue survives a broker restart.
    pub durable: bool,
    /// Whether the queue is deleted when its last consumer unsubscribes.
    pub auto_delete: bool,
    /// Optional queue arguments (`x-max-length`, `x-message-ttl`, ...).
    #[serde(default)]
    pub arguments: serde_json::Map<String, Value>,
}

/// Action for `POST /api/queues/{vhost}/{name}/actions`.
#[derive(Debug, Clone, Copy)]
pub enum QueueAction {
    /// Synchronise the queue (for quorum / classic mirrored queues).
    Sync,
    /// Cancel an in-progress synchronisation.
    CancelSync,
}

impl QueueAction {
    /// Wire string for the `"action"` field of the request body.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Sync => "sync",
            Self::CancelSync => "cancel_sync",
        }
    }
}

/// Acknowledgement mode for `POST /api/queues/{vhost}/{name}/get`.
#[derive(Debug, Clone, Copy)]
pub enum AckMode {
    /// Acknowledge the message, requeueing it.
    AckRequeueTrue,
    /// Acknowledge the message without requeueing (message removed).
    AckRequeueFalse,
    /// Negatively acknowledge, requeueing the message.
    NackRequeueTrue,
    /// Reject the message without requeueing.
    RejectRequeueFalse,
}

impl AckMode {
    /// Wire string for the `"ackmode"` field of the request body.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AckRequeueTrue => "ack_requeue_true",
            Self::AckRequeueFalse => "ack_requeue_false",
            Self::NackRequeueTrue => "nack_requeue_true",
            Self::RejectRequeueFalse => "reject_requeue_false",
        }
    }
}

/// Payload encoding for messages fetched via the HTTP API.
#[derive(Debug, Clone, Copy)]
pub enum PayloadEncoding {
    /// Return the payload as a string if it is valid UTF-8, else base64.
    Auto,
    /// Always return the payload base64-encoded.
    Base64,
}

impl PayloadEncoding {
    /// Wire string for the `"encoding"` field of the request body.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Base64 => "base64",
        }
    }
}

/// A single message as returned by `POST /api/queues/{vhost}/{name}/get`.
#[derive(Debug, Clone, Deserialize)]
pub struct GetMessage {
    /// Message payload (a string, or base64 per `payload_encoding`).
    pub payload: String,
    /// Encoding of `payload`: `"auto"` or `"base64"`.
    pub payload_encoding: String,
    /// Messages remaining in the queue after this one was returned.
    pub message_count: u64,
    /// Exchange the message was published to.
    #[serde(default)]
    pub exchange: Option<String>,
    /// Routing key the message was published with.
    #[serde(default)]
    pub routing_key: Option<String>,
    /// Message properties (open-ended map).
    #[serde(default)]
    pub properties: Option<Value>,
    /// Whether the message was redelivered.
    #[serde(default)]
    pub redelivered: Option<bool>,
}
