use crate::parser::Rule;
use pest::iterators::Pair;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Serialize)]
pub struct UseDatabase {
    pub name: String,
}

impl From<Pair<'_, Rule>> for UseDatabase {
    fn from(pair: Pair<'_, Rule>) -> Self {
        Self {
            name: pair
                .into_inner()
                .next()
                .expect("QUOTED_IDENTIFIER")
                .as_str()
                .trim_matches('`')
                .to_string(),
        }
    }
}

impl Display for UseDatabase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "USE `{}`", self.name)
    }
}
