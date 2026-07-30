//! Types for `GET /api/channels` and related endpoints.

use serde::Deserialize;

/// A channel on a client connection, as returned by
/// `GET /api/channels`.
#[derive(Debug, Clone, Deserialize)]
pub struct Channel {
    /// Channel name, typically `"<connection name> (<number>)"`.
    pub name: String,
    /// Virtual host the channel is on.
    #[serde(default)]
    pub vhost: Option<String>,
    /// Username the underlying connection authenticated as.
    #[serde(default)]
    pub user: Option<String>,
    /// Channel number within the connection.
    #[serde(default)]
    pub number: Option<u32>,
    /// Name of the cluster node the channel is running on.
    #[serde(default)]
    pub node: Option<String>,
    /// Channel state (e.g. `"running"`, `"blocked"`).
    #[serde(default)]
    pub state: Option<String>,
    /// Number of consumers on this channel.
    #[serde(default)]
    pub consumer_count: Option<u64>,
    /// Per-consumer prefetch (QoS) limit.
    #[serde(default)]
    pub prefetch_count: Option<u32>,
    /// Messages delivered but not yet acknowledged.
    #[serde(default)]
    pub messages_unacknowledged: Option<u64>,
    /// Messages published but not yet confirmed by the broker.
    #[serde(default)]
    pub messages_unconfirmed: Option<u64>,
    /// Message counters (publish, ack, ...), when present.
    #[serde(default)]
    pub message_stats: Option<super::common::MessageStats>,
    /// Details of the connection this channel belongs to (open-ended map).
    #[serde(default)]
    pub connection_details: Option<serde_json::Value>,
}
