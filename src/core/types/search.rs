//! Abstraction over providing search params

use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::{
    core::types::PaginatedResults,
    params,
};

#[bon::builder(finish_fn = search)]
#[doc(hidden)]
#[allow(missing_docs)]
pub async fn generic_search<R: DeserializeOwned>(
    #[builder(start_fn)] client: crate::Client,

    #[builder(start_fn)] kind: super::SearchResultKind,

    #[builder(start_fn, into)] q: String,

    #[builder(into)] fields: Option<String>,

    #[builder(into)] sort: Option<String>,

    #[builder(into)] lang: Option<String>,

    #[builder(into)] limit: Option<u8>,

    #[builder(into)] offset: Option<u64>,

    #[builder(into)] page: Option<u64>,
) -> crate::Result<PaginatedResults<R>> {
    let url = match kind.clone() {
        super::SearchResultKind::Work => "search.json",
        super::SearchResultKind::Author => "search/authors.json",
        super::SearchResultKind::Subject => "search/subjects.json",
    }
    .to_string();

    let resolved_fields = fields.map(|v| {
        if v.contains("key") {
            v
        } else {
            format!("{},key", v.trim_end_matches(','))
        }
    });

    let result = client
        .request(Method::GET, url)
        .query(params!(
            "q" = q,
            "fields" = resolved_fields,
            "sort" = sort,
            "lang" = lang,
            "limit" = limit,
            "offset" = offset,
            "page" = page
        ))
        .send()
        .await?
        .error_for_status()?;

    let mut raw = result
        .json::<Value>()
        .await?
        .as_object()
        .cloned()
        .unwrap_or(Map::new());
    let updated_results = raw
        .get("docs")
        .cloned()
        .unwrap_or(Value::Array(Vec::new()))
        .as_array()
        .cloned()
        .unwrap_or(Vec::new())
        .into_iter()
        .map(|v| {
            let mut obj = v.as_object().cloned().unwrap_or(Map::new());
            let _ = obj.insert(
                String::from("type"),
                serde_json::to_value(kind.clone()).unwrap(),
            );
            Value::Object(obj)
        })
        .collect::<Vec<_>>();
    let _ = raw.insert(String::from("docs"), Value::Array(updated_results));

    Ok(serde_json::from_value::<PaginatedResults<R>>(
        Value::Object(raw),
    )?)
}
