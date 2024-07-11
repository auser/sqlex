use crate::parser::Rule;
use pest::iterators::Pair;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Serialize)]
pub struct DropTable {
    pub names: Vec<String>,
    pub temporary: bool,
    pub if_exists: bool,
}

impl From<Pair<'_, Rule>> for DropTable {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let uppercase = pair.as_str().to_ascii_uppercase();
        let temporary = uppercase.contains("TEMPORARY");
        let if_exists = uppercase.contains("EXISTS");
        let names = pair
            .into_inner()
            .filter_map(|p| match p.as_rule() {
                Rule::QUOTED_IDENTIFIER => Some(p.as_str().trim_matches('`').to_string()),
                _ => None,
            })
            .collect::<Vec<String>>();

        Self {
            names,
            temporary,
            if_exists,
        }
    }
}

impl Display for DropTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "DROP{} TABLE{} {}",
            if self.temporary { " TEMPORARY" } else { "" },
            if self.if_exists { " IF EXISTS" } else { "" },
            self.names
                .iter()
                .map(|name| format!("`{name}`"))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;

    #[test]
    fn can_parse_drop_table() {
        let drop_table = DropTable::from(
            MySqlParser::parse(
                Rule::DROP_TABLE,
                "DROP TEMPORARY TABLE IF EXISTS `one`, `two`, `three`;",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(
            drop_table.names,
            vec![
                String::from("one"),
                String::from("two"),
                String::from("three")
            ]
        );
        assert!(drop_table.temporary);
        assert!(drop_table.if_exists);
    }

    #[test]
    fn can_write_drop_table() {
        assert_eq!(
            DropTable {
                names: vec![String::from("one")],
                temporary: false,
                if_exists: true,
            }
            .to_string()
            .as_str(),
            "DROP TABLE IF EXISTS `one`",
        );
    }
}
