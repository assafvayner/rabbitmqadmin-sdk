//! `GET /api/vhosts` and related vhost endpoints.

use crate::api::encode_segment;
use crate::types::permission::Permission;
use crate::types::vhost::Vhost;
use crate::{Client, Result};

impl Client {
    /// `GET /api/vhosts` — lists all virtual hosts.
    pub async fn list_vhosts(&self) -> Result<Vec<Vhost>> {
        self.get("vhosts", None).await
    }

    /// `GET /api/vhosts/{name}` — returns details of a single vhost.
    /// The name is percent-encoded (e.g. `/` becomes `%2F`).
    pub async fn get_vhost(&self, name: &str) -> Result<Vhost> {
        self.get_ctx(
            &format!("vhosts/{}", encode_segment(name)),
            None,
            &format!("vhost '{name}'"),
        )
        .await
    }

    /// `PUT /api/vhosts/{name}` — creates a vhost. The name is
    /// percent-encoded.
    pub async fn create_vhost(&self, name: &str) -> Result<()> {
        self.put(
            &format!("vhosts/{}", encode_segment(name)),
            &serde_json::json!({}),
        )
        .await
    }

    /// `DELETE /api/vhosts/{name}` — deletes a vhost. The name is
    /// percent-encoded.
    pub async fn delete_vhost(&self, name: &str) -> Result<()> {
        self.delete_ctx(
            &format!("vhosts/{}", encode_segment(name)),
            &format!("vhost '{name}'"),
        )
        .await
    }

    /// `GET /api/vhosts/{vhost}/permissions` — lists all permissions
    /// granted within a single vhost. The vhost is percent-encoded.
    pub async fn list_vhost_permissions(&self, vhost: &str) -> Result<Vec<Permission>> {
        self.get(
            &format!("vhosts/{}/permissions", encode_segment(vhost)),
            None,
        )
        .await
    }
}
