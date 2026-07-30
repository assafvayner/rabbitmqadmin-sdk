//! Types for the `/api/users` endpoints.

use serde::{Deserialize, Serialize};

/// A user, as returned by `GET /api/users` and `GET /api/users/{name}`.
#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    /// Name of the user.
    pub name: String,
    /// Comma-separated list of user tags (e.g. `"administrator"`), as
    /// returned by the API.
    #[serde(default)]
    pub tags: Option<String>,
    /// Password hashing algorithm the user's password is stored with.
    #[serde(default)]
    pub hashing_algorithm: Option<String>,
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
