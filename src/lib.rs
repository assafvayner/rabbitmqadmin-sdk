//! Async Rust SDK for the RabbitMQ Management HTTP API.
//!
//! This crate provides a strongly-typed async client for the RabbitMQ
//! Management Plugin HTTP API (`/api/*`, typically port 15672). It supports
//! monitoring (overview, nodes, queues, exchanges, connections, ...) as well
//! as administrative actions (declaring and deleting queues, exchanges,
//! bindings, vhosts, users, policies, purging queues, and more).
