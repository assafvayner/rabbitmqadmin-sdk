//! Types for the `/api/vhosts` endpoints.

use serde::Deserialize;

/// A virtual host, as returned by `GET /api/vhosts` and
/// `GET /api/vhosts/{name}`.
#[derive(Debug, Clone, Deserialize)]
pub struct Vhost {
    /// Name of the virtual host (e.g. `"/"` for the default vhost).
    pub name: String,
    /// Optional human-readable description of the vhost.
    #[serde(default)]
    pub description: Option<String>,
    /// Optional list of tags attached to the vhost.
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    /// Cluster state of the vhost on each node; open-ended map.
    #[serde(default)]
    pub cluster_state: Option<serde_json::Value>,
    /// Total number of messages in queues in this vhost.
    #[serde(default)]
    pub messages: Option<u64>,
    /// Number of messages ready for delivery in this vhost.
    #[serde(default)]
    pub messages_ready: Option<u64>,
    /// Number of unacknowledged messages in this vhost.
    #[serde(default)]
    pub messages_unacknowledged: Option<u64>,
}
