//! Response and request payload types for the RabbitMQ Management API.

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
