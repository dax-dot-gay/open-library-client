//! Types and wrapper for interacting with the [Search API](https://openlibrary.org/dev/docs/api/search)

use std::collections::HashMap;

use getset::CloneGetters;
use serde::{Deserialize, Serialize};

use crate::{OpenLibraryClient, types::{
    result::PaginatedResults,
    search::{PaginationMode, SOLRQuery, SearchFields, SearchType},
}};
/// Shared struct for query options
#[derive(bon::Builder, Clone, Debug, CloneGetters)]
#[getset(get_clone = "pub")]
pub struct SearchOptions {
    /// The API instance
    #[builder(start_fn)]
    api: OpenLibraryClient,

    /// What type of entity to search for
    #[builder(start_fn)]
    search_type: SearchType,

    /// Query string
    #[builder(into, start_fn)]
    query: SOLRQuery,

    /// Fields to retrieve
    /// Will append `key` and `type` to this list if not present
    /// If empty, will *ONLY* return `key` and `type`
    #[builder(field)]
    fields: Vec<SearchFields>,

    /// Fields to sort by (omitted if empty)
    #[builder(field)]
    sorts: Vec<String>,

    /// ISO-639-1 Language Code for the user's language (omitted if empty)
    language: Option<String>,

    /// Pagination parameters (omitted if empty)
    #[builder(setters(vis = "", name = pagination_internal))]
    pagination: Option<PaginationMode>,
}

impl<S: search_options_builder::State> SearchOptionsBuilder<S> {
    /// Include a specific field in the results
    pub fn field(mut self, field: impl Into<SearchFields>) -> Self {
        self.fields.push(field.into());
        self
    }

    /// Include a set of fields in the results
    pub fn fields(mut self, fields: impl IntoIterator<Item = impl Into<SearchFields>>) -> Self {
        self.fields.extend(fields.into_iter().map(|v| v.into()));
        self
    }

    /// Sort the results by this facet
    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sorts.push(sort.into());
        self
    }

    /// Sort the results by these facets
    pub fn sorts(mut self, sorts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.sorts.extend(sorts.into_iter().map(|v| v.into()));
        self
    }

    /// Paginate by offset
    pub fn offset(
        self,
        offset: usize,
        limit: usize,
    ) -> SearchOptionsBuilder<search_options_builder::SetPagination<S>>
    where
        S::Pagination: search_options_builder::IsUnset,
    {
        self.pagination_internal(PaginationMode::offset(offset, limit))
    }

    /// Paginate by offset
    pub fn page(
        self,
        page: usize,
        page_size: usize,
    ) -> SearchOptionsBuilder<search_options_builder::SetPagination<S>>
    where
        S::Pagination: search_options_builder::IsUnset,
    {
        self.pagination_internal(PaginationMode::page(page, page_size))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct QueryParams {
    pub q: String,
    pub fields: String,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sort: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lang: Option<String>,

    #[serde(flatten)]
    pub pagination: HashMap<String, usize>,
}

impl SearchOptions {
    /// Execute the search operation
    pub async fn search(self) -> crate::Result<PaginatedResults> {
        let mut final_fields = self.fields();
        if !final_fields.contains(&SearchFields::Key) {
            final_fields.push(SearchFields::Key);
        }
        if !final_fields.contains(&SearchFields::Type) {
            final_fields.push(SearchFields::Type);
        }

        let query = QueryParams {
            q: self.query().as_query(),
            fields: final_fields
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(","),
            sort: if self.sorts().len() > 0 {
                Some(self.sorts().join(","))
            } else {
                None
            },
            lang: self.language(),
            pagination: self
                .pagination()
                .map(|v| v.as_parameters())
                .unwrap_or_default(),
        };

        let url = match self.search_type() {
            SearchType::Work => "search.json",
            SearchType::Author => "search/authors.json",
            SearchType::Subject => "search/subjects.json",
        }
        .to_string();

        let request = self
            .api().client()
            .get(format!("https://openlibrary.org/{url}"))
            .query(&query);
        let response = request.send().await?.error_for_status()?;
        let content = response.json::<PaginatedResults>().await?;
        Ok(content)
    }
}
