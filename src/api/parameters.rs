//! `GET /api/parameters` and related runtime parameter endpoints.

use crate::api::encode_segment;
use crate::{Client, Result};

/// Request body for `PUT /api/parameters/{component}/{vhost}/{name}`.
/// The parameter value is wrapped in a `{"value": ...}` envelope.
#[derive(serde::Serialize)]
struct ParameterSet {
    value: serde_json::Value,
}

impl Client {
    /// `GET /api/parameters` — lists all runtime parameters, or those
    /// of a single component via `GET /api/parameters/{component}` when
    /// `component` is `Some`. Common component values include
    /// `"federation-upstream"` and `"shovel"`.
    pub async fn list_parameters(&self, component: Option<&str>) -> Result<Vec<serde_json::Value>> {
        match component {
            Some(c) => {
                self.get(&format!("parameters/{}", encode_segment(c)), None)
                    .await
            }
            None => self.get("parameters", None).await,
        }
    }

    /// `GET /api/parameters/{component}/{vhost}` — lists all runtime
    /// parameters of a component within a single vhost. The component
    /// and vhost are percent-encoded.
    pub async fn list_parameters_in_vhost(
        &self,
        component: &str,
        vhost: &str,
    ) -> Result<Vec<serde_json::Value>> {
        self.get(
            &format!(
                "parameters/{}/{}",
                encode_segment(component),
                encode_segment(vhost)
            ),
            None,
        )
        .await
    }

    /// `GET /api/parameters/{component}/{vhost}/{name}` — returns a
    /// single runtime parameter. The component, vhost, and name are
    /// percent-encoded.
    pub async fn get_parameter(
        &self,
        component: &str,
        vhost: &str,
        name: &str,
    ) -> Result<serde_json::Value> {
        self.get_ctx(
            &format!(
                "parameters/{}/{}/{}",
                encode_segment(component),
                encode_segment(vhost),
                encode_segment(name)
            ),
            None,
            &format!("parameter '{name}' of component '{component}' in vhost '{vhost}'"),
        )
        .await
    }

    /// `PUT /api/parameters/{component}/{vhost}/{name}` — creates or
    /// updates a runtime parameter; the value is sent as
    /// `{"value": ...}`. Common component values include
    /// `"federation-upstream"` and `"shovel"`. The component, vhost,
    /// and name are percent-encoded.
    pub async fn set_parameter(
        &self,
        component: &str,
        vhost: &str,
        name: &str,
        value: serde_json::Value,
    ) -> Result<()> {
        self.put(
            &format!(
                "parameters/{}/{}/{}",
                encode_segment(component),
                encode_segment(vhost),
                encode_segment(name)
            ),
            &ParameterSet { value },
        )
        .await
    }

    /// `DELETE /api/parameters/{component}/{vhost}/{name}` — deletes a
    /// runtime parameter. The component, vhost, and name are
    /// percent-encoded.
    pub async fn delete_parameter(&self, component: &str, vhost: &str, name: &str) -> Result<()> {
        self.delete_ctx(
            &format!(
                "parameters/{}/{}/{}",
                encode_segment(component),
                encode_segment(vhost),
                encode_segment(name)
            ),
            &format!("parameter '{name}' of component '{component}' in vhost '{vhost}'"),
        )
        .await
    }
}
