//! Utilities for parsing result data

use std::{fmt::Display, ops::Deref};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub(self) enum KeyedValue {
    Object { key: String },
    Raw(String),
}

impl KeyedValue {
    pub(self) fn key(&self) -> String {
        match self.clone() {
            Self::Object { key } => key,
            Self::Raw(key) => key,
        }
    }
}

macro_rules! KeyType {
    ($key:literal => $name:ident) => {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
        #[serde(try_from = "KeyedValue", into = "KeyedValue")]
        pub(self) struct $name(String);

        impl TryFrom<KeyedValue> for $name {
            type Error = crate::Error;
            fn try_from(value: KeyedValue) -> crate::Result<Self> {
                let parts = value
                    .key()
                    .clone()
                    .split("/")
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>();
                let provided = parts.get(1).cloned().unwrap_or_default();
                let value = parts.get(2).cloned().unwrap_or_default();
                if provided == $key.to_string() {
                    Ok(Self(value))
                } else {
                    Err(crate::Error::key_type(provided, $key))
                }
            }
        }

        impl From<$name> for KeyedValue {
            fn from(value: $name) -> Self {
                Self::Object {
                    key: format!("/{}/{}", $key, value.0),
                }
            }
        }
    };
}

macro_rules! ConcatKeyTypes {
    ($ename:ident = {$($key:literal => $name:ident),*}) => {
        $(KeyType!($key => $name);)*

        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
        #[serde(untagged)]
        pub(self) enum $ename {
            $($name($name)),*
        }
    };
}

ConcatKeyTypes! (KeyTypes = {
    "authors" => AuthorKT,
    "type" => TypeKT,
    "languages" => LanguageKT,
    "works" => WorkKT,
    "series" => SeriesKT,
    "books" => BooksKT,
    "tags" => TagsKT,
    "people" => PeopleKT
});

/// Representation of tagged types
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "value")]
pub(self) enum TaggedValue {
    #[serde(rename = "/type/text")]
    Text(String),
    #[serde(rename = "/type/datetime")]
    Datetime(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub(self) enum UnifiedTaggedValue {
    Raw(String),
    KeyType(KeyTypes),
    TaggedType(TaggedValue),
    Link {
        title: String,
        url: String,
        #[serde(rename = "type")]
        kind: TypeKT,
    },
    AuthorRole {
        author: AuthorKT,
        #[serde(rename = "type")]
        kind: TypeKT,
    },
    Excerpt {
        #[serde(default)]
        pages: Option<String>,

        #[serde(default)]
        comment: Option<String>,
        excerpt: String,
        author: PeopleKT,
    },
    Series {
        series: SeriesKT,
        position: String,
    },
}

/// Wrapper around various OL type indicators
#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(missing_docs)]
#[serde(from = "UnifiedTaggedValue", into = "UnifiedTaggedValue")]
pub enum ExplicitType {
    Raw(String),
    AuthorKey(String),
    TypeKey(String),
    LanguageKey(String),
    WorkKey(String),
    SeriesKey(String),
    BookKey(String),
    TagKey(String),
    PeopleKey(String),
    TextValue(String),
    DatetimeValue(String),
    LinkObject {
        title: String,
        url: String,
    },
    AuthorObject {
        author: String,
    },
    ExcerptObject {
        pages: Option<String>,
        comment: Option<String>,
        excerpt: String,
        author: String,
    },
    SeriesObject {
        series: String,
        position: String,
    },
}

impl From<UnifiedTaggedValue> for ExplicitType {
    fn from(value: UnifiedTaggedValue) -> Self {
        match value {
            UnifiedTaggedValue::Raw(raw) => Self::Raw(raw),
            UnifiedTaggedValue::KeyType(key_types) => match key_types {
                KeyTypes::AuthorKT(AuthorKT(kt)) => Self::AuthorKey(kt),
                KeyTypes::TypeKT(TypeKT(kt)) => Self::TypeKey(kt),
                KeyTypes::LanguageKT(LanguageKT(kt)) => Self::LanguageKey(kt),
                KeyTypes::WorkKT(WorkKT(kt)) => Self::WorkKey(kt),
                KeyTypes::SeriesKT(SeriesKT(kt)) => Self::SeriesKey(kt),
                KeyTypes::BooksKT(BooksKT(kt)) => Self::BookKey(kt),
                KeyTypes::TagsKT(TagsKT(kt)) => Self::TagKey(kt),
                KeyTypes::PeopleKT(PeopleKT(kt)) => Self::PeopleKey(kt),
            },
            UnifiedTaggedValue::TaggedType(tagged_value) => match tagged_value {
                TaggedValue::Text(value) => Self::TextValue(value),
                TaggedValue::Datetime(value) => Self::DatetimeValue(value),
            },
            UnifiedTaggedValue::Link { title, url, .. } => Self::LinkObject { title, url },
            UnifiedTaggedValue::AuthorRole { author, .. } => {
                Self::AuthorObject { author: author.0 }
            }
            UnifiedTaggedValue::Excerpt {
                pages,
                comment,
                excerpt,
                author,
            } => Self::ExcerptObject {
                pages,
                comment,
                excerpt,
                author: author.0,
            },
            UnifiedTaggedValue::Series { series, position } => Self::SeriesObject {
                series: series.0,
                position,
            },
        }
    }
}

impl From<ExplicitType> for UnifiedTaggedValue {
    fn from(value: ExplicitType) -> Self {
        match value {
            ExplicitType::AuthorKey(key) => Self::KeyType(KeyTypes::AuthorKT(AuthorKT(key))),
            ExplicitType::TypeKey(key) => Self::KeyType(KeyTypes::TypeKT(TypeKT(key))),
            ExplicitType::LanguageKey(key) => Self::KeyType(KeyTypes::LanguageKT(LanguageKT(key))),
            ExplicitType::WorkKey(key) => Self::KeyType(KeyTypes::WorkKT(WorkKT(key))),
            ExplicitType::SeriesKey(key) => Self::KeyType(KeyTypes::SeriesKT(SeriesKT(key))),
            ExplicitType::BookKey(key) => Self::KeyType(KeyTypes::BooksKT(BooksKT(key))),
            ExplicitType::TagKey(key) => Self::KeyType(KeyTypes::TagsKT(TagsKT(key))),
            ExplicitType::PeopleKey(key) => Self::KeyType(KeyTypes::PeopleKT(PeopleKT(key))),
            ExplicitType::TextValue(value) => Self::TaggedType(TaggedValue::Text(value)),
            ExplicitType::DatetimeValue(value) => Self::TaggedType(TaggedValue::Datetime(value)),
            ExplicitType::LinkObject { title, url } => Self::Link {
                title,
                url,
                kind: TypeKT("link".to_string()),
            },
            ExplicitType::AuthorObject { author } => Self::AuthorRole {
                author: AuthorKT(author),
                kind: TypeKT("author_role".to_string()),
            },
            ExplicitType::ExcerptObject {
                pages,
                comment,
                excerpt,
                author,
            } => Self::Excerpt {
                pages,
                comment,
                excerpt,
                author: PeopleKT(author),
            },
            ExplicitType::SeriesObject { series, position } => Self::Series {
                series: SeriesKT(series),
                position,
            },
            ExplicitType::Raw(raw) => Self::Raw(raw),
        }
    }
}

/// Sources that can produce an OLID
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub(self) enum OLIDSource {
    Raw(String),
    AuthorRole {
        #[serde(rename = "type")]
        kind: KeyedValue,
        author: KeyedValue,
    },
    AuthorKeyed(AuthorKT),
    WorkKeyed(WorkKT),
}

impl OLIDSource {
    pub(self) fn olid(self) -> OLID {
        match self {
            OLIDSource::Raw(id) => id.into(),
            OLIDSource::AuthorRole { author, .. } => author.key().into(),
            OLIDSource::AuthorKeyed(author) => author.0.into(),
            OLIDSource::WorkKeyed(work) => work.0.into(),
        }
    }
}

/// Normalization wrapper for OLIDs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[serde(from = "OLIDSource", into = "String")]
pub struct OLID(String);

impl Display for OLID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<OLIDSource> for OLID {
    fn from(value: OLIDSource) -> Self {
        value.olid()
    }
}

impl From<String> for OLID {
    fn from(value: String) -> Self {
        if value.contains("/") {
            Self(value.split("/").last().unwrap().to_string())
        } else {
            Self(value)
        }
    }
}

impl From<OLID> for String {
    fn from(value: OLID) -> Self {
        value.0
    }
}

impl Deref for OLID {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
