//! `GET /api/users` and related user endpoints.

use crate::api::encode_segment;
use crate::types::permission::Permission;
use crate::types::user::{UserCreate, UserInfo};
use crate::{Client, Result};

impl Client {
    /// `GET /api/users` — lists all users.
    pub async fn list_users(&self) -> Result<Vec<UserInfo>> {
        self.get("users", None).await
    }

    /// `GET /api/users/{name}` — returns details of a single user.
    /// The name is percent-encoded.
    pub async fn get_user(&self, name: &str) -> Result<UserInfo> {
        self.get_ctx(
            &format!("users/{}", encode_segment(name)),
            None,
            &format!("user '{name}'"),
        )
        .await
    }

    /// `PUT /api/users/{name}` — creates or updates a user. The name is
    /// percent-encoded.
    pub async fn create_user(&self, name: &str, user: &UserCreate) -> Result<()> {
        self.put(&format!("users/{}", encode_segment(name)), user)
            .await
    }

    /// `DELETE /api/users/{name}` — deletes a user. The name is
    /// percent-encoded.
    pub async fn delete_user(&self, name: &str) -> Result<()> {
        self.delete_ctx(
            &format!("users/{}", encode_segment(name)),
            &format!("user '{name}'"),
        )
        .await
    }

    /// `GET /api/users/{user}/permissions` — lists all permissions
    /// granted to a single user across vhosts. The user name is
    /// percent-encoded.
    pub async fn list_user_permissions(&self, user: &str) -> Result<Vec<Permission>> {
        self.get(&format!("users/{}/permissions", encode_segment(user)), None)
            .await
    }
}
