//! Types for the `/api/definitions` endpoints (cluster-wide export/import).
//!
//! The [`Definitions`] struct is deliberately typed only where this SDK
//! owns the schema — [`Permission`], [`Policy`], [`Binding`] — and uses
//! [`serde_json::Value`] for collections whose export bodies contain
//! server-generated fields that must round-trip verbatim on import:
//! password hashes and hashing algorithms on users, queue/exchange
//! internals (arguments, recovery state), and vhost metadata. Typing
//! those would silently drop unknown fields on re-serialization, making
//! `export -> import` lossy; `Value` keeps the round-trip lossless.

use serde::{Deserialize, Serialize};

use crate::types::binding::Binding;
use crate::types::permission::Permission;
use crate::types::policy::Policy;

/// A cluster definitions document, as returned by
/// `GET /api/definitions` and accepted by `POST /api/definitions`.
///
/// Fields are optional/defaulted so both exports from older servers and
/// hand-constructed documents deserialize cleanly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Definitions {
    /// RabbitMQ version that produced the export.
    #[serde(default)]
    pub rabbit_version: Option<String>,
    /// Users, as open-ended values (contain server-generated password
    /// hashes that must round-trip verbatim).
    #[serde(default)]
    pub users: Vec<serde_json::Value>,
    /// Virtual hosts, as open-ended values.
    #[serde(default)]
    pub vhosts: Vec<serde_json::Value>,
    /// Permission grants.
    #[serde(default)]
    pub permissions: Vec<Permission>,
    /// Runtime parameters, as open-ended values.
    #[serde(default)]
    pub parameters: Vec<serde_json::Value>,
    /// Global parameters, as open-ended values.
    #[serde(default)]
    pub global_parameters: Vec<serde_json::Value>,
    /// Policies.
    #[serde(default)]
    pub policies: Vec<Policy>,
    /// Queues, as open-ended values (contain server-generated internals
    /// that must round-trip verbatim).
    #[serde(default)]
    pub queues: Vec<serde_json::Value>,
    /// Exchanges, as open-ended values.
    #[serde(default)]
    pub exchanges: Vec<serde_json::Value>,
    /// Bindings.
    #[serde(default)]
    pub bindings: Vec<Binding>,
}
