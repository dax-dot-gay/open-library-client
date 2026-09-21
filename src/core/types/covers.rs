//! Types for requesting cover & author images

use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// What kind of image to return
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum CoverType {
    #[default]
    Book,
    Author,
}

impl Display for CoverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.clone() {
            CoverType::Book => "b",
            CoverType::Author => "a",
        })
    }
}

/// What key to index by
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum CoverKeyType {
    #[default]
    Olid,
    Id,
    Isbn,
    Oclc,
    Lccn,
}

impl Display for CoverKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            serde_json::to_string(&self)
                .unwrap_or(String::from("olid"))
                .trim_matches('"'),
        )
    }
}

/// How large the returned image should be
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum CoverImageSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl Display for CoverImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.clone() {
            CoverImageSize::Small => "S",
            CoverImageSize::Medium => "M",
            CoverImageSize::Large => "L",
        })
    }
}
