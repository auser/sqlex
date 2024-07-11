use crate::parser::Rule;
use pest::iterators::Pair;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Note: Only necessary options are currently implemented.
/// For a full list, see https://dev.mysql.com/doc/refman/8.4/en/create-table.html
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum TableOption {
    AutoIncrement { value: u32 },
    CharacterSet { default: bool, value: String },
    Collate { default: bool, value: String },
    Comment { value: String },
    Engine { value: String },
    RowFormat { value: String },
    StatsPersistent { value: String },
}

impl From<Pair<'_, Rule>> for TableOption {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let option = pair.into_inner().next().expect("table option value");

        match option.as_rule() {
            Rule::TABLE_OPT_AUTO_INCREMENT => Self::AutoIncrement {
                value: option
                    .into_inner()
                    .next()
                    .expect("AUTO_INCREMENT NUMBER")
                    .as_str()
                    .parse::<u32>()
                    .unwrap(),
            },
            Rule::TABLE_OPT_CHARSET => Self::CharacterSet {
                default: option
                    .as_str()
                    .trim()
                    .to_ascii_uppercase()
                    .starts_with("DEFAULT"),
                value: option
                    .into_inner()
                    .next()
                    .expect("CHARACTER SET IDENTIFIER")
                    .as_str()
                    .to_string(),
            },
            Rule::TABLE_OPT_COLLATE => Self::Collate {
                default: option
                    .as_str()
                    .trim()
                    .to_ascii_uppercase()
                    .starts_with("DEFAULT"),
                value: option
                    .into_inner()
                    .next()
                    .expect("COLLATE IDENTIFIER")
                    .as_str()
                    .to_string(),
            },
            Rule::TABLE_OPT_COMMENT => Self::Comment {
                value: option
                    .into_inner()
                    .next()
                    .expect("COMMENT STRING_LITERAL")
                    .as_str()
                    .trim_matches('\'')
                    .to_string(),
            },
            Rule::TABLE_OPT_ENGINE => Self::Engine {
                value: option
                    .into_inner()
                    .next()
                    .expect("ENGINE IDENTIFIER")
                    .as_str()
                    .to_string(),
            },
            Rule::TABLE_OPT_ROW_FORMAT => Self::RowFormat {
                value: option
                    .into_inner()
                    .next()
                    .expect("ROW_FORMAT IDENTIFIER")
                    .as_str()
                    .to_string(),
            },
            Rule::TABLE_OPT_STATS_PERSISTENT => Self::StatsPersistent {
                value: option
                    .into_inner()
                    .next()
                    .expect("STATS_PERSISTENT IDENTIFIER or NUMBER")
                    .as_str()
                    .to_string(),
            },
            other => panic!("{other:?} is not a valid table option rule"),
        }
    }
}

impl Display for TableOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::AutoIncrement { value } => write!(f, "AUTO_INCREMENT={}", value),
            Self::CharacterSet { default, value } => write!(
                f,
                "{}CHARSET={}",
                if *default { "DEFAULT " } else { "" },
                value,
            ),
            Self::Collate { default, value } => write!(
                f,
                "{}COLLATE={}",
                if *default { "DEFAULT " } else { "" },
                value,
            ),
            Self::Comment { value } => write!(f, "COMMENT='{}'", value),
            Self::Engine { value } => write!(f, "ENGINE={}", value),
            Self::RowFormat { value } => write!(f, "ROW_FORMAT={}", value),
            Self::StatsPersistent { value } => write!(f, "STATS_PERSISTENT={}", value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;
    use std::ops::Not;

    #[test]
    fn can_parse_table_options() {
        let table_options = MySqlParser::parse(
            Rule::TABLE_OPTIONS,
            "ENGINE=InnoDB AUTO_INCREMENT=1155053 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci STATS_PERSISTENT=0 ROW_FORMAT=DYNAMIC COMMENT='help categories'"
        )
        .expect("Invalid input")
        .next()
        .expect("Unable to parse input")
        .into_inner()
        .map(|p| TableOption::from(p))
        .collect::<Vec<TableOption>>();

        match table_options.get(0).unwrap() {
            TableOption::Engine { value } => assert_eq!(value.as_str(), "InnoDB"),
            _ => panic!("Expected engine"),
        }

        match table_options.get(1).unwrap() {
            TableOption::AutoIncrement { value } => assert_eq!(*value, 1155053),
            _ => panic!("Expected auto increment"),
        }

        match table_options.get(2).unwrap() {
            TableOption::CharacterSet { default, value } => {
                assert!(default);
                assert_eq!(value.as_str(), "utf8mb4");
            }
            _ => panic!("Expected character set"),
        }

        match table_options.get(3).unwrap() {
            TableOption::Collate { default, value } => {
                assert!(default.not());
                assert_eq!(value.as_str(), "utf8mb4_0900_ai_ci");
            }
            _ => panic!("Expected collate"),
        }

        match table_options.get(4).unwrap() {
            TableOption::StatsPersistent { value } => assert_eq!(value.as_str(), "0"),
            _ => panic!("Expected stats persistent"),
        }

        match table_options.get(5).unwrap() {
            TableOption::RowFormat { value } => assert_eq!(value.as_str(), "DYNAMIC"),
            _ => panic!("Expected row format"),
        }

        match table_options.get(6).unwrap() {
            TableOption::Comment { value } => assert_eq!(value.as_str(), "help categories"),
            _ => panic!("Expected comment"),
        }

        assert_eq!(table_options.len(), 7);
    }

    #[test]
    fn can_write_table_options() {
        assert_eq!(
            vec![
                TableOption::Engine { value: String::from("InnoDB") },
                TableOption::AutoIncrement { value: 1155053 },
                TableOption::CharacterSet { default: true, value: String::from("utf8mb4") },
                TableOption::Collate { default: false, value: String::from("utf8mb4_0900_ai_ci") },
                TableOption::StatsPersistent { value: String::from("0") },
                TableOption::RowFormat { value: String::from("DYNAMIC") },
                TableOption::Comment { value: String::from("help categories") },
            ]
            .into_iter()
            .map(|opt| opt.to_string())
            .collect::<Vec<String>>()
            .join(" ")
            .as_str(),
        "ENGINE=InnoDB AUTO_INCREMENT=1155053 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci STATS_PERSISTENT=0 ROW_FORMAT=DYNAMIC COMMENT='help categories'")
    }
}
