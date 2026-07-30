//! `GET /api/overview` and `GET /api/cluster-name` — cluster-wide
//! monitoring summary and cluster identity.

use crate::types::cluster_name::ClusterName;
use crate::types::overview::Overview;
use crate::{Client, Result};

impl Client {
    /// `GET /api/overview` — returns a cluster-wide monitoring summary:
    /// object totals, queue totals, message rates, listeners, and versions.
    pub async fn overview(&self) -> Result<Overview> {
        self.get("overview", None).await
    }

    /// `GET /api/cluster-name` — returns the cluster's name as a plain
    /// string (the server's `{ "name": ... }` wrapper is unwrapped).
    pub async fn cluster_name(&self) -> Result<String> {
        let cn: ClusterName = self.get("cluster-name", None).await?;
        Ok(cn.name)
    }
}
