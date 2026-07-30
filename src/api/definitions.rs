//! `GET/POST /api/definitions` — cluster definitions export and import.

use crate::api::encode_segment;
use crate::types::definitions::Definitions;
use crate::{Client, Result};

impl Client {
    /// `GET /api/definitions` — exports the cluster-wide definitions
    /// (users, vhosts, permissions, parameters, policies, queues,
    /// exchanges, bindings) as a [`Definitions`] document.
    pub async fn export_definitions(&self) -> Result<Definitions> {
        self.get("definitions", None).await
    }

    /// `GET /api/definitions/{vhost}` — exports the definitions of a
    /// single virtual host. The vhost name is percent-encoded (the
    /// default vhost `/` becomes `%2F`).
    ///
    /// The vhost-scoped export shape is open-ended (server version
    /// dependent), so it is returned as a [`serde_json::Value`].
    pub async fn export_definitions_in_vhost(&self, vhost: &str) -> Result<serde_json::Value> {
        self.get(&format!("definitions/{}", encode_segment(vhost)), None)
            .await
    }

    /// `POST /api/definitions` — imports a definitions document into the
    /// cluster. The document is typically the result of a previous
    /// [`Client::export_definitions`] call (server-generated fields
    /// round-trip verbatim; see the [`Definitions`] module docs).
    pub async fn import_definitions(&self, defs: &Definitions) -> Result<()> {
        self.post("definitions", defs).await
    }
}
