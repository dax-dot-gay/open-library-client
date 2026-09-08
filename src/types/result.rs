//! Types representing search results

use serde::{Deserialize, Serialize};

use crate::types::entity::SearchableEntity;

/// Raw format of paginated results
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaginatedResults {
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
    pub results: Vec<SearchableEntity>,
}
