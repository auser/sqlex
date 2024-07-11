use crate::parser::{
    types::{DataType, DefaultValue, OnUpdateValue},
    Rule,
};
use pest::iterators::Pair;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default: Option<DefaultValue>,
    pub on_update: Option<OnUpdateValue>,
    pub auto_increment: bool,
    pub comment: Option<String>,
}

impl Column {
    pub fn new(name: String, data_type: DataType) -> Self {
        Self {
            name,
            data_type,
            nullable: true,
            default: None,
            on_update: None,
            auto_increment: false,
            comment: None,
        }
    }
}

impl From<Pair<'_, Rule>> for Column {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().trim_matches('`').to_string();
        let data_type = DataType::from(inner.next().unwrap());
        let mut column = Column::new(name, data_type);

        for constraint in inner {
            match constraint.as_str().to_uppercase().as_str() {
                "NOT NULL" => column.nullable = false,
                "NULL" => column.nullable = true,
                s if s.starts_with("DEFAULT") => {
                    column.default = Some(DefaultValue::from(
                        constraint.into_inner().next().expect("DEFAULT_VALUE"),
                    ))
                }
                s if s.starts_with("ON") => {
                    column.on_update = Some(OnUpdateValue::from(
                        constraint.into_inner().next().expect("ON_UPDATE_VALUE"),
                    ))
                }
                "AUTO_INCREMENT" => column.auto_increment = true,
                "PRIMARY KEY" => todo!("support for PRIMARY KEY"),
                s if s.starts_with("COMMENT") => {
                    column.comment = Some(
                        constraint
                            .as_str()
                            .get(8..)
                            .unwrap()
                            .trim_matches('\'')
                            .to_string(),
                    )
                }
                other => todo!("support for {}", other),
            }
        }

        column
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "`{}` {}{}{}{}{}{}",
            self.name,
            self.data_type,
            if !self.nullable { " NOT NULL" } else { "" },
            if let Some(ref default) = self.default {
                format!(" DEFAULT {default}")
            } else {
                "".to_string()
            },
            if let Some(ref update) = self.on_update {
                format!(" ON UPDATE {update}")
            } else {
                "".to_string()
            },
            if self.auto_increment {
                " AUTO_INCREMENT"
            } else {
                ""
            },
            if let Some(ref comment) = self.comment {
                format!(" COMMENT '{comment}'")
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
    use std::ops::Not;

    #[test]
    fn can_parse_column() {
        let column = Column::from(
            MySqlParser::parse(Rule::COLUMN_DEFINITION, "`raw_response_json` text,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );

        assert_eq!(column.name.as_str(), "raw_response_json");
        assert!(matches!(
            column.data_type,
            DataType::Text {
                m: None,
                charset_name: None,
                collation_name: None
            }
        ));
        assert!(column.nullable);
        assert!(column.default.is_none());
        assert!(column.auto_increment.not());
        assert!(column.comment.is_none());
    }

    #[test]
    fn can_parse_column_with_default() {
        let column = Column::from(
            MySqlParser::parse(
                Rule::COLUMN_DEFINITION,
                "`settledBusinessDate` date DEFAULT NULL,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(column.name.as_str(), "settledBusinessDate");
        assert!(matches!(column.data_type, DataType::Date,));
        assert!(column.nullable);
        assert!(matches!(column.default, Some(DefaultValue::Null)));
        assert!(column.auto_increment.not());
        assert!(column.comment.is_none());
    }

    #[test]
    fn can_parse_column_not_null() {
        let column = Column::from(
            MySqlParser::parse(Rule::COLUMN_DEFINITION, "`key` varchar(255) NOT NULL,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );

        assert_eq!(column.name.as_str(), "key");
        assert!(matches!(
            column.data_type,
            DataType::Varchar {
                m: Some(255),
                charset_name: None,
                collation_name: None
            }
        ));
        assert!(column.nullable.not());
        assert!(column.default.is_none());
        assert!(column.auto_increment.not());
        assert!(column.comment.is_none());
    }

    #[test]
    fn can_parse_column_with_auto_increment() {
        let column = Column::from(
            MySqlParser::parse(Rule::COLUMN_DEFINITION, "`id` int NOT NULL AUTO_INCREMENT,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );

        assert_eq!(column.name.as_str(), "id");
        assert!(matches!(
            column.data_type,
            DataType::Int {
                m: None,
                unsigned: false,
                zerofill: false,
            }
        ));
        assert!(column.nullable.not());
        assert!(column.default.is_none());
        assert!(column.auto_increment);
        assert!(column.comment.is_none());
    }

    #[test]
    fn can_parse_column_with_comment() {
        let column = Column::from(
            MySqlParser::parse(Rule::COLUMN_DEFINITION, "`pg_monthly_flat_fee` decimal(8,2) DEFAULT '0.00' COMMENT 'i.e. 150.00 dollars per month',")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );

        assert_eq!(column.name.as_str(), "pg_monthly_flat_fee");
        assert!(matches!(
            column.data_type,
            DataType::Decimal {
                m: Some(8),
                d: Some(2),
                unsigned: false,
                zerofill: false,
            }
        ));
        assert!(column.nullable);

        let default_value = match column.default.unwrap() {
            DefaultValue::Text { value } => value,
            _ => panic!("Expected DefaultValue::Text"),
        };

        assert_eq!(default_value.as_str(), "0.00");
        assert!(column.auto_increment.not());
        assert_eq!(
            column.comment.unwrap().as_str(),
            "i.e. 150.00 dollars per month"
        );
    }

    #[test]
    fn can_parse_column_with_unsigned_zerofill_int() {
        let column = Column::from(
            MySqlParser::parse(
                Rule::COLUMN_DEFINITION,
                "`CurrentDisplayCount` int(11) unsigned zerofill NOT NULL DEFAULT '00000000000',",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(column.name.as_str(), "CurrentDisplayCount");
        assert!(matches!(
            column.data_type,
            DataType::Int {
                m: Some(11),
                unsigned: true,
                zerofill: true,
            }
        ));
        assert!(column.nullable.not());

        let default_value = match column.default.unwrap() {
            DefaultValue::Text { value } => value,
            _ => panic!("Expected DefaultValue::Text"),
        };

        assert_eq!(default_value.as_str(), "00000000000");
        assert!(column.auto_increment.not());
        assert!(column.comment.is_none());
    }

    #[test]
    fn can_parse_column_with_on_update() {
        let column = Column::from(
            MySqlParser::parse(
                Rule::COLUMN_DEFINITION,
                "`updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP(6),",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert_eq!(column.name.as_str(), "updated_at");
        assert!(matches!(column.data_type, DataType::DateTime { fsp: None },));
        assert!(column.nullable.not());

        let default_value = match column.default.unwrap() {
            DefaultValue::CurrentTimestamp { value } => value,
            _ => panic!("Expected DefaultValue::Text"),
        };

        assert!(default_value.is_none());

        let on_update_value = match column.on_update.unwrap() {
            OnUpdateValue::CurrentTimestamp { value } => value,
            _ => panic!("Expected OnUpdateValue::CurrentTimestamp"),
        };

        assert_eq!(on_update_value.unwrap(), 6);
        assert!(column.auto_increment.not());
        assert!(column.comment.is_none());
    }

    #[test]
    fn can_write_column() {
        assert_eq!(
            Column {
                name: String::from("raw_response_json"),
                data_type: DataType::Text { m: Some(42), charset_name: Some(String::from("utf8mb4")), collation_name: Some(String::from("utf8mb4_general_ci")) },
                nullable: false,
                default: Some(DefaultValue::Text { value: String::from("Hello, world!") }),
                on_update: Some(OnUpdateValue::CurrentTimestamp { value: Some(6) }),
                auto_increment: true,
                comment: Some(String::from("This is a fully loaded column")),
            }
            .to_string()
            .trim(),
            "`raw_response_json` TEXT (42) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'Hello, world!' ON UPDATE CURRENT_TIMESTAMP (6) AUTO_INCREMENT COMMENT 'This is a fully loaded column'"
        );
    }
}
