use crate::parser::Rule;
use pest::iterators::Pair;
use serde::Serialize;
use std::fmt::{Display, Formatter, Result as FmtResult};
use strum_macros::IntoStaticStr;

#[derive(Debug, Clone, Serialize, IntoStaticStr)]
#[serde(tag = "type")]
#[strum(serialize_all = "UPPERCASE")]
pub enum DataType {
    TinyInt {
        m: Option<u32>,
        unsigned: bool,
        zerofill: bool,
    },
    SmallInt {
        m: Option<u32>,
        unsigned: bool,
        zerofill: bool,
    },
    MediumInt {
        m: Option<u32>,
        unsigned: bool,
        zerofill: bool,
    },
    Int {
        m: Option<u32>,
        unsigned: bool,
        zerofill: bool,
    },
    BigInt {
        m: Option<u32>,
        unsigned: bool,
        zerofill: bool,
    },
    Decimal {
        m: Option<u32>,
        d: Option<u32>,
        unsigned: bool,
        zerofill: bool,
    },
    Float {
        m: Option<u32>,
        d: Option<u32>,
        unsigned: bool,
        zerofill: bool,
    },
    Double {
        m: Option<u32>,
        d: Option<u32>,
        unsigned: bool,
        zerofill: bool,
    },
    Bit {
        m: Option<u32>,
    },
    Date,
    DateTime {
        fsp: Option<u32>,
    },
    Timestamp {
        fsp: Option<u32>,
    },
    Time {
        fsp: Option<u32>,
    },
    Year {
        m: Option<u32>,
    },
    Char {
        m: Option<u32>,
        charset_name: Option<String>,
        collation_name: Option<String>,
    },
    Varchar {
        m: Option<u32>,
        charset_name: Option<String>,
        collation_name: Option<String>,
    },
    Binary {
        m: Option<u32>,
    },
    Varbinary {
        m: u32,
    },
    Blob {
        m: Option<u32>,
    },
    TinyBlob,
    MediumBlob,
    LongBlob,
    Text {
        m: Option<u32>,
        charset_name: Option<String>,
        collation_name: Option<String>,
    },
    TinyText {
        charset_name: Option<String>,
        collation_name: Option<String>,
    },
    MediumText {
        charset_name: Option<String>,
        collation_name: Option<String>,
    },
    LongText {
        charset_name: Option<String>,
        collation_name: Option<String>,
    },
    Enum {
        values: Vec<String>,
        charset_name: Option<String>,
        collation_name: Option<String>,
    },
    Set {
        values: Vec<String>,
        charset_name: Option<String>,
        collation_name: Option<String>,
    },
    Json,
}

impl From<Pair<'_, Rule>> for DataType {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let type_name = pair
            .as_str()
            .split(' ')
            .next()
            .unwrap()
            .split('(')
            .next()
            .unwrap()
            .trim()
            .to_uppercase();
        let mut inner = pair.into_inner();

        match type_name.as_str() {
            "TINYINT" | "SMALLINT" | "MEDIUMINT" | "INT" | "INTEGER" | "BIGINT" => {
                let mut m = None;
                let mut unsigned = false;
                let mut zerofill = false;

                inner.for_each(|p| match p.as_rule() {
                    Rule::NUMBER => m = Some(p.as_str().parse::<u32>().unwrap()),
                    Rule::UNSIGNED => unsigned = true,
                    Rule::ZEROFILL => zerofill = true,
                    other => panic!("Expected NUMBER, UNSIGNED or ZEROFILL, not {other:?}"),
                });

                match type_name.as_str() {
                    "TINYINT" => DataType::TinyInt {
                        m,
                        unsigned,
                        zerofill,
                    },
                    "SMALLINT" => DataType::SmallInt {
                        m,
                        unsigned,
                        zerofill,
                    },
                    "MEDIUMINT" => DataType::MediumInt {
                        m,
                        unsigned,
                        zerofill,
                    },
                    "INT" | "INTEGER" => DataType::Int {
                        m,
                        unsigned,
                        zerofill,
                    },
                    "BIGINT" => DataType::BigInt {
                        m,
                        unsigned,
                        zerofill,
                    },
                    _ => unreachable!(),
                }
            }
            "DECIMAL" | "NUMERIC" | "FLOAT" | "DOUBLE" => {
                let mut m = None;
                let mut d = None;
                let mut unsigned = false;
                let mut zerofill = false;

                inner.for_each(|p| match p.as_rule() {
                    Rule::NUMBER => {
                        let ptr = if m.is_none() { &mut m } else { &mut d };

                        *ptr = Some(p.as_str().parse::<u32>().unwrap());
                    }
                    Rule::UNSIGNED => unsigned = true,
                    Rule::ZEROFILL => zerofill = true,
                    other => panic!("Expected NUMBER, UNSIGNED or ZEROFILL, not {other:?}"),
                });

                match type_name.as_str() {
                    "DECIMAL" | "NUMERIC" => DataType::Decimal {
                        m,
                        d,
                        unsigned,
                        zerofill,
                    },
                    "FLOAT" => DataType::Float {
                        m,
                        d,
                        unsigned,
                        zerofill,
                    },
                    "DOUBLE" => DataType::Double {
                        m,
                        d,
                        unsigned,
                        zerofill,
                    },
                    _ => unreachable!(),
                }
            }
            "BIT" => DataType::Bit {
                m: inner.next().map(|p| p.as_str().parse::<u32>().unwrap()),
            },
            "DATE" | "DATETIME" | "TIMESTAMP" | "TIME" => {
                if type_name.as_str() == "DATE" {
                    DataType::Date
                } else {
                    let fsp = inner.next().map(|p| p.as_str().parse::<u32>().unwrap());

                    match type_name.as_str() {
                        "DATETIME" => DataType::DateTime { fsp },
                        "TIMESTAMP" => DataType::Timestamp { fsp },
                        "TIME" => DataType::Time { fsp },
                        _ => unreachable!(),
                    }
                }
            }
            "YEAR" => DataType::Year {
                m: inner.next().map(|p| p.as_str().parse::<u32>().unwrap()),
            },
            "CHAR" | "VARCHAR" => {
                let mut m = None;
                let mut charset_name = None;
                let mut collation_name = None;

                inner.for_each(|p| match p.as_rule() {
                    Rule::NUMBER => m = Some(p.as_str().parse::<u32>().unwrap()),
                    Rule::CHARACTER_SET => {
                        charset_name = Some(
                            p.into_inner()
                                .next()
                                .expect("IDENTIFIER")
                                .as_str()
                                .to_string(),
                        )
                    }
                    Rule::COLLATE => {
                        collation_name = Some(
                            p.into_inner()
                                .next()
                                .expect("IDENTIFIER")
                                .as_str()
                                .to_string(),
                        )
                    }
                    other => panic!("Expected NUMBER, CHARACTER SET or COLLATE, not {other:?}"),
                });

                match type_name.as_str() {
                    "CHAR" => DataType::Char {
                        m,
                        charset_name,
                        collation_name,
                    },
                    "VARCHAR" => DataType::Varchar {
                        m,
                        charset_name,
                        collation_name,
                    },
                    _ => unreachable!(),
                }
            }
            "BINARY" => DataType::Binary {
                m: inner.next().map(|p| p.as_str().parse::<u32>().unwrap()),
            },
            "VARBINARY" => DataType::Varbinary {
                m: inner.next().unwrap().as_str().parse::<u32>().unwrap(),
            },
            "BLOB" => DataType::Blob {
                m: inner.next().map(|p| p.as_str().parse::<u32>().unwrap()),
            },
            "TINYBLOB" => DataType::TinyBlob,
            "MEDIUMBLOB" => DataType::MediumBlob,
            "LONGBLOB" => DataType::LongBlob,
            "TEXT" => {
                let mut m = None;
                let mut charset_name = None;
                let mut collation_name = None;

                inner.for_each(|p| match p.as_rule() {
                    Rule::NUMBER => m = Some(p.as_str().parse::<u32>().unwrap()),
                    Rule::CHARACTER_SET => {
                        charset_name = Some(
                            p.into_inner()
                                .next()
                                .expect("IDENTIFIER")
                                .as_str()
                                .to_string(),
                        )
                    }
                    Rule::COLLATE => {
                        collation_name = Some(
                            p.into_inner()
                                .next()
                                .expect("IDENTIFIER")
                                .as_str()
                                .to_string(),
                        )
                    }
                    other => panic!("Expected NUMBER, CHARACTER SET or COLLATE, not {other:?}"),
                });

                DataType::Text {
                    m,
                    charset_name,
                    collation_name,
                }
            }
            "TINYTEXT" | "MEDIUMTEXT" | "LONGTEXT" => {
                let mut charset_name = None;
                let mut collation_name = None;

                inner.for_each(|p| match p.as_rule() {
                    Rule::CHARACTER_SET => {
                        charset_name = Some(
                            p.into_inner()
                                .next()
                                .expect("IDENTIFIER")
                                .as_str()
                                .to_string(),
                        )
                    }
                    Rule::COLLATE => {
                        collation_name = Some(
                            p.into_inner()
                                .next()
                                .expect("IDENTIFIER")
                                .as_str()
                                .to_string(),
                        )
                    }
                    other => panic!("Expected CHARACTER SET or COLLATE, not {other:?}"),
                });

                match type_name.as_str() {
                    "TINYTEXT" => DataType::TinyText {
                        charset_name,
                        collation_name,
                    },
                    "MEDIUMTEXT" => DataType::MediumText {
                        charset_name,
                        collation_name,
                    },
                    "LONGTEXT" => DataType::LongText {
                        charset_name,
                        collation_name,
                    },
                    _ => unreachable!(),
                }
            }
            "ENUM" | "SET" => {
                let mut values = Vec::new();
                let mut charset_name = None;
                let mut collation_name = None;

                inner.for_each(|p| match p.as_rule() {
                    Rule::STRING_LITERAL => values.push(p.as_str().trim_matches('\'').to_string()),
                    Rule::CHARACTER_SET => {
                        charset_name = Some(
                            p.into_inner()
                                .next()
                                .expect("IDENTIFIER")
                                .as_str()
                                .to_string(),
                        )
                    }
                    Rule::COLLATE => {
                        collation_name = Some(
                            p.into_inner()
                                .next()
                                .expect("IDENTIFIER")
                                .as_str()
                                .to_string(),
                        )
                    }
                    other => {
                        panic!("Expected STRING_LITERAL, CHARACTER_SET or COLLATE, not {other:?}")
                    }
                });

                match type_name.as_str() {
                    "ENUM" => DataType::Enum {
                        values,
                        charset_name,
                        collation_name,
                    },
                    "SET" => DataType::Set {
                        values,
                        charset_name,
                        collation_name,
                    },
                    _ => unreachable!(),
                }
            }
            "JSON" => DataType::Json,
            other => unimplemented!("Data type {} not implemented", other),
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let name: &'static str = self.into();

        match self {
            Self::TinyInt {
                m,
                unsigned,
                zerofill,
            }
            | Self::SmallInt {
                m,
                unsigned,
                zerofill,
            }
            | Self::MediumInt {
                m,
                unsigned,
                zerofill,
            }
            | Self::Int {
                m,
                unsigned,
                zerofill,
            }
            | Self::BigInt {
                m,
                unsigned,
                zerofill,
            } => write!(
                f,
                "{}{}{}{}",
                name,
                if let Some(m) = m {
                    format!(" ({m})")
                } else {
                    "".to_string()
                },
                if *unsigned { " UNSIGNED" } else { "" },
                if *zerofill { " ZEROFILL" } else { "" },
            ),
            Self::Decimal {
                m,
                d,
                unsigned,
                zerofill,
            }
            | Self::Float {
                m,
                d,
                unsigned,
                zerofill,
            }
            | Self::Double {
                m,
                d,
                unsigned,
                zerofill,
            } => write!(
                f,
                "{}{}{}{}",
                name,
                match (m, d) {
                    (None, None) => "".to_string(),
                    (Some(m), None) => format!(" ({m})"),
                    (Some(m), Some(d)) => format!(" ({m}, {d})"),
                    (None, Some(_)) => panic!("m must be defined"),
                },
                if *unsigned { " UNSIGNED" } else { "" },
                if *zerofill { " ZEROFILL" } else { "" },
            ),
            Self::Bit { m } => write!(
                f,
                "{}{}",
                name,
                if let Some(m) = m {
                    format!(" ({m})")
                } else {
                    "".to_string()
                },
            ),
            Self::Date => write!(f, "{}", name),
            Self::DateTime { fsp } | Self::Timestamp { fsp } | Self::Time { fsp } => write!(
                f,
                "{}{}",
                name,
                if let Some(fsp) = fsp {
                    format!(" ({fsp})")
                } else {
                    "".to_string()
                },
            ),
            Self::Year { m } => write!(
                f,
                "{}{}",
                name,
                if let Some(m) = m {
                    format!(" ({m})")
                } else {
                    "".to_string()
                }
            ),
            Self::Char {
                m,
                charset_name,
                collation_name,
            }
            | Self::Varchar {
                m,
                charset_name,
                collation_name,
            } => write!(
                f,
                "{}{}{}{}",
                name,
                if let Some(m) = m {
                    format!(" ({m})")
                } else {
                    "".to_string()
                },
                if let Some(charset_name) = charset_name {
                    format!(" CHARACTER SET {}", charset_name)
                } else {
                    "".to_string()
                },
                if let Some(collation_name) = collation_name {
                    format!(" COLLATE {}", collation_name)
                } else {
                    "".to_string()
                },
            ),
            Self::Binary { m } => write!(
                f,
                "{}{}",
                name,
                if let Some(m) = m {
                    format!(" ({m})")
                } else {
                    "".to_string()
                },
            ),
            Self::Varbinary { m } => write!(f, "{} ({})", name, m),
            Self::Blob { m } => write!(
                f,
                "{}{}",
                name,
                if let Some(m) = m {
                    format!(" ({m})")
                } else {
                    "".to_string()
                },
            ),
            Self::TinyBlob | Self::MediumBlob | Self::LongBlob => write!(f, "{}", name),
            Self::Text {
                m,
                charset_name,
                collation_name,
            } => write!(
                f,
                "{}{}{}{}",
                name,
                if let Some(m) = m {
                    format!(" ({m})")
                } else {
                    "".to_string()
                },
                if let Some(charset_name) = charset_name {
                    format!(" CHARACTER SET {}", charset_name)
                } else {
                    "".to_string()
                },
                if let Some(collation_name) = collation_name {
                    format!(" COLLATE {}", collation_name)
                } else {
                    "".to_string()
                },
            ),
            Self::TinyText {
                charset_name,
                collation_name,
            }
            | Self::MediumText {
                charset_name,
                collation_name,
            }
            | Self::LongText {
                charset_name,
                collation_name,
            } => write!(
                f,
                "{}{}{}",
                name,
                if let Some(charset_name) = charset_name {
                    format!(" CHARACTER SET {}", charset_name)
                } else {
                    "".to_string()
                },
                if let Some(collation_name) = collation_name {
                    format!(" COLLATE {}", collation_name)
                } else {
                    "".to_string()
                },
            ),
            Self::Enum {
                values,
                charset_name,
                collation_name,
            }
            | Self::Set {
                values,
                charset_name,
                collation_name,
            } => write!(
                f,
                "{} ({}){}{}",
                name,
                values
                    .iter()
                    .map(|value| format!("'{value}'"))
                    .collect::<Vec<String>>()
                    .join(", ")
                    .to_string(),
                if let Some(charset_name) = charset_name {
                    format!(" CHARACTER SET {}", charset_name)
                } else {
                    "".to_string()
                },
                if let Some(collation_name) = collation_name {
                    format!(" COLLATE {}", collation_name)
                } else {
                    "".to_string()
                },
            ),
            Self::Json => write!(f, "{}", name),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;

    #[test]
    fn can_parse_tinyint_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TINYINT,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::TinyInt {
                m: None,
                unsigned: false,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_tinyint() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TINYINT (4),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::TinyInt {
                m: Some(4),
                unsigned: false,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_tinyint_unsigned() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TINYINT (4) UNSIGNED,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::TinyInt {
                m: Some(4),
                unsigned: true,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_tinyint_unsigned_zerofill() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TINYINT (4) UNSIGNED ZEROFILL,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::TinyInt {
                m: Some(4),
                unsigned: true,
                zerofill: true,
            }
        ));
    }

    #[test]
    fn can_write_tinyint_default() {
        assert_eq!(
            DataType::TinyInt {
                m: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "TINYINT"
        );
    }

    #[test]
    fn can_write_tinyint() {
        assert_eq!(
            DataType::TinyInt {
                m: Some(4),
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "TINYINT (4)"
        );
    }

    #[test]
    fn can_write_tinyint_unsigned() {
        assert_eq!(
            DataType::TinyInt {
                m: Some(4),
                unsigned: true,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "TINYINT (4) UNSIGNED"
        );
    }

    #[test]
    fn can_write_tinyint_unsigned_zerofill() {
        assert_eq!(
            DataType::TinyInt {
                m: Some(4),
                unsigned: true,
                zerofill: true
            }
            .to_string()
            .as_str(),
            "TINYINT (4) UNSIGNED ZEROFILL"
        );
    }

    #[test]
    fn can_parse_smallint_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "SMALLINT,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::SmallInt {
                m: None,
                unsigned: false,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_smallint() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "SMALLINT (4),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::SmallInt {
                m: Some(4),
                unsigned: false,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_smallint_unsigned() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "SMALLINT (4) UNSIGNED,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::SmallInt {
                m: Some(4),
                unsigned: true,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_smallint_unsigned_zerofill() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "SMALLINT (4) UNSIGNED ZEROFILL,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::SmallInt {
                m: Some(4),
                unsigned: true,
                zerofill: true,
            }
        ));
    }

    #[test]
    fn can_write_smallint_default() {
        assert_eq!(
            DataType::SmallInt {
                m: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "SMALLINT"
        );
    }

    #[test]
    fn can_write_smallint() {
        assert_eq!(
            DataType::SmallInt {
                m: Some(4),
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "SMALLINT (4)"
        );
    }

    #[test]
    fn can_write_smallint_unsigned() {
        assert_eq!(
            DataType::SmallInt {
                m: Some(4),
                unsigned: true,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "SMALLINT (4) UNSIGNED"
        );
    }

    #[test]
    fn can_write_smallint_unsigned_zerofill() {
        assert_eq!(
            DataType::SmallInt {
                m: Some(4),
                unsigned: true,
                zerofill: true
            }
            .to_string()
            .as_str(),
            "SMALLINT (4) UNSIGNED ZEROFILL"
        );
    }

    #[test]
    fn can_parse_mediumint_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "MEDIUMINT,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::MediumInt {
                m: None,
                unsigned: false,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_mediumint() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "MEDIUMINT (4),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::MediumInt {
                m: Some(4),
                unsigned: false,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_mediumint_unsigned() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "MEDIUMINT (4) UNSIGNED,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::MediumInt {
                m: Some(4),
                unsigned: true,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_mediumint_unsigned_zerofill() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "MEDIUMINT (4) UNSIGNED ZEROFILL,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::MediumInt {
                m: Some(4),
                unsigned: true,
                zerofill: true,
            }
        ));
    }

    #[test]
    fn can_write_mediumint_default() {
        assert_eq!(
            DataType::MediumInt {
                m: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "MEDIUMINT"
        );
    }

    #[test]
    fn can_write_mediumint() {
        assert_eq!(
            DataType::MediumInt {
                m: Some(4),
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "MEDIUMINT (4)"
        );
    }

    #[test]
    fn can_write_mediumint_unsigned() {
        assert_eq!(
            DataType::MediumInt {
                m: Some(4),
                unsigned: true,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "MEDIUMINT (4) UNSIGNED"
        );
    }

    #[test]
    fn can_write_mediumint_unsigned_zerofill() {
        assert_eq!(
            DataType::MediumInt {
                m: Some(4),
                unsigned: true,
                zerofill: true
            }
            .to_string()
            .as_str(),
            "MEDIUMINT (4) UNSIGNED ZEROFILL"
        );
    }

    #[test]
    fn can_parse_int_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "INT,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Int {
                m: None,
                unsigned: false,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_int() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "INT (4),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Int {
                m: Some(4),
                unsigned: false,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_int_unsigned() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "INT (4) UNSIGNED,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Int {
                m: Some(4),
                unsigned: true,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_int_unsigned_zerofill() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "INT (4) UNSIGNED ZEROFILL,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Int {
                m: Some(4),
                unsigned: true,
                zerofill: true,
            }
        ));
    }

    #[test]
    fn can_write_int_default() {
        assert_eq!(
            DataType::Int {
                m: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "INT"
        );
    }

    #[test]
    fn can_write_int() {
        assert_eq!(
            DataType::Int {
                m: Some(4),
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "INT (4)"
        );
    }

    #[test]
    fn can_write_int_unsigned() {
        assert_eq!(
            DataType::Int {
                m: Some(4),
                unsigned: true,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "INT (4) UNSIGNED"
        );
    }

    #[test]
    fn can_write_int_unsigned_zerofill() {
        assert_eq!(
            DataType::Int {
                m: Some(4),
                unsigned: true,
                zerofill: true
            }
            .to_string()
            .as_str(),
            "INT (4) UNSIGNED ZEROFILL"
        );
    }

    #[test]
    fn can_parse_bigint_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "BIGINT,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::BigInt {
                m: None,
                unsigned: false,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_bigint() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "BIGINT (4),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::BigInt {
                m: Some(4),
                unsigned: false,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_bigint_unsigned() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "BIGINT (4) UNSIGNED,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::BigInt {
                m: Some(4),
                unsigned: true,
                zerofill: false,
            }
        ));
    }

    #[test]
    fn can_parse_bigint_unsigned_zerofill() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "BIGINT (4) UNSIGNED ZEROFILL,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::BigInt {
                m: Some(4),
                unsigned: true,
                zerofill: true,
            }
        ));
    }

    #[test]
    fn can_write_bigint_default() {
        assert_eq!(
            DataType::BigInt {
                m: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "BIGINT"
        );
    }

    #[test]
    fn can_write_bigint() {
        assert_eq!(
            DataType::BigInt {
                m: Some(4),
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "BIGINT (4)"
        );
    }

    #[test]
    fn can_write_bigint_unsigned() {
        assert_eq!(
            DataType::BigInt {
                m: Some(4),
                unsigned: true,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "BIGINT (4) UNSIGNED"
        );
    }

    #[test]
    fn can_write_bigint_unsigned_zerofill() {
        assert_eq!(
            DataType::BigInt {
                m: Some(4),
                unsigned: true,
                zerofill: true
            }
            .to_string()
            .as_str(),
            "BIGINT (4) UNSIGNED ZEROFILL"
        );
    }

    #[test]
    fn can_parse_decimal_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DECIMAL,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Decimal {
                m: None,
                d: None,
                unsigned: false,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_decimal_with_m() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DECIMAL (4),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Decimal {
                m: Some(4),
                d: None,
                unsigned: false,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_decimal_with_md() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DECIMAL (4, 6),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Decimal {
                m: Some(4),
                d: Some(6),
                unsigned: false,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_decimal_with_md_unsigned() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DECIMAL (4, 6) UNSIGNED,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Decimal {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_decimal_with_md_unsigned_zerofill() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DECIMAL (4, 6) UNSIGNED ZEROFILL,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Decimal {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: true
            }
        ));
    }

    #[test]
    fn can_write_decimal_default() {
        assert_eq!(
            DataType::Decimal {
                m: None,
                d: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "DECIMAL"
        );
    }

    #[test]
    fn can_write_decimal_with_m() {
        assert_eq!(
            DataType::Decimal {
                m: Some(4),
                d: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "DECIMAL (4)"
        );
    }

    #[test]
    fn can_write_decimal_with_md() {
        assert_eq!(
            DataType::Decimal {
                m: Some(4),
                d: Some(6),
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "DECIMAL (4, 6)"
        );
    }

    #[test]
    fn can_write_decimal_with_md_unsigned() {
        assert_eq!(
            DataType::Decimal {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "DECIMAL (4, 6) UNSIGNED"
        );
    }

    #[test]
    fn can_write_decimal_with_md_unsigned_zerofill() {
        assert_eq!(
            DataType::Decimal {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: true
            }
            .to_string()
            .as_str(),
            "DECIMAL (4, 6) UNSIGNED ZEROFILL"
        );
    }

    #[test]
    fn can_parse_float_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "FLOAT,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Float {
                m: None,
                d: None,
                unsigned: false,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_float_with_m() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "FLOAT (4),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Float {
                m: Some(4),
                d: None,
                unsigned: false,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_float_with_md() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "FLOAT (4, 6),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Float {
                m: Some(4),
                d: Some(6),
                unsigned: false,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_float_with_md_unsigned() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "FLOAT (4, 6) UNSIGNED,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Float {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_float_with_md_unsigned_zerofill() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "FLOAT (4, 6) UNSIGNED ZEROFILL,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Float {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: true
            }
        ));
    }

    #[test]
    fn can_write_float_default() {
        assert_eq!(
            DataType::Float {
                m: None,
                d: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "FLOAT"
        );
    }

    #[test]
    fn can_write_float_with_m() {
        assert_eq!(
            DataType::Float {
                m: Some(4),
                d: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "FLOAT (4)"
        );
    }

    #[test]
    fn can_write_float_with_md() {
        assert_eq!(
            DataType::Float {
                m: Some(4),
                d: Some(6),
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "FLOAT (4, 6)"
        );
    }

    #[test]
    fn can_write_float_with_md_unsigned() {
        assert_eq!(
            DataType::Float {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "FLOAT (4, 6) UNSIGNED"
        );
    }

    #[test]
    fn can_write_float_with_md_unsigned_zerofill() {
        assert_eq!(
            DataType::Float {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: true
            }
            .to_string()
            .as_str(),
            "FLOAT (4, 6) UNSIGNED ZEROFILL"
        );
    }

    #[test]
    fn can_parse_double_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DOUBLE,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Double {
                m: None,
                d: None,
                unsigned: false,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_double_with_m() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DOUBLE (4),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Double {
                m: Some(4),
                d: None,
                unsigned: false,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_double_with_md() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DOUBLE (4, 6),",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Double {
                m: Some(4),
                d: Some(6),
                unsigned: false,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_double_with_md_unsigned() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DOUBLE (4, 6) UNSIGNED,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Double {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: false
            }
        ));
    }

    #[test]
    fn can_parse_double_with_md_unsigned_zerofill() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DOUBLE (4, 6) UNSIGNED ZEROFILL,",)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Double {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: true
            }
        ));
    }

    #[test]
    fn can_write_double_default() {
        assert_eq!(
            DataType::Double {
                m: None,
                d: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "DOUBLE"
        );
    }

    #[test]
    fn can_write_double_with_m() {
        assert_eq!(
            DataType::Double {
                m: Some(4),
                d: None,
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "DOUBLE (4)"
        );
    }

    #[test]
    fn can_write_double_with_md() {
        assert_eq!(
            DataType::Double {
                m: Some(4),
                d: Some(6),
                unsigned: false,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "DOUBLE (4, 6)"
        );
    }

    #[test]
    fn can_write_double_with_md_unsigned() {
        assert_eq!(
            DataType::Double {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: false
            }
            .to_string()
            .as_str(),
            "DOUBLE (4, 6) UNSIGNED"
        );
    }

    #[test]
    fn can_write_double_with_md_unsigned_zerofill() {
        assert_eq!(
            DataType::Double {
                m: Some(4),
                d: Some(6),
                unsigned: true,
                zerofill: true
            }
            .to_string()
            .as_str(),
            "DOUBLE (4, 6) UNSIGNED ZEROFILL"
        );
    }

    #[test]
    fn can_parse_bit_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "BIT,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Bit { m: None },
        ))
    }

    #[test]
    fn can_parse_bit() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "BIT (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Bit { m: Some(4) },
        ))
    }

    #[test]
    fn can_write_bit_default() {
        assert_eq!(DataType::Bit { m: None }.to_string().as_str(), "BIT");
    }

    #[test]
    fn can_write_bit() {
        assert_eq!(DataType::Bit { m: Some(4) }.to_string().as_str(), "BIT (4)");
    }

    #[test]
    fn can_parse_date() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DATE,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Date,
        ));
    }

    #[test]
    fn can_write_date() {
        assert_eq!(DataType::Date.to_string().as_str(), "DATE");
    }

    #[test]
    fn can_parse_datetime_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DATETIME,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::DateTime { fsp: None },
        ))
    }

    #[test]
    fn can_parse_datetime() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "DATETIME (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::DateTime { fsp: Some(4) },
        ))
    }

    #[test]
    fn can_write_datetime_default() {
        assert_eq!(
            DataType::DateTime { fsp: None }.to_string().as_str(),
            "DATETIME"
        );
    }

    #[test]
    fn can_write_datetime() {
        assert_eq!(
            DataType::DateTime { fsp: Some(4) }.to_string().as_str(),
            "DATETIME (4)"
        );
    }

    #[test]
    fn can_parse_timestamp_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TIMESTAMP,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Timestamp { fsp: None },
        ))
    }

    #[test]
    fn can_parse_timestamp() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TIMESTAMP (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Timestamp { fsp: Some(4) },
        ))
    }

    #[test]
    fn can_write_timestamp_default() {
        assert_eq!(
            DataType::Timestamp { fsp: None }.to_string().as_str(),
            "TIMESTAMP"
        );
    }

    #[test]
    fn can_write_timestamp() {
        assert_eq!(
            DataType::Timestamp { fsp: Some(4) }.to_string().as_str(),
            "TIMESTAMP (4)"
        );
    }

    #[test]
    fn can_parse_time_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TIME,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Time { fsp: None },
        ))
    }

    #[test]
    fn can_parse_time() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TIME (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Time { fsp: Some(4) },
        ))
    }

    #[test]
    fn can_write_time_default() {
        assert_eq!(DataType::Time { fsp: None }.to_string().as_str(), "TIME");
    }

    #[test]
    fn can_write_time() {
        assert_eq!(
            DataType::Time { fsp: Some(4) }.to_string().as_str(),
            "TIME (4)"
        );
    }

    #[test]
    fn can_parse_year_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "YEAR,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Year { m: None },
        ))
    }

    #[test]
    fn can_parse_year() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "YEAR (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Year { m: Some(4) },
        ))
    }

    #[test]
    fn can_write_year_default() {
        assert_eq!(DataType::Year { m: None }.to_string().as_str(), "YEAR");
    }

    #[test]
    fn can_write_year() {
        assert_eq!(
            DataType::Year { m: Some(4) }.to_string().as_str(),
            "YEAR (4)"
        );
    }

    #[test]
    fn can_parse_char_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "CHAR,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Char {
                m: None,
                charset_name: None,
                collation_name: None
            },
        ));
    }

    #[test]
    fn can_parse_char() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "CHAR (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Char {
                m: Some(4),
                charset_name: None,
                collation_name: None
            },
        ));
    }

    #[test]
    fn can_parse_char_with_charset() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "CHAR (4) CHARACTER SET utf8mb4,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (m, charset_name, collation_name) = match data_type {
            DataType::Char {
                m,
                charset_name,
                collation_name,
            } => (m, charset_name, collation_name),
            other => panic!("Expected DataType::Char, not {other:?}"),
        };

        assert!(matches!(m, Some(4)));
        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert!(matches!(collation_name, None));
    }

    #[test]
    fn can_parse_char_with_charset_and_collate() {
        let data_type = DataType::from(
            MySqlParser::parse(
                Rule::DATA_TYPE,
                "CHAR (4) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );
        let (m, charset_name, collation_name) = match data_type {
            DataType::Char {
                m,
                charset_name,
                collation_name,
            } => (m, charset_name, collation_name),
            other => panic!("Expected DataType::Char, not {other:?}"),
        };

        assert!(matches!(m, Some(4)));
        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert_eq!(collation_name.unwrap().as_str(), "utf8mb4_general_ci");
    }

    #[test]
    fn can_write_char_default() {
        assert_eq!(
            DataType::Char {
                m: None,
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "CHAR"
        );
    }

    #[test]
    fn can_write_char() {
        assert_eq!(
            DataType::Char {
                m: Some(4),
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "CHAR (4)"
        );
    }

    #[test]
    fn can_write_char_with_charset() {
        assert_eq!(
            DataType::Char {
                m: Some(4),
                charset_name: Some(String::from("utf8mb4")),
                collation_name: None
            }
            .to_string()
            .as_str(),
            "CHAR (4) CHARACTER SET utf8mb4"
        );
    }

    #[test]
    fn can_write_char_with_charset_and_collate() {
        assert_eq!(
            DataType::Char {
                m: Some(4),
                charset_name: Some(String::from("utf8mb4")),
                collation_name: Some(String::from("utf8mb4_general_ci"))
            }
            .to_string()
            .as_str(),
            "CHAR (4) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci"
        );
    }

    #[test]
    fn can_parse_varchar_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "VARCHAR,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Varchar {
                m: None,
                charset_name: None,
                collation_name: None
            },
        ));
    }

    #[test]
    fn can_parse_varchar() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "VARCHAR (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Varchar {
                m: Some(4),
                charset_name: None,
                collation_name: None
            },
        ));
    }

    #[test]
    fn can_parse_varchar_with_charset() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "VARCHAR (4) CHARACTER SET utf8mb4,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (m, charset_name, collation_name) = match data_type {
            DataType::Varchar {
                m,
                charset_name,
                collation_name,
            } => (m, charset_name, collation_name),
            other => panic!("Expected DataType::Varchar, not {other:?}"),
        };

        assert!(matches!(m, Some(4)));
        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert!(matches!(collation_name, None));
    }

    #[test]
    fn can_parse_varchar_with_charset_and_collate() {
        let data_type = DataType::from(
            MySqlParser::parse(
                Rule::DATA_TYPE,
                "VARCHAR (4) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );
        let (m, charset_name, collation_name) = match data_type {
            DataType::Varchar {
                m,
                charset_name,
                collation_name,
            } => (m, charset_name, collation_name),
            other => panic!("Expected DataType::Varchar, not {other:?}"),
        };

        assert!(matches!(m, Some(4)));
        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert_eq!(collation_name.unwrap().as_str(), "utf8mb4_general_ci");
    }

    #[test]
    fn can_write_varchar_default() {
        assert_eq!(
            DataType::Varchar {
                m: None,
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "VARCHAR"
        );
    }

    #[test]
    fn can_write_varchar() {
        assert_eq!(
            DataType::Varchar {
                m: Some(4),
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "VARCHAR (4)"
        );
    }

    #[test]
    fn can_write_varchar_with_charset() {
        assert_eq!(
            DataType::Varchar {
                m: Some(4),
                charset_name: Some(String::from("utf8mb4")),
                collation_name: None
            }
            .to_string()
            .as_str(),
            "VARCHAR (4) CHARACTER SET utf8mb4"
        );
    }

    #[test]
    fn can_write_varchar_with_charset_and_collate() {
        assert_eq!(
            DataType::Varchar {
                m: Some(4),
                charset_name: Some(String::from("utf8mb4")),
                collation_name: Some(String::from("utf8mb4_general_ci"))
            }
            .to_string()
            .as_str(),
            "VARCHAR (4) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci"
        );
    }

    #[test]
    fn can_parse_binary_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "BINARY,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Binary { m: None }
        ));
    }

    #[test]
    fn can_parse_binary() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "BINARY (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Binary { m: Some(4) }
        ));
    }

    #[test]
    fn can_write_binary_default() {
        assert_eq!(DataType::Binary { m: None }.to_string().as_str(), "BINARY");
    }

    #[test]
    fn can_write_binary() {
        assert_eq!(
            DataType::Binary { m: Some(4) }.to_string().as_str(),
            "BINARY (4)"
        );
    }

    #[test]
    fn can_parse_varbinary() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "VARBINARY (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Varbinary { m: 4 }
        ));
    }

    #[test]
    fn can_write_varbinary() {
        assert_eq!(
            DataType::Varbinary { m: 4 }.to_string().as_str(),
            "VARBINARY (4)"
        );
    }

    #[test]
    fn can_parse_blob_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "BLOB,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Blob { m: None }
        ));
    }

    #[test]
    fn can_parse_blob() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "BLOB (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Blob { m: Some(4) }
        ));
    }

    #[test]
    fn can_write_blob_default() {
        assert_eq!(DataType::Blob { m: None }.to_string().as_str(), "BLOB");
    }

    #[test]
    fn can_write_blob() {
        assert_eq!(
            DataType::Blob { m: Some(4) }.to_string().as_str(),
            "BLOB (4)"
        );
    }

    #[test]
    fn can_parse_tinyblob() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TINYBLOB,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::TinyBlob,
        ));
    }

    #[test]
    fn can_write_tinyblob() {
        assert_eq!(DataType::TinyBlob.to_string().as_str(), "TINYBLOB");
    }

    #[test]
    fn can_parse_mediumblob() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "MEDIUMBLOB,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::MediumBlob,
        ));
    }

    #[test]
    fn can_write_mediumblob() {
        assert_eq!(DataType::MediumBlob.to_string().as_str(), "MEDIUMBLOB");
    }

    #[test]
    fn can_parse_longblob() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "LONGBLOB,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::LongBlob,
        ));
    }

    #[test]
    fn can_write_longblob() {
        assert_eq!(DataType::LongBlob.to_string().as_str(), "LONGBLOB");
    }

    #[test]
    fn can_parse_text_default() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TEXT,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Text {
                m: None,
                charset_name: None,
                collation_name: None
            }
        ));
    }

    #[test]
    fn can_parse_text() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "TEXT (4),")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            ),
            DataType::Text {
                m: Some(4),
                charset_name: None,
                collation_name: None
            }
        ));
    }

    #[test]
    fn can_parse_text_with_charset() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "TEXT (4) CHARACTER SET utf8mb4,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (m, charset_name, collation_name) = match data_type {
            DataType::Text {
                m,
                charset_name,
                collation_name,
            } => (m, charset_name, collation_name),
            other => panic!("Expected DataType::Text, not {other:?}"),
        };

        assert!(matches!(m, Some(4)));
        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert!(matches!(collation_name, None));
    }

    #[test]
    fn can_parse_text_with_charset_and_collate() {
        let data_type = DataType::from(
            MySqlParser::parse(
                Rule::DATA_TYPE,
                "TEXT (4) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );
        let (m, charset_name, collation_name) = match data_type {
            DataType::Text {
                m,
                charset_name,
                collation_name,
            } => (m, charset_name, collation_name),
            other => panic!("Expected DataType::Text, not {other:?}"),
        };

        assert!(matches!(m, Some(4)));
        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert_eq!(collation_name.unwrap().as_str(), "utf8mb4_general_ci");
    }

    #[test]
    fn can_write_text_default() {
        assert_eq!(
            DataType::Text {
                m: None,
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "TEXT"
        );
    }

    #[test]
    fn can_write_text() {
        assert_eq!(
            DataType::Text {
                m: Some(4),
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "TEXT (4)"
        );
    }

    #[test]
    fn can_write_text_with_charset() {
        assert_eq!(
            DataType::Text {
                m: Some(4),
                charset_name: Some(String::from("utf8mb4")),
                collation_name: None
            }
            .to_string()
            .as_str(),
            "TEXT (4) CHARACTER SET utf8mb4"
        );
    }

    #[test]
    fn can_write_text_with_charset_and_collate() {
        assert_eq!(
            DataType::Text {
                m: Some(4),
                charset_name: Some(String::from("utf8mb4")),
                collation_name: Some(String::from("utf8mb4_general_ci"))
            }
            .to_string()
            .as_str(),
            "TEXT (4) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci"
        );
    }

    #[test]
    fn can_parse_tinytext() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "TINYTEXT,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (charset_name, collation_name) = match data_type {
            DataType::TinyText {
                charset_name,
                collation_name,
            } => (charset_name, collation_name),
            other => panic!("Expected DataType::TinyText, not {other:?}"),
        };

        assert!(charset_name.is_none());
        assert!(collation_name.is_none());
    }

    #[test]
    fn can_parse_tinytext_with_charset() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "TINYTEXT CHARACTER SET utf8mb4,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (charset_name, collation_name) = match data_type {
            DataType::TinyText {
                charset_name,
                collation_name,
            } => (charset_name, collation_name),
            other => panic!("Expected DataType::TinyText, not {other:?}"),
        };

        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert!(collation_name.is_none());
    }

    #[test]
    fn can_parse_tinytext_with_charset_and_collate() {
        let data_type = DataType::from(
            MySqlParser::parse(
                Rule::DATA_TYPE,
                "TINYTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );
        let (charset_name, collation_name) = match data_type {
            DataType::TinyText {
                charset_name,
                collation_name,
            } => (charset_name, collation_name),
            other => panic!("Expected DataType::TinyText, not {other:?}"),
        };

        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert_eq!(collation_name.unwrap().as_str(), "utf8mb4_general_ci");
    }

    #[test]
    fn can_write_tinytext() {
        assert_eq!(
            DataType::TinyText {
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "TINYTEXT"
        );
    }

    #[test]
    fn can_write_tinytext_with_charset() {
        assert_eq!(
            DataType::TinyText {
                charset_name: Some(String::from("utf8mb4")),
                collation_name: None
            }
            .to_string()
            .as_str(),
            "TINYTEXT CHARACTER SET utf8mb4"
        );
    }

    #[test]
    fn can_write_tinytext_with_charset_and_collate() {
        assert_eq!(
            DataType::TinyText {
                charset_name: Some(String::from("utf8mb4")),
                collation_name: Some(String::from("utf8mb4_general_ci"))
            }
            .to_string()
            .as_str(),
            "TINYTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci"
        );
    }

    #[test]
    fn can_parse_mediumtext() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "MEDIUMTEXT,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (charset_name, collation_name) = match data_type {
            DataType::MediumText {
                charset_name,
                collation_name,
            } => (charset_name, collation_name),
            other => panic!("Expected DataType::MediumText, not {other:?}"),
        };

        assert!(charset_name.is_none());
        assert!(collation_name.is_none());
    }

    #[test]
    fn can_parse_mediumtext_with_charset() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "MEDIUMTEXT CHARACTER SET utf8mb4,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (charset_name, collation_name) = match data_type {
            DataType::MediumText {
                charset_name,
                collation_name,
            } => (charset_name, collation_name),
            other => panic!("Expected DataType::MediumText, not {other:?}"),
        };

        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert!(collation_name.is_none());
    }

    #[test]
    fn can_parse_mediumtext_with_charset_and_collate() {
        let data_type = DataType::from(
            MySqlParser::parse(
                Rule::DATA_TYPE,
                "MEDIUMTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );
        let (charset_name, collation_name) = match data_type {
            DataType::MediumText {
                charset_name,
                collation_name,
            } => (charset_name, collation_name),
            other => panic!("Expected DataType::MediumText, not {other:?}"),
        };

        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert_eq!(collation_name.unwrap().as_str(), "utf8mb4_general_ci");
    }

    #[test]
    fn can_write_mediumtext() {
        assert_eq!(
            DataType::MediumText {
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "MEDIUMTEXT"
        );
    }

    #[test]
    fn can_write_mediumtext_with_charset() {
        assert_eq!(
            DataType::MediumText {
                charset_name: Some(String::from("utf8mb4")),
                collation_name: None
            }
            .to_string()
            .as_str(),
            "MEDIUMTEXT CHARACTER SET utf8mb4"
        );
    }

    #[test]
    fn can_write_mediumtext_with_charset_and_collate() {
        assert_eq!(
            DataType::MediumText {
                charset_name: Some(String::from("utf8mb4")),
                collation_name: Some(String::from("utf8mb4_general_ci"))
            }
            .to_string()
            .as_str(),
            "MEDIUMTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci"
        );
    }

    #[test]
    fn can_parse_longtext() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "LONGTEXT,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (charset_name, collation_name) = match data_type {
            DataType::LongText {
                charset_name,
                collation_name,
            } => (charset_name, collation_name),
            other => panic!("Expected DataType::LongText, not {other:?}"),
        };

        assert!(charset_name.is_none());
        assert!(collation_name.is_none());
    }

    #[test]
    fn can_parse_longtext_with_charset() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "LONGTEXT CHARACTER SET utf8mb4,")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (charset_name, collation_name) = match data_type {
            DataType::LongText {
                charset_name,
                collation_name,
            } => (charset_name, collation_name),
            other => panic!("Expected DataType::LongText, not {other:?}"),
        };

        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert!(collation_name.is_none());
    }

    #[test]
    fn can_parse_longtext_with_charset_and_collate() {
        let data_type = DataType::from(
            MySqlParser::parse(
                Rule::DATA_TYPE,
                "LONGTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );
        let (charset_name, collation_name) = match data_type {
            DataType::LongText {
                charset_name,
                collation_name,
            } => (charset_name, collation_name),
            other => panic!("Expected DataType::LongText, not {other:?}"),
        };

        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert_eq!(collation_name.unwrap().as_str(), "utf8mb4_general_ci");
    }

    #[test]
    fn can_write_longtext() {
        assert_eq!(
            DataType::LongText {
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "LONGTEXT"
        );
    }

    #[test]
    fn can_write_longtext_with_charset() {
        assert_eq!(
            DataType::LongText {
                charset_name: Some(String::from("utf8mb4")),
                collation_name: None
            }
            .to_string()
            .as_str(),
            "LONGTEXT CHARACTER SET utf8mb4"
        );
    }

    #[test]
    fn can_write_longtext_with_charset_and_collate() {
        assert_eq!(
            DataType::LongText {
                charset_name: Some(String::from("utf8mb4")),
                collation_name: Some(String::from("utf8mb4_general_ci"))
            }
            .to_string()
            .as_str(),
            "LONGTEXT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci"
        );
    }

    #[test]
    fn can_parse_enum() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "ENUM ('value_one', 'value_two'),")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (values, charset_name, collation_name) = match data_type {
            DataType::Enum {
                values,
                charset_name,
                collation_name,
            } => (values, charset_name, collation_name),
            other => panic!("Expected DataType::Enum, not {other:?}"),
        };

        assert_eq!(
            values,
            vec![String::from("value_one"), String::from("value_two")]
        );
        assert!(charset_name.is_none());
        assert!(collation_name.is_none());
    }

    #[test]
    fn can_parse_enum_with_charset() {
        let data_type = DataType::from(
            MySqlParser::parse(
                Rule::DATA_TYPE,
                "ENUM ('value_one', 'value_two') CHARACTER SET utf8mb4,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );
        let (values, charset_name, collation_name) = match data_type {
            DataType::Enum {
                values,
                charset_name,
                collation_name,
            } => (values, charset_name, collation_name),
            other => panic!("Expected DataType::Enum, not {other:?}"),
        };

        assert_eq!(
            values,
            vec![String::from("value_one"), String::from("value_two")]
        );
        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert!(collation_name.is_none());
    }

    #[test]
    fn can_parse_enum_with_charset_and_collate() {
        let data_type = DataType::from(
            MySqlParser::parse(
                Rule::DATA_TYPE,
                "ENUM ('value_one', 'value_two') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );
        let (values, charset_name, collation_name) = match data_type {
            DataType::Enum {
                values,
                charset_name,
                collation_name,
            } => (values, charset_name, collation_name),
            other => panic!("Expected DataType::Enum, not {other:?}"),
        };

        assert_eq!(
            values,
            vec![String::from("value_one"), String::from("value_two")]
        );
        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert_eq!(collation_name.unwrap().as_str(), "utf8mb4_general_ci");
    }

    #[test]
    fn can_write_enum() {
        assert_eq!(
            DataType::Enum {
                values: vec![String::from("value_one"), String::from("value_two")],
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "ENUM ('value_one', 'value_two')"
        );
    }

    #[test]
    fn can_write_enum_with_charset() {
        assert_eq!(
            DataType::Enum {
                values: vec![String::from("value_one"), String::from("value_two")],
                charset_name: Some(String::from("utf8mb4")),
                collation_name: None
            }
            .to_string()
            .as_str(),
            "ENUM ('value_one', 'value_two') CHARACTER SET utf8mb4"
        );
    }

    #[test]
    fn can_write_enum_with_charset_and_collate() {
        assert_eq!(
            DataType::Enum {
                values: vec![String::from("value_one"), String::from("value_two")],
                charset_name: Some(String::from("utf8mb4")),
                collation_name: Some(String::from("utf8mb4_general_ci"))
            }
            .to_string()
            .as_str(),
            "ENUM ('value_one', 'value_two') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci"
        );
    }

    #[test]
    fn can_parse_set() {
        let data_type = DataType::from(
            MySqlParser::parse(Rule::DATA_TYPE, "SET ('value_one', 'value_two'),")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );
        let (values, charset_name, collation_name) = match data_type {
            DataType::Set {
                values,
                charset_name,
                collation_name,
            } => (values, charset_name, collation_name),
            other => panic!("Expected DataType::Set, not {other:?}"),
        };

        assert_eq!(
            values,
            vec![String::from("value_one"), String::from("value_two")]
        );
        assert!(charset_name.is_none());
        assert!(collation_name.is_none());
    }

    #[test]
    fn can_parse_set_with_charset() {
        let data_type = DataType::from(
            MySqlParser::parse(
                Rule::DATA_TYPE,
                "SET ('value_one', 'value_two') CHARACTER SET utf8mb4,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );
        let (values, charset_name, collation_name) = match data_type {
            DataType::Set {
                values,
                charset_name,
                collation_name,
            } => (values, charset_name, collation_name),
            other => panic!("Expected DataType::Set, not {other:?}"),
        };

        assert_eq!(
            values,
            vec![String::from("value_one"), String::from("value_two")]
        );
        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert!(collation_name.is_none());
    }

    #[test]
    fn can_parse_set_with_charset_and_collate() {
        let data_type = DataType::from(
            MySqlParser::parse(
                Rule::DATA_TYPE,
                "SET ('value_one', 'value_two') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci,",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );
        let (values, charset_name, collation_name) = match data_type {
            DataType::Set {
                values,
                charset_name,
                collation_name,
            } => (values, charset_name, collation_name),
            other => panic!("Expected DataType::Set, not {other:?}"),
        };

        assert_eq!(
            values,
            vec![String::from("value_one"), String::from("value_two")]
        );
        assert_eq!(charset_name.unwrap().as_str(), "utf8mb4");
        assert_eq!(collation_name.unwrap().as_str(), "utf8mb4_general_ci");
    }

    #[test]
    fn can_write_set() {
        assert_eq!(
            DataType::Set {
                values: vec![String::from("value_one"), String::from("value_two")],
                charset_name: None,
                collation_name: None
            }
            .to_string()
            .as_str(),
            "SET ('value_one', 'value_two')"
        );
    }

    #[test]
    fn can_write_set_with_charset() {
        assert_eq!(
            DataType::Set {
                values: vec![String::from("value_one"), String::from("value_two")],
                charset_name: Some(String::from("utf8mb4")),
                collation_name: None
            }
            .to_string()
            .as_str(),
            "SET ('value_one', 'value_two') CHARACTER SET utf8mb4"
        );
    }

    #[test]
    fn can_write_set_with_charset_and_collate() {
        assert_eq!(
            DataType::Set {
                values: vec![String::from("value_one"), String::from("value_two")],
                charset_name: Some(String::from("utf8mb4")),
                collation_name: Some(String::from("utf8mb4_general_ci"))
            }
            .to_string()
            .as_str(),
            "SET ('value_one', 'value_two') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci"
        );
    }

    #[test]
    fn can_parse_json() {
        assert!(matches!(
            DataType::from(
                MySqlParser::parse(Rule::DATA_TYPE, "JSON,")
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input")
            ),
            DataType::Json,
        ));
    }

    #[test]
    fn can_write_json() {
        assert_eq!(DataType::Json.to_string().as_str(), "JSON");
    }
}
