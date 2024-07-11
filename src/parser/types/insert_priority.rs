use crate::parser::Rule;
use pest::iterators::Pair;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub enum InsertPriority {
    Low,
    Delayed,
    High,
}

impl From<Pair<'_, Rule>> for InsertPriority {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let uppercase = pair.as_str().trim().to_ascii_uppercase();

        if uppercase.contains("LOW_PRIORITY") {
            Self::Low
        } else if uppercase.contains("DELAYED") {
            Self::Delayed
        } else if uppercase.contains("HIGH_PRIORITY") {
            Self::High
        } else {
            panic!("{} not a valid priority level", pair.as_str());
        }
    }
}

impl Display for InsertPriority {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Low => write!(f, "LOW_PRIORITY"),
            Self::Delayed => write!(f, "DELAYED"),
            Self::High => write!(f, "HIGH_PRIORITY"),
        }
    }
}
