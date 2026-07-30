//! Types for the `/api/permissions` endpoints.

use serde::{Deserialize, Serialize};

/// A permission grant: the configure/write/read regular expressions a user
/// holds within a virtual host, as returned by
/// `GET /api/vhosts/{vhost}/permissions` and
/// `GET /api/users/{user}/permissions`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    /// Name of the user the permission is granted to.
    pub user: String,
    /// Virtual host the permission applies to.
    pub vhost: String,
    /// Regular expression controlling which resources the user may
    /// configure (declare/delete).
    pub configure: String,
    /// Regular expression controlling which resources the user may
    /// write (publish) to.
    pub write: String,
    /// Regular expression controlling which resources the user may
    /// read (consume) from.
    pub read: String,
}
