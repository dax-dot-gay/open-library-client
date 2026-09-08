//! Interfaces to the individual APIs specified [here](https://openlibrary.org/developers/api#api-index)

mod search;
pub use search::SearchAPI;

pub(crate) mod apiservice;
pub(crate) use apiservice::api_service;
