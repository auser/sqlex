use crate::parser::Rule;
use pest::iterators::Pair;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum DefaultValue {
    Null,
    CurrentTimestamp { value: Option<u32> },
    Text { value: String },
    Number { value: String },
}

impl From<Pair<'_, Rule>> for DefaultValue {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair
            .as_str()
            .to_ascii_uppercase()
            .split_ascii_whitespace()
            .next()
            .expect("keyword")
        {
            "NULL" => Self::Null,
            "CURRENT_TIMESTAMP" => Self::CurrentTimestamp {
                value: pair
                    .into_inner()
                    .next()
                    .map(|p| p.as_str().parse::<u32>().unwrap()),
            },
            _ => {
                let inner = pair.into_inner().next().expect("inner pair");

                match inner.as_rule() {
                    Rule::STRING_LITERAL => Self::Text {
                        value: inner.as_str().trim_matches('\'').to_string(),
                    },
                    Rule::NUMBER => Self::Number {
                        value: inner.as_str().to_string(),
                    },
                    other => panic!("Expected STRING_LITERAL or NUMBER, not rule {other:?}"),
                }
            }
        }
    }
}

impl Display for DefaultValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Null => write!(f, "NULL"),
            Self::CurrentTimestamp { value } => write!(
                f,
                "CURRENT_TIMESTAMP{}",
                if let Some(value) = value {
                    format!(" ({value})")
                } else {
                    "".to_string()
                }
            ),
            Self::Text { value } => write!(f, "'{}'", value),
            Self::Number { value } => write!(f, "{}", value),
        }
    }
}
