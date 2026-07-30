//! Response and request payload types for the RabbitMQ Management API.

pub mod binding;
pub mod channel;
pub mod cluster_name;
pub mod common;
pub mod connection;
pub mod consumer;
pub mod definitions;
pub mod exchange;
pub mod health;
pub mod node;
pub mod overview;
pub mod permission;
pub mod policy;
pub mod queue;
pub mod user;
pub mod vhost;

use serde::Deserialize;

use crate::types::common::deserialize_tags;

/// Response of `GET /api/whoami`: the identity the server sees for the
/// credentials used by this client.
#[derive(Debug, Clone, Deserialize)]
pub struct WhoAmI {
    /// Username of the authenticated user.
    pub name: String,
    /// User tags (e.g. `["administrator"]`). RabbitMQ 4.x returns a JSON
    /// array; RabbitMQ 3.12 returns a comma-separated string — both are
    /// normalized to a `Vec<String>`.
    #[serde(default, deserialize_with = "deserialize_tags")]
    pub tags: Vec<String>,
    /// Whether the authenticated user is an internal user (RabbitMQ 4.x
    /// only; `None` on 3.12).
    #[serde(default)]
    pub is_internal_user: Option<bool>,
}
