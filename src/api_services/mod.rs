//! Interfaces to the individual APIs specified [here](https://openlibrary.org/developers/api#api-index)

pub mod book_search;

pub(crate) mod apiservice;
pub(crate) use apiservice::api_service;
