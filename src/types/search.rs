//! Type definitions for search queries

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Representation of SOLR Universe types
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SearchType {
    #[serde(rename = "type:work")]
    Work,

    #[serde(rename = "type:author")]
    Author,

    #[serde(rename = "type:subject")]
    Subject,
}

/// Enum defining the SOLR query structure
#[allow(missing_docs)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum SOLR {
    Text {
        value: String,
    },
    Facet {
        key: String,
        value: Box<SOLR>,
    },
    And {
        values: Vec<SOLR>,
    },
    Or {
        values: Vec<SOLR>,
    },
    Not {
        value: Box<SOLR>,
    },
    Wildcard,
    Range {
        from: Option<String>,
        to: Option<String>,
    },
}

impl<T: Display> From<T> for SOLR {
    fn from(value: T) -> Self {
        Self::text(value.to_string())
    }
}

impl SOLR {
    /// Create a [`SOLR::Text`] node
    pub fn text(value: impl Display) -> Self {
        Self::Text {
            value: value.to_string(),
        }
    }

    /// Create a [`SOLR::Facet`] node
    pub fn facet(key: impl Display, value: impl Into<SOLR>) -> Self {
        Self::Facet {
            key: key.to_string(),
            value: Box::new(value.into()),
        }
    }

    /// Create a [`SOLR::And`] node
    pub fn and(nodes: impl IntoIterator<Item = impl Into<SOLR>>) -> Self {
        Self::And {
            values: nodes.into_iter().map(|v| v.into()).collect(),
        }
    }

    /// Create a [`SOLR::Or`] node
    pub fn or(nodes: impl IntoIterator<Item = impl Into<SOLR>>) -> Self {
        Self::And {
            values: nodes.into_iter().map(|v| v.into()).collect(),
        }
    }

    /// Create a [`SOLR::Not`] node
    pub fn not(node: impl Into<SOLR>) -> Self {
        Self::Not {
            value: Box::new(node.into()),
        }
    }

    /// Create a [`SOLR::Range`] node
    /// If either end is [`None`], resolves to "*"
    pub fn range(from: Option<impl Display>, to: Option<impl Display>) -> Self {
        Self::Range {
            from: from.map(|v| v.to_string()),
            to: to.map(|v| v.to_string()),
        }
    }

    /// Create a [`SOLR::Wildcard`] node
    pub fn wildcard() -> Self {
        Self::Wildcard
    }
}

impl SOLR {
    pub fn as_query(&self) -> String {
        match self.clone() {
            SOLR::Text { value } => format!("\"{value}\""),
            SOLR::Facet { key, value } => format!("{key}:{}", value.as_query()),
            SOLR::And { values } => format!(
                "({})",
                values
                    .into_iter()
                    .map(|v| v.as_query())
                    .collect::<Vec<String>>()
                    .join(" AND ")
            ),
            SOLR::Or { values } => format!(
                "({})",
                values
                    .into_iter()
                    .map(|v| v.as_query())
                    .collect::<Vec<String>>()
                    .join(" OR ")
            ),
            SOLR::Not { value } => format!("(NOT {})", value.as_query()),
            SOLR::Wildcard => "*".to_string(),
            SOLR::Range { from, to } => format!(
                "[{} TO {}]",
                from.unwrap_or(String::from("*")),
                to.unwrap_or(String::from("*"))
            ),
        }
    }
}

/// Wrapper for a SOLR query
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SOLRQuery(Vec<SOLR>);

impl<S: Into<SOLR>> FromIterator<S> for SOLRQuery {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        Self(iter.into_iter().map(|v| v.into()).collect())
    }
}

impl SOLRQuery {
    /// Construct a new query object
    pub fn new(components: impl IntoIterator<Item = impl Into<SOLR>>) -> Self {
        Self::from_iter(components)
    }

    /// Convert this query into a query string
    pub fn as_query(&self) -> String {
        self.0
            .clone()
            .into_iter()
            .map(|v| v.as_query())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Display for SOLRQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.as_query())
    }
}
