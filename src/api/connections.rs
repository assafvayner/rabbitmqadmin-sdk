//! `GET /api/connections` and related connection endpoints.

use crate::api::{encode_segment, nf};
use crate::types::channel::Channel;
use crate::types::connection::Connection;
use crate::{Client, Paginated, PaginationQuery, Result};

impl Client {
    /// `GET /api/connections` — lists all connections to the cluster.
    pub async fn list_connections(&self) -> Result<Vec<Connection>> {
        self.get("connections", None).await
    }

    /// `GET /api/connections` with pagination parameters — returns a
    /// [`Paginated`] page of connections.
    pub async fn list_connections_paged(
        &self,
        q: &PaginationQuery,
    ) -> Result<Paginated<Connection>> {
        self.get("connections", Some(q)).await
    }

    /// `GET /api/connections/{name}` — returns a single connection. The
    /// name is percent-encoded (connection names commonly contain spaces,
    /// colons, and `->`, e.g. `"127.0.0.1:52341 -> 127.0.0.1:5672"`).
    pub async fn get_connection(&self, name: &str) -> Result<Connection> {
        self.get_ctx(
            &format!("connections/{}", encode_segment(name)),
            None,
            &format!("connection '{name}'"),
        )
        .await
    }

    /// `DELETE /api/connections/{name}` — closes a connection. When
    /// `reason` is `Some`, it is sent to the server in the `X-Reason`
    /// request header (recorded in the server log). The name is
    /// percent-encoded.
    pub async fn close_connection(&self, name: &str, reason: Option<&str>) -> Result<()> {
        let headers: &[(&str, &str)] = match reason {
            Some(r) => &[("X-Reason", r)],
            None => &[],
        };
        self.delete_with_headers(&format!("connections/{}", encode_segment(name)), headers)
            .await
            .map_err(|e| nf(e, &format!("connection '{name}'")))
    }

    /// `GET /api/connections/{name}/channels` — lists all channels open
    /// on a single connection. The name is percent-encoded.
    pub async fn list_connection_channels(&self, name: &str) -> Result<Vec<Channel>> {
        self.get(
            &format!("connections/{}/channels", encode_segment(name)),
            None,
        )
        .await
    }
}
