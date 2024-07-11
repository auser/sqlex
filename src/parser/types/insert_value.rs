use crate::parser::Rule;
use pest::iterators::Pair;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub enum InsertValue {
    Null,
    Default,
    Text { value: String },
    Number { value: String },
    Identifier { value: String },
}

impl From<Pair<'_, Rule>> for InsertValue {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let uppercase = pair.as_str().trim().to_ascii_uppercase();

        if uppercase.starts_with("NULL") {
            Self::Null
        } else if uppercase.starts_with("DEFAULT") {
            Self::Default
        } else {
            let inner = pair.into_inner().next().unwrap();

            match inner.as_rule() {
                Rule::STRING_LITERAL => Self::Text {
                    value: inner.as_str().trim_matches('\'').to_string(),
                },
                Rule::NUMBER => Self::Number {
                    value: inner.as_str().to_string(),
                },
                Rule::IDENTIFIER => Self::Identifier {
                    value: inner.as_str().to_string(),
                },
                other => panic!("Expected STRING_LITERAL, NUMBER or IDENTIFIER, not {other:?}"),
            }
        }
    }
}

impl Display for InsertValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Null => write!(f, "NULL"),
            Self::Default => write!(f, "DEFAULT"),
            Self::Text { value } => write!(f, "'{value}'"),
            Self::Number { value } => write!(f, "{value}"),
            Self::Identifier { value } => write!(f, "{value}"),
        }
    }
}
