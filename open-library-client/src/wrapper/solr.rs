//! Abstractions over the SOLR query language

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};
use serde::{Deserialize, Serialize};

/// Top-level SOLR query
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SOLR(pub Vec<SOLRTerm>);

impl Deref for SOLR {
    type Target = Vec<SOLRTerm>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SOLR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for SOLR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            self.0
                .clone()
                .into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ")
                .as_str(),
        )
    }
}

/// Delimiter for [`SOLR::Group`]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum SOLRGroupDelimiter {
    #[default]
    Space,
    And,
    Or,
}

impl Display for SOLRGroupDelimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.clone() {
            SOLRGroupDelimiter::Space => " ",
            SOLRGroupDelimiter::And => " AND ",
            SOLRGroupDelimiter::Or => " OR ",
        })
    }
}

/// SOLR terms
/// Not all configurations of this will result in valid output
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "term", content = "params")]
#[allow(missing_docs)]
pub enum SOLRTerm {
    Field {
        key: String,
        clause: Box<SOLRTerm>,
    },
    Wildcard {},
    Literal {
        value: String,
    },
    LiteralPhrase {
        value: String,
    },
    LiteralProximity {
        value: String,
        proximity: u64,
    },
    Require {
        clause: Box<SOLRTerm>,
    },
    Exclude {
        clause: Box<SOLRTerm>,
    },
    Group {
        clauses: Vec<SOLRTerm>,
        delimiter: SOLRGroupDelimiter,
    },
    Range {
        low: Option<String>,
        high: Option<String>,
    },
    Boost {
        clause: Box<SOLRTerm>,
        amount: f64,
    },
    ConstantScore {
        clause: Box<SOLRTerm>,
        score: f64,
    },
}

impl Display for SOLRTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self.clone() {
                SOLRTerm::Field { key, clause } => format!("{key}:{clause}"),
                SOLRTerm::Wildcard {} => "*".to_string(),
                SOLRTerm::Literal { value } => value,
                SOLRTerm::LiteralPhrase { value } => format!("\"{value}\""),
                SOLRTerm::LiteralProximity { value, proximity } => {
                    format!("\"{value}\"~{proximity}")
                }
                SOLRTerm::Require { clause } => format!("+{clause}"),
                SOLRTerm::Exclude { clause } => format!("-{clause}"),
                SOLRTerm::Group { clauses, delimiter } => format!(
                    "({})",
                    clauses
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(&delimiter.to_string())
                ),
                SOLRTerm::Range { low, high } => format!(
                    "[{} TO {}]",
                    low.unwrap_or("*".to_string()),
                    high.unwrap_or("*".to_string())
                ),
                SOLRTerm::Boost { clause, amount } => format!("{clause}^{amount}"),
                SOLRTerm::ConstantScore { clause, score } => format!("{clause}^={score}"),
            }
            .as_str(),
        )
    }
}
