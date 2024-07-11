use crate::parser::{types::DatabaseOption, Rule};
use pest::iterators::Pair;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Serialize)]
pub struct CreateDatabase {
    pub name: String,
    pub if_not_exists: bool,
    pub options: Vec<DatabaseOption>,
}

impl From<Pair<'_, Rule>> for CreateDatabase {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let element = inner.next().expect("rule");
        let (name, if_not_exists) = match element.as_rule() {
            Rule::IF_NOT_EXISTS => (
                inner
                    .next()
                    .expect("QUOTED_IDENTIFIER")
                    .as_str()
                    .trim_matches('`')
                    .to_string(),
                true,
            ),
            _ => (element.as_str().trim_matches('`').to_string(), false),
        };
        let options = inner
            .next()
            .map(|p| {
                p.into_inner()
                    .map(|opt| DatabaseOption::from(opt))
                    .collect::<Vec<DatabaseOption>>()
            })
            .unwrap_or_else(|| Vec::new());

        Self {
            name,
            if_not_exists,
            options,
        }
    }
}

impl Display for CreateDatabase {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "CREATE DATABASE{} `{}`{}",
            if self.if_not_exists {
                " IF NOT EXISTS"
            } else {
                ""
            },
            self.name,
            self.options
                .iter()
                .map(|opt| format!(" {opt}"))
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;

    #[test]
    fn can_parse_create_database() {
        let create_database = CreateDatabase::from(
            MySqlParser::parse(
                Rule::CREATE_DATABASE,
                "CREATE DATABASE IF NOT EXISTS `vpay` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT ENCRYPTION='N';",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input")
        );

        assert_eq!(create_database.name.as_str(), "vpay");
        assert!(create_database.if_not_exists);
        assert_eq!(create_database.options.len(), 3);
    }

    #[test]
    fn can_write_create_database() {
        assert_eq!(
            CreateDatabase {
                name: String::from("my_db"),
                if_not_exists: true,
                options: vec![
                    DatabaseOption::CharacterSet { default: true, value: String::from("utf8mb4") },
                    DatabaseOption::Collate { default: false, value: String::from("utf8mb4_0900_ai_ci") },
                    DatabaseOption::Encryption { default: true, value: String::from("N") },
                ]
            }
            .to_string()
            .as_str(),
            "CREATE DATABASE IF NOT EXISTS `my_db` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT ENCRYPTION='N'"
        );
    }
}
