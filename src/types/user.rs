//! Types for the `/api/users` endpoints.

use serde::{Deserialize, Serialize};

use crate::types::common::deserialize_tags;

/// A user, as returned by `GET /api/users` and `GET /api/users/{name}`.
#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    /// Name of the user.
    pub name: String,
    /// User tags (e.g. `["administrator"]`). RabbitMQ 4.x returns a JSON
    /// array; RabbitMQ 3.12 returns a comma-separated string — both are
    /// normalized to a `Vec<String>`. Absent tags deserialize to an
    /// empty vector.
    #[serde(default, deserialize_with = "deserialize_tags")]
    pub tags: Vec<String>,
    /// Password hashing algorithm the user's password is stored with.
    #[serde(default)]
    pub hashing_algorithm: Option<String>,
    /// Password hash, when the server includes it (export workflows).
    #[serde(default)]
    pub password_hash: Option<String>,
    /// Per-user limits (RabbitMQ 4.x); open-ended map.
    #[serde(default)]
    pub limits: Option<serde_json::Value>,
}

/// Request body for `PUT /api/users/{name}`: creates or updates a user.
#[derive(Debug, Clone, Serialize)]
pub struct UserCreate {
    /// Password of the user.
    pub password: String,
    /// Comma-separated list of user tags (e.g. `"management"`).
    pub tags: String,
}

impl UserCreate {
    /// Create a new [`UserCreate`] with the given password and
    /// comma-separated tags.
    pub fn new(password: impl Into<String>, tags: impl Into<String>) -> Self {
        Self {
            password: password.into(),
            tags: tags.into(),
        }
    }
}
