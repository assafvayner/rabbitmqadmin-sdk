//! Types shared across multiple Management API responses.

use serde::Deserialize;
use serde_json::Value;

/// Message rate counters as returned inside overview, queue, channel, and
/// connection payloads under the `message_stats` key.
///
/// Any counter may be absent (RabbitMQ omits stats for objects that have
/// had no traffic), so every field is optional.
#[derive(Debug, Clone, Deserialize)]
pub struct MessageStats {
    /// Total messages published.
    #[serde(default)]
    pub publish: Option<u64>,
    /// Total messages delivered to consumers plus basic.get responses.
    #[serde(default)]
    pub deliver_get: Option<u64>,
    /// Total messages acknowledged.
    #[serde(default)]
    pub ack: Option<u64>,
    /// Total messages redelivered.
    #[serde(default)]
    pub redeliver: Option<u64>,
    /// Total publisher confirms received.
    #[serde(default)]
    pub confirm: Option<u64>,
    /// Rate details for `publish` (open-ended map of `rate`, `interval`, ...).
    #[serde(default)]
    pub publish_details: Option<Value>,
    /// Rate details for `deliver_get`.
    #[serde(default)]
    pub deliver_get_details: Option<Value>,
    /// Rate details for `ack`.
    #[serde(default)]
    pub ack_details: Option<Value>,
}

/// Cluster-wide object counts, as returned by `GET /api/overview` under
/// `object_totals`. All fields are always present in the response.
#[derive(Debug, Clone, Deserialize)]
pub struct ObjectTotals {
    /// Number of channels in the cluster.
    pub channels: u64,
    /// Number of connections in the cluster.
    pub connections: u64,
    /// Number of consumers in the cluster.
    pub consumers: u64,
    /// Number of exchanges in the cluster.
    pub exchanges: u64,
    /// Number of queues in the cluster.
    pub queues: u64,
}

/// Cluster-wide message totals, as returned by `GET /api/overview` under
/// `queue_totals`. All fields are always present in the response.
#[derive(Debug, Clone, Deserialize)]
pub struct QueueTotals {
    /// Total messages across all queues (ready + unacknowledged).
    pub messages: u64,
    /// Total messages ready for delivery across all queues.
    pub messages_ready: u64,
    /// Total messages delivered but not yet acknowledged across all queues.
    pub messages_unacknowledged: u64,
}
