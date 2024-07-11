use crate::parser::Rule;
use pest::iterators::Pair;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub name: Option<String>,
    pub local_column_names: Vec<String>,
    pub foreign_column_names: Vec<String>,
    pub foreign_table_name: String,
    pub on_update: Option<String>,
}

impl From<Pair<'_, Rule>> for ForeignKey {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let name = match inner.peek().expect("Expected an inner rule").as_rule() {
            Rule::INDEX_NAME => inner
                .next()
                .map(|p| p.as_str().trim_matches('`').to_string()),
            _ => None,
        };
        let (local_column_names, foreign_table_name, foreign_column_names, on_update) = inner.fold(
            (Vec::new(), String::new(), Vec::new(), None),
            |(mut local, mut table, mut foreign, mut on_update), pair| {
                match pair.as_rule() {
                    Rule::QUOTED_IDENTIFIER => {
                        table
                            .is_empty()
                            .then(|| &mut local)
                            .unwrap_or_else(|| &mut foreign)
                            .push(pair.as_str().trim_matches('`').to_string());
                    }
                    Rule::TABLE_NAME => {
                        table = pair.as_str().trim_matches('`').to_string();
                    }
                    Rule::FK_ON_UPDATE => {
                        on_update = Some(pair.as_str().split_ascii_whitespace().rev().next().expect("ON UPDATE value").to_string());
                    }
                    rule => {
                        panic!("Expected QUOTED_IDENTIFIER, TABLE_NAME or FK_ON_UPDATE, not not {rule:?}")
                    }
                };

                (local, table, foreign, on_update)
            },
        );

        Self {
            name,
            local_column_names,
            foreign_column_names,
            foreign_table_name,
            on_update,
        }
    }
}

impl Display for ForeignKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{}FOREIGN KEY ({}) REFERENCES `{}` ({}){}",
            if let Some(ref name) = self.name {
                format!("CONSTRAINT `{name}` ")
            } else {
                "".to_string()
            },
            self.local_column_names
                .iter()
                .map(|col| format!("`{col}`"))
                .collect::<Vec<String>>()
                .join(", "),
            self.foreign_table_name,
            self.foreign_column_names
                .iter()
                .map(|col| format!("`{col}`"))
                .collect::<Vec<String>>()
                .join(", "),
            if let Some(ref update) = self.on_update {
                format!(" ON UPDATE {update}")
            } else {
                "".to_string()
            },
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;

    #[test]
    fn can_parse_single_foreign_key_without_name() {
        let foreign_key = ForeignKey::from(
            MySqlParser::parse(
                Rule::FOREIGN_KEY,
                "FOREIGN KEY (`column_id`) REFERENCES `column` (`id`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert!(foreign_key.name.is_none());
        assert_eq!(
            foreign_key.local_column_names,
            vec![String::from("column_id")]
        );
        assert_eq!(foreign_key.foreign_column_names, vec![String::from("id")]);
    }

    #[test]
    fn can_parse_multiple_foreign_key_without_name() {
        let foreign_key = ForeignKey::from(
            MySqlParser::parse(
                Rule::FOREIGN_KEY,
                "FOREIGN KEY (`column_id`, `column_name`) REFERENCES `column` (`id`, `name`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert!(foreign_key.name.is_none());
        assert_eq!(
            foreign_key.local_column_names,
            vec![String::from("column_id"), String::from("column_name")]
        );
        assert_eq!(
            foreign_key.foreign_column_names,
            vec![String::from("id"), String::from("name")]
        );
    }

    #[test]
    fn can_parse_single_foreign_key_with_name() {
        let foreign_key = ForeignKey::from(
            MySqlParser::parse(
                Rule::FOREIGN_KEY,
                "CONSTRAINT `fk_column_id` FOREIGN KEY (`column_id`) REFERENCES `column` (`id`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(foreign_key.name.unwrap().as_str(), "fk_column_id");
        assert_eq!(
            foreign_key.local_column_names,
            vec![String::from("column_id")]
        );
        assert_eq!(foreign_key.foreign_column_names, vec![String::from("id")]);
    }

    #[test]
    fn can_parse_multiple_foreign_key_with_name() {
        let foreign_key = ForeignKey::from(
            MySqlParser::parse(
                Rule::FOREIGN_KEY,
                "CONSTRAINT `fk_column` FOREIGN KEY (`column_id`, `column_name`) REFERENCES `column` (`id`, `name`),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(foreign_key.name.unwrap().as_str(), "fk_column");
        assert_eq!(
            foreign_key.local_column_names,
            vec![String::from("column_id"), String::from("column_name")]
        );
        assert_eq!(
            foreign_key.foreign_column_names,
            vec![String::from("id"), String::from("name")]
        );
    }

    #[test]
    fn can_parse_foreign_key_with_on_update() {
        let foreign_key = ForeignKey::from(
            MySqlParser::parse(
                Rule::FOREIGN_KEY,
                "CONSTRAINT `fk_column` FOREIGN KEY (`column_id`) REFERENCES `column` (`id`) ON UPDATE CASCADE,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(foreign_key.on_update.unwrap().as_str(), "CASCADE");
    }

    #[test]
    fn can_write_foreign_key_without_name() {
        let foreign_key = ForeignKey {
            name: None,
            local_column_names: vec![String::from("column_id"), String::from("column_name")],
            foreign_column_names: vec![String::from("id"), String::from("name")],
            foreign_table_name: String::from("column"),
            on_update: None,
        };

        assert_eq!(
            foreign_key.to_string().as_str(),
            "FOREIGN KEY (`column_id`, `column_name`) REFERENCES `column` (`id`, `name`)",
        );
    }

    #[test]
    fn can_write_foreign_key_with_name() {
        let foreign_key = ForeignKey {
            name: Some(String::from("fk_column")),
            local_column_names: vec![String::from("column_id"), String::from("column_name")],
            foreign_column_names: vec![String::from("id"), String::from("name")],
            foreign_table_name: String::from("column"),
            on_update: None,
        };

        assert_eq!(
            foreign_key.to_string().as_str(),
            "CONSTRAINT `fk_column` FOREIGN KEY (`column_id`, `column_name`) REFERENCES `column` (`id`, `name`)",
        );
    }

    #[test]
    fn can_write_foreign_key_with_on_update() {
        assert_eq!(
            ForeignKey {
                name: Some(String::from("fk_column")),
                local_column_names: vec![String::from("column_id"), String::from("column_name")],
                foreign_column_names: vec![String::from("id"), String::from("name")],
                foreign_table_name: String::from("column"),
                on_update: Some(String::from("CASCADE")),
            }
            .to_string()
            .as_str(),
            "CONSTRAINT `fk_column` FOREIGN KEY (`column_id`, `column_name`) REFERENCES `column` (`id`, `name`) ON UPDATE CASCADE",
        );
    }
}
