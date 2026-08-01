//! Response type for `GET /api/nodes` and `GET /api/nodes/{name}`.

use serde::Deserialize;

/// A RabbitMQ cluster node and its resource usage metrics, as returned by
/// `GET /api/nodes` or `GET /api/nodes/{name}`.
///
/// Field presence varies by server version and node state (target is
/// RabbitMQ 3.12+), so everything except `name` is optional.
#[derive(Debug, Clone, Deserialize)]
pub struct Node {
    /// Node name, e.g. `"rabbit@host1"`.
    pub name: String,
    /// Node type: `"disc"` or `"ram"`.
    #[serde(rename = "type", default)]
    pub node_type: Option<String>,
    /// Whether the node is running.
    #[serde(default)]
    pub running: Option<bool>,
    /// Memory used by the node, in bytes.
    #[serde(default)]
    pub mem_used: Option<u64>,
    /// Memory high-watermark limit, in bytes.
    #[serde(default)]
    pub mem_limit: Option<u64>,
    /// Whether the memory alarm is in effect.
    #[serde(default)]
    pub mem_alarm: Option<bool>,
    /// Free disk space, in bytes.
    #[serde(default)]
    pub disk_free: Option<u64>,
    /// Disk free low-watermark limit, in bytes.
    #[serde(default)]
    pub disk_free_limit: Option<u64>,
    /// Whether the disk free alarm is in effect.
    #[serde(default)]
    pub disk_free_alarm: Option<bool>,
    /// File descriptors used.
    #[serde(default)]
    pub fd_used: Option<u64>,
    /// File descriptors available.
    #[serde(default)]
    pub fd_total: Option<u64>,
    /// Sockets used.
    #[serde(default)]
    pub sockets_used: Option<u64>,
    /// Sockets available.
    #[serde(default)]
    pub sockets_total: Option<u64>,
    /// Erlang processes used.
    #[serde(default)]
    pub proc_used: Option<u64>,
    /// Erlang processes available.
    #[serde(default)]
    pub proc_total: Option<u64>,
    /// Average length of the Erlang run queue.
    #[serde(default)]
    pub run_queue: Option<u64>,
    /// Node uptime, in milliseconds.
    #[serde(default)]
    pub uptime: Option<u64>,
    /// Operating system PID of the node, as a string.
    #[serde(default)]
    pub os_pid: Option<String>,
    /// Statistics collection mode of the management plugin on this node
    /// (`"none"`, `"basic"`, or `"detailed"`).
    #[serde(default)]
    pub rates_mode: Option<String>,
}
