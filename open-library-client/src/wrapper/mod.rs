//! High-level API wrapper & utilities

mod solr;
pub use solr::*;

mod client;
pub use client::*;

pub mod caches;
pub use caches::Cache;