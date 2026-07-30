//! Types for the `/api/policies` and `/api/operator-policies` endpoints.

use serde::{Deserialize, Serialize};

/// A policy (or operator policy): a pattern-based rule that applies a
/// definition (a map of arguments) to matching queues, exchanges, or
/// other resources within a virtual host, as returned by
/// `GET /api/policies` and `GET /api/operator-policies`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    /// Name of the policy.
    pub name: String,
    /// Virtual host the policy belongs to.
    pub vhost: String,
    /// Regular expression matching the resource names the policy
    /// applies to.
    pub pattern: String,
    /// What the policy applies to (e.g. `"queues"`, `"exchanges"`,
    /// `"all"`). The Management API serializes this field with a hyphen:
    /// `"apply-to"`.
    #[serde(rename = "apply-to")]
    pub apply_to: String,
    /// Policy definition: an open-ended map of argument keys to values
    /// (e.g. `{"ha-mode": "all"}`).
    pub definition: serde_json::Value,
    /// Policy priority; higher-priority policies take precedence when
    /// several policies match the same resource.
    pub priority: i32,
}
