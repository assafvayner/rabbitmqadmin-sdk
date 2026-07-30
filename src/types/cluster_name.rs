//! Response type for `GET /api/cluster-name`.

use serde::Deserialize;

/// Wire shape of `GET /api/cluster-name`: `{ "name": "..." }`.
///
/// The public API ([`crate::Client::cluster_name`]) unwraps this and
/// returns a plain [`String`].
#[derive(Debug, Clone, Deserialize)]
pub struct ClusterName {
    /// The cluster's name (typically the name of the first node, e.g.
    /// `"rabbit@host"`).
    pub name: String,
}
