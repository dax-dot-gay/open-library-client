//! Types representing search results

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Raw format of paginated results
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaginatedResults<E> {
    /// Total number of possible results
    #[serde(alias = "numFound")]
    pub total: u64,

    /// Whether `total` is exact
    #[serde(alias = "numFoundExact")]
    pub exact_count: bool,

    /// Index of first result
    pub start: u64,

    /// List of results on this page
    #[serde(alias = "docs")]
    pub results: Vec<E>,
}

/// "links" section of a [`PageLinkedRecords`]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageLinkedRecordsLinks {
    /// URL of the current page
    #[serde(rename = "self")]
    pub this: String,

    /// URL of the next page
    pub next: String,

    /// Any extra fields
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>
}

/// Representation of paginated results using the "links" field
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageLinkedRecords<E> {
    /// Navigation links
    pub links: PageLinkedRecordsLinks,

    /// Total number of results
    pub size: u64,

    /// Result array
    pub entries: Vec<E>
}
