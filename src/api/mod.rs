//! Internal API helpers and per-resource `impl Client` blocks.

mod nodes;
mod overview;

use crate::{Error, Result};

/// Percent-encode a single URL path segment.
///
/// RabbitMQ resource names (vhosts, queue names, user names, ...) may
/// contain `/`, spaces, and other characters that are not valid in a URL
/// path segment — most notably the default vhost, which is literally `"/"`.
/// This uses `application/x-www-form-urlencoded` byte serialization and then
/// rewrites `+` (form encoding for space) to `%20`, which is the correct
/// encoding inside a path segment.
// Used by resource modules landing in later milestones.
#[allow(dead_code)]
pub(crate) fn encode_segment(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes())
        .collect::<String>()
        .replace('+', "%20")
}

/// Map a reqwest response to a [`Result`] of the response body text,
/// applying the crate's error-mapping rules:
///
/// * 2xx → `Ok(body)`
/// * 404 → [`Error::NotFound`]
/// * any other status → [`Error::Api`]
pub(crate) async fn handle_response(resp: reqwest::Response) -> Result<String> {
    let status = resp.status();
    let body = resp.text().await?;
    if status.is_success() {
        Ok(body)
    } else if status.as_u16() == 404 {
        Err(Error::NotFound(body))
    } else {
        Err(Error::Api {
            status: status.as_u16(),
            reason: body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_segment_encodes_slash() {
        assert_eq!(encode_segment("/"), "%2F");
    }

    #[test]
    fn encode_segment_encodes_space_as_percent20_not_plus() {
        assert_eq!(encode_segment("my queue"), "my%20queue");
    }

    #[test]
    fn encode_segment_at_sign() {
        // url::form_urlencoded::byte_serialize percent-encodes '@' as
        // "%40", so node names like "rabbit@host" are encoded.
        assert_eq!(encode_segment("rabbit@host"), "rabbit%40host");
    }
}
