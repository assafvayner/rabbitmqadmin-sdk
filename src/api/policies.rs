//! `GET /api/policies`, `GET /api/operator-policies`, and related
//! policy endpoints.

use crate::api::encode_segment;
use crate::types::policy::Policy;
use crate::{Client, Result};

/// Request body for `PUT /api/policies/{vhost}/{name}` and
/// `PUT /api/operator-policies/{vhost}/{name}`. Name and vhost come
/// from the path, so the body carries only the policy attributes.
#[derive(serde::Serialize)]
struct PolicySet {
    pattern: String,
    definition: serde_json::Value,
    priority: i32,
    #[serde(rename = "apply-to")]
    apply_to: String,
}

impl Client {
    /// `GET /api/policies` — lists all policies across all vhosts.
    pub async fn list_policies(&self) -> Result<Vec<Policy>> {
        self.get("policies", None).await
    }

    /// `GET /api/policies/{vhost}` — lists all policies within a single
    /// vhost. The vhost is percent-encoded.
    pub async fn list_policies_in_vhost(&self, vhost: &str) -> Result<Vec<Policy>> {
        self.get(&format!("policies/{}", encode_segment(vhost)), None)
            .await
    }

    /// `GET /api/policies/{vhost}/{name}` — returns a single policy.
    /// The vhost and name are percent-encoded.
    pub async fn get_policy(&self, vhost: &str, name: &str) -> Result<Policy> {
        self.get_ctx(
            &format!(
                "policies/{}/{}",
                encode_segment(vhost),
                encode_segment(name)
            ),
            None,
            &format!("policy '{name}' in vhost '{vhost}'"),
        )
        .await
    }

    /// `PUT /api/policies/{vhost}/{name}` — creates or updates a policy.
    /// The vhost and name are percent-encoded.
    pub async fn set_policy(
        &self,
        vhost: &str,
        name: &str,
        pattern: &str,
        definition: serde_json::Value,
        priority: i32,
        apply_to: &str,
    ) -> Result<()> {
        self.put(
            &format!(
                "policies/{}/{}",
                encode_segment(vhost),
                encode_segment(name)
            ),
            &PolicySet {
                pattern: pattern.to_string(),
                definition,
                priority,
                apply_to: apply_to.to_string(),
            },
        )
        .await
    }

    /// `DELETE /api/policies/{vhost}/{name}` — deletes a policy. The
    /// vhost and name are percent-encoded.
    pub async fn delete_policy(&self, vhost: &str, name: &str) -> Result<()> {
        self.delete_ctx(
            &format!(
                "policies/{}/{}",
                encode_segment(vhost),
                encode_segment(name)
            ),
            &format!("policy '{name}' in vhost '{vhost}'"),
        )
        .await
    }

    /// `GET /api/operator-policies` — lists all operator policies
    /// across all vhosts.
    pub async fn list_operator_policies(&self) -> Result<Vec<Policy>> {
        self.get("operator-policies", None).await
    }

    /// `GET /api/operator-policies/{vhost}` — lists all operator
    /// policies within a single vhost. The vhost is percent-encoded.
    pub async fn list_operator_policies_in_vhost(&self, vhost: &str) -> Result<Vec<Policy>> {
        self.get(
            &format!("operator-policies/{}", encode_segment(vhost)),
            None,
        )
        .await
    }

    /// `GET /api/operator-policies/{vhost}/{name}` — returns a single
    /// operator policy. The vhost and name are percent-encoded.
    pub async fn get_operator_policy(&self, vhost: &str, name: &str) -> Result<Policy> {
        self.get_ctx(
            &format!(
                "operator-policies/{}/{}",
                encode_segment(vhost),
                encode_segment(name)
            ),
            None,
            &format!("operator policy '{name}' in vhost '{vhost}'"),
        )
        .await
    }

    /// `PUT /api/operator-policies/{vhost}/{name}` — creates or updates
    /// an operator policy. The vhost and name are percent-encoded.
    pub async fn set_operator_policy(
        &self,
        vhost: &str,
        name: &str,
        pattern: &str,
        definition: serde_json::Value,
        priority: i32,
        apply_to: &str,
    ) -> Result<()> {
        self.put(
            &format!(
                "operator-policies/{}/{}",
                encode_segment(vhost),
                encode_segment(name)
            ),
            &PolicySet {
                pattern: pattern.to_string(),
                definition,
                priority,
                apply_to: apply_to.to_string(),
            },
        )
        .await
    }

    /// `DELETE /api/operator-policies/{vhost}/{name}` — deletes an
    /// operator policy. The vhost and name are percent-encoded.
    pub async fn delete_operator_policy(&self, vhost: &str, name: &str) -> Result<()> {
        self.delete_ctx(
            &format!(
                "operator-policies/{}/{}",
                encode_segment(vhost),
                encode_segment(name)
            ),
            &format!("operator policy '{name}' in vhost '{vhost}'"),
        )
        .await
    }
}
