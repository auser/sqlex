use crate::parser::Rule;
use pest::iterators::Pair;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum OnUpdateValue {
    Cascade,
    CurrentTimestamp { value: Option<u32> },
}

impl From<Pair<'_, Rule>> for OnUpdateValue {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair
            .as_str()
            .to_ascii_uppercase()
            .split_ascii_whitespace()
            .next()
            .expect("keyword")
            .split("(")
            .next()
            .expect("keyword before (")
        {
            "CASCADE" => Self::Cascade,
            "CURRENT_TIMESTAMP" => Self::CurrentTimestamp {
                value: pair
                    .into_inner()
                    .next()
                    .map(|p| p.as_str().parse::<u32>().unwrap()),
            },
            other => panic!("Expected CASCADE or CURRENT_TIMESTAMP, not {other:?}"),
        }
    }
}

impl Display for OnUpdateValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Cascade => write!(f, "CASCADE"),
            Self::CurrentTimestamp { value } => write!(
                f,
                "CURRENT_TIMESTAMP{}",
                if let Some(value) = value {
                    format!(" ({value})")
                } else {
                    "".to_string()
                }
            ),
        }
    }
}
