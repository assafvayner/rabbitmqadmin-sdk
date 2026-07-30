//! `PUT/DELETE /api/permissions/{vhost}/{user}` endpoints.

use crate::api::encode_segment;
use crate::{Client, Result};

/// Request body for `PUT /api/permissions/{vhost}/{user}`.
#[derive(Debug, Clone, serde::Serialize)]
struct PermissionSet {
    /// Regular expression controlling which resources the user may
    /// configure (declare/delete).
    configure: String,
    /// Regular expression controlling which resources the user may
    /// write (publish) to.
    write: String,
    /// Regular expression controlling which resources the user may
    /// read (consume) from.
    read: String,
}

impl Client {
    /// `PUT /api/permissions/{vhost}/{user}` — grants a user configure,
    /// write, and read permissions within a vhost, each as a regular
    /// expression matched against resource names. Both segments are
    /// percent-encoded.
    pub async fn set_permission(
        &self,
        vhost: &str,
        user: &str,
        configure: &str,
        write: &str,
        read: &str,
    ) -> Result<()> {
        self.put(
            &permission_path(vhost, user),
            &PermissionSet {
                configure: configure.to_owned(),
                write: write.to_owned(),
                read: read.to_owned(),
            },
        )
        .await
    }

    /// `DELETE /api/permissions/{vhost}/{user}` — revokes all of a user's
    /// permissions within a vhost. Both segments are percent-encoded.
    pub async fn delete_permission(&self, vhost: &str, user: &str) -> Result<()> {
        self.delete_ctx(
            &permission_path(vhost, user),
            &format!("permission for user '{user}' in vhost '{vhost}'"),
        )
        .await
    }
}

/// Percent-encode vhost and user into a `permissions/{vhost}/{user}`
/// relative path.
fn permission_path(vhost: &str, user: &str) -> String {
    format!(
        "permissions/{}/{}",
        encode_segment(vhost),
        encode_segment(user)
    )
}
