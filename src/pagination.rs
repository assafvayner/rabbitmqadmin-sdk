//! Pagination support for list endpoints.
//!
//! RabbitMQ list endpoints support server-side pagination via query
//! parameters (`page`, `page_size`, `name`, `use_regex`, plus
//! `pagination=true`) and then respond with a [`Paginated`] envelope instead
//! of a bare JSON array.

use serde::{Deserialize, Serialize};

/// Query parameters for paginated list endpoints.
///
/// All fields are optional; unset fields are simply omitted from the query
/// string. When a `PaginationQuery` is supplied to a list method, the
/// `pagination=true` parameter is always added so the server responds with a
/// [`Paginated`] envelope.
#[derive(Debug, Default, Clone, Serialize)]
pub struct PaginationQuery {
    /// 1-based page number to fetch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    /// Filter by name (literal string, or regular expression when
    /// `use_regex` is true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Interpret `name` as a regular expression.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_regex: Option<bool>,
}

impl PaginationQuery {
    /// Convert to URL query string key/value pairs.
    ///
    /// Always includes `("pagination", "true")` so the server returns a
    /// paginated response envelope.
    pub(crate) fn to_pairs(&self) -> Vec<(String, String)> {
        let mut v = Vec::new();
        if let Some(p) = self.page {
            v.push(("page".into(), p.to_string()));
        }
        if let Some(ps) = self.page_size {
            v.push(("page_size".into(), ps.to_string()));
        }
        if let Some(n) = &self.name {
            v.push(("name".into(), n.clone()));
        }
        if let Some(r) = self.use_regex {
            v.push(("use_regex".into(), r.to_string()));
        }
        v.push(("pagination".into(), "true".into()));
        v
    }
}

/// Envelope returned by paginated list endpoints.
#[derive(Debug, Clone, Deserialize)]
pub struct Paginated<T> {
    /// The items on the current page.
    pub items: Vec<T>,
    /// Current page number (1-based).
    pub page: u32,
    /// Number of items per page.
    pub page_size: u32,
    /// Total number of items (before name filtering).
    pub total_count: u64,
    /// Number of items matching the current filter.
    pub filtered_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_pairs_with_all_fields_set() {
        let q = PaginationQuery {
            page: Some(2),
            page_size: Some(50),
            name: Some("orders.*".into()),
            use_regex: Some(true),
        };
        let pairs = q.to_pairs();
        assert!(pairs.contains(&("page".to_string(), "2".to_string())));
        assert!(pairs.contains(&("page_size".to_string(), "50".to_string())));
        assert!(pairs.contains(&("name".to_string(), "orders.*".to_string())));
        assert!(pairs.contains(&("use_regex".to_string(), "true".to_string())));
        assert!(pairs.contains(&("pagination".to_string(), "true".to_string())));
        assert_eq!(pairs.len(), 5);
    }

    #[test]
    fn to_pairs_with_no_fields_set_still_enables_pagination() {
        let q = PaginationQuery::default();
        let pairs = q.to_pairs();
        assert_eq!(pairs, vec![("pagination".to_string(), "true".to_string())]);
    }

    #[test]
    fn paginated_deserializes_from_json() {
        let body = serde_json::json!({
            "items": [{"name": "q1"}, {"name": "q2"}],
            "page": 1,
            "page_size": 100,
            "total_count": 5,
            "filtered_count": 2
        });
        #[derive(serde::Deserialize)]
        struct Item {
            name: String,
        }
        let p: Paginated<Item> = serde_json::from_value(body).unwrap();
        assert_eq!(p.items.len(), 2);
        assert_eq!(p.items[0].name, "q1");
        assert_eq!(p.items[1].name, "q2");
        assert_eq!(p.page, 1);
        assert_eq!(p.page_size, 100);
        assert_eq!(p.total_count, 5);
        assert_eq!(p.filtered_count, 2);
    }
}
