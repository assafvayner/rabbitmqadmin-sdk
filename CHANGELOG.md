# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-08-03

### Fixed

- `Node::node_type` now deserializes from RabbitMQ's `"type"` JSON key. `GET /api/nodes` returns the
  disc/ram indicator under `"type"`, so the field previously always came back as `None` against a
  real broker ([#1](https://github.com/assafvayner/rabbitmqadmin-sdk/issues/1)).

## [0.1.0] - 2026-08-02

Initial release.

### Added

- `Client` with HTTP Basic auth, base-URL validation, automatic percent-encoding of vhost/queue/
  exchange path segments, and `Client::builder(..).http_client(..)` for a pre-configured
  `reqwest::Client`.
- Monitoring: `overview`, `cluster_name`, nodes, queues, exchanges, bindings, connections, channels,
  and consumers listings with `get_*` variants.
- Administration: declare/delete queues and exchanges, bind/unbind, vhosts, users, permissions,
  policies, operator policies, and runtime parameters.
- Operations: purge queues, `queue_action`, publish and fetch messages over HTTP, close connections
  with `X-Reason`, export/import definitions, six health checks, `list_alarms`, and
  `rebalance_queues`.
- Server-side pagination via `PaginationQuery` and the `Paginated<T>` envelope on large list
  endpoints.
- `Error` enum with `NotFound`, `Api`, `Transport`, `Deserialize`, and `InvalidUrl` variants.

[0.1.1]: https://github.com/assafvayner/rabbitmqadmin-sdk/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/assafvayner/rabbitmqadmin-sdk/releases/tag/v0.1.0
