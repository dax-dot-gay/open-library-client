//! Type definitions for search queries

use std::{collections::HashMap, fmt::Display};

use convert_case::{Case, Casing};
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
    Number {
        value: i128
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
        if let Ok(parsed) = value.to_string().parse::<i128>() {
            Self::number(parsed)
        } else {
            Self::text(value.to_string())
        }
    }
}

impl SOLR {
    /// Create a [`SOLR::Text`] node
    pub fn text(value: impl Display) -> Self {
        Self::Text {
            value: value.to_string(),
        }
    }
    
    /// Create a [`SOLR::Number`] node
    pub fn number(value: impl Into<i128>) -> Self {
        Self::Number { value: value.into() }
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
    /// Convert this node and any children to its SOLR representation
    pub fn as_query(&self) -> String {
        match self.clone() {
            SOLR::Text { value } => format!("\"{value}\""),
            SOLR::Number { value } => value.to_string(),
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

impl<T: Into<String>> From<T> for SOLRQuery {
    fn from(value: T) -> Self {
        Self(vec![SOLR::text(value.into())])
    }
}

/// Enum representing special search fields and a variant for all others
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SearchFields {
    /// Request all fields
    #[serde(rename = "*")]
    Wildcard,

    /// Request availability
    Availability,

    /// Request entity key
    Key,

    /// Request entity type
    Type,

    /// Request other fields
    #[serde(untagged)]
    Named(String)
}

impl SearchFields {
    /// Construct a new field item
    pub fn new(field: impl Into<String>) -> Self {
        Self::from(field.into())
    }
}

impl Display for SearchFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(serde_json::to_string(&self).unwrap().trim_matches('"'))
    }
}

impl<T: Into<String>> From<T> for SearchFields {
    fn from(value: T) -> Self {
        let normalized = value.into().trim().to_case(Case::Snake);
        match normalized.as_str() {
            "*" => Self::Wildcard,
            "availability" => Self::Availability,
            "key" => Self::Key,
            "type" => Self::Type,
            other => Self::Named(other.to_string())
        }
    }
}

/// Pagination parameters
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "mode")]
pub enum PaginationMode {
    /// Offset by `offset` results, then return `max_results`
    Offset {
        /// Amount of entities to offset by
        offset: usize,

        /// Maximum number of results to return
        max_results: usize
    },

    /// Use standard pagination
    Page {
        /// Page number, indexed from 1
        page: usize,

        /// Number of results per page
        page_size: usize
    }
}

impl PaginationMode {
    /// Construct [`PaginationMode::Offset`]
    pub fn offset(offset: usize, max_results: usize) -> Self {
        Self::Offset { offset, max_results }
    }

    /// Construct [`PaginationMode::Page`]
    pub fn page(page: usize, page_size: usize) -> Self {
        Self::Page { page, page_size }
    }

    pub(crate) fn as_parameters(&self) -> HashMap<String, usize> {
        match self.clone() {
            PaginationMode::Offset { offset, max_results } => HashMap::<String, usize>::from([("offset".to_string(), offset), ("limit".to_string(), max_results)]),
            PaginationMode::Page { page, page_size } => HashMap::<String, usize>::from([("page".to_string(), page), ("limit".to_string(), page_size)]),
        }
    }
}

