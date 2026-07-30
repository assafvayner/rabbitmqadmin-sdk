//! Types for `GET /api/connections` and related endpoints.

use serde::Deserialize;

/// A client connection to the broker, as returned by
/// `GET /api/connections`.
///
/// Only `name` is guaranteed; every other field is optional because the
/// Management API may omit fields depending on connection state and the
/// monitoring level.
#[derive(Debug, Clone, Deserialize)]
pub struct Connection {
    /// Connection name, typically
    /// `"<peer_host>:<peer_port> -> <host>:<port>"`.
    pub name: String,
    /// Virtual host the connection is on.
    #[serde(default)]
    pub vhost: Option<String>,
    /// Username the connection authenticated as.
    #[serde(default)]
    pub user: Option<String>,
    /// Connection state (e.g. `"running"`, `"blocked"`, `"closing"`).
    #[serde(default)]
    pub state: Option<String>,
    /// Protocol name (e.g. `"AMQP 0-9-1"`).
    #[serde(default)]
    pub protocol: Option<String>,
    /// Broker host the connection is established to.
    #[serde(default)]
    pub host: Option<String>,
    /// Broker port the connection is established to.
    #[serde(default)]
    pub port: Option<u32>,
    /// Client (peer) host.
    #[serde(default)]
    pub peer_host: Option<String>,
    /// Client (peer) port.
    #[serde(default)]
    pub peer_port: Option<u32>,
    /// Number of channels open on this connection.
    #[serde(default)]
    pub channels: Option<u64>,
    /// Timestamp (milliseconds since epoch) when the connection was
    /// established.
    #[serde(default)]
    pub connected_at: Option<u64>,
    /// Octets received over this connection.
    #[serde(default)]
    pub recv_oct: Option<u64>,
    /// Octets sent over this connection.
    #[serde(default)]
    pub send_oct: Option<u64>,
}
