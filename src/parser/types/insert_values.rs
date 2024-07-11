use crate::parser::{types::InsertValue, Rule};
use pest::iterators::Pair;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct InsertValues(pub Vec<InsertValue>);

impl From<Pair<'_, Rule>> for InsertValues {
    fn from(pair: Pair<'_, Rule>) -> Self {
        Self(pair.into_inner().map(|p| InsertValue::from(p)).collect())
    }
}

impl Display for InsertValues {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "(")?;

        for (i, value) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}", value.to_string())?;
        }

        write!(f, ")")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;

    #[test]
    fn can_parse_insert_values() {
        let insert_values = InsertValues::from(
            MySqlParser::parse(Rule::INSERT_VALUES, "(NULL, DEFAULT, 'Foo', 42.69, Baz)")
                .expect("Invalid input")
                .next()
                .expect("Unable to parse input"),
        );

        assert!(matches!(insert_values.0[0], InsertValue::Null));
        assert!(matches!(insert_values.0[1], InsertValue::Default));

        match insert_values.0.get(2).unwrap() {
            InsertValue::Text { value } => assert_eq!(value.as_str(), "Foo"),
            _ => panic!("Expected value 2 to be a string literal"),
        }
        match insert_values.0.get(3).unwrap() {
            InsertValue::Number { value } => assert_eq!(value.as_str(), "42.69"),
            _ => panic!("Expected value 3 to be a number"),
        }
        match insert_values.0.get(4).unwrap() {
            InsertValue::Identifier { value } => assert_eq!(value.as_str(), "Baz"),
            _ => panic!("Expected value 4 to be an identifier"),
        }
    }

    #[test]
    fn can_write_insert_values() {
        assert_eq!(
            InsertValues(vec![
                InsertValue::Null,
                InsertValue::Default,
                InsertValue::Text {
                    value: String::from("Foo")
                },
                InsertValue::Number {
                    value: String::from("42.69")
                },
                InsertValue::Identifier {
                    value: String::from("Baz")
                }
            ])
            .to_string()
            .as_str(),
            "(NULL, DEFAULT, 'Foo', 42.69, Baz)"
        );
    }
}
