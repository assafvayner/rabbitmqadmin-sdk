//! Response and request payload types for the RabbitMQ Management API.

pub mod binding;
pub mod channel;
pub mod cluster_name;
pub mod common;
pub mod connection;
pub mod consumer;
pub mod exchange;
pub mod node;
pub mod overview;
pub mod permission;
pub mod policy;
pub mod queue;
pub mod user;
pub mod vhost;

use serde::Deserialize;

/// Response of `GET /api/whoami`: the identity the server sees for the
/// credentials used by this client.
#[derive(Debug, Clone, Deserialize)]
pub struct WhoAmI {
    /// Username of the authenticated user.
    pub name: String,
    /// Comma-separated list of user tags (e.g. `"administrator"`).
    pub tags: String,
}
