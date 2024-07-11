use crate::parser::{
    types::{InsertPriority, InsertValues},
    Rule,
};
use pest::iterators::Pair;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct Insert {
    pub priority: Option<InsertPriority>,
    pub ignore: bool,
    pub table_name: String,
    pub column_names: Vec<String>,
    pub values: Vec<InsertValues>,
}

impl From<Pair<'_, Rule>> for Insert {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let mut priority = None;
        let mut ignore = false;
        let mut table_name = None;
        let mut column_names = None;
        let values;

        loop {
            let element = inner.next().unwrap();

            match element.as_rule() {
                Rule::INSERT_PRIORITY => priority = Some(InsertPriority::from(element)),
                Rule::INSERT_IGNORE => ignore = true,
                Rule::QUOTED_IDENTIFIER => table_name = Some(element.as_str().trim_matches('`').to_string()),
                Rule::INSERT_COLUMNS => column_names = Some(element.into_inner().map(|p| p.as_str().trim_matches('`').to_string()).collect::<Vec<String>>()),
                Rule::INSERT_VALUES_LIST => { values = element.into_inner().map(|p| InsertValues::from(p)).collect::<Vec<InsertValues>>(); break },
                other => panic!("Expected INSERT_PRIORITY, INSERT_IGNORE, QUOTED_IDENTIFIER, INSERT_COLUMNS or INSERT_VALUES_LIST, not {other:?}"),
            }
        }

        Self {
            priority,
            ignore,
            table_name: table_name.expect("table name"),
            column_names: column_names.unwrap_or_else(Vec::new),
            values,
        }
    }
}

impl Display for Insert {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "INSERT{}{} INTO `{}` ({}) VALUES {}",
            if let Some(ref priority) = self.priority {
                format!(" {priority}")
            } else {
                "".to_string()
            },
            if self.ignore { " IGNORE" } else { "" },
            self.table_name,
            self.column_names
                .iter()
                .map(|name| format!("`{name}`"))
                .collect::<Vec<String>>()
                .join(", "),
            self.values
                .iter()
                .map(|value| format!("{value}"))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{types::InsertValue, MySqlParser};
    use pest::Parser;

    #[test]
    fn can_parse_insert() {
        let insert = Insert::from(
            MySqlParser::parse(
                Rule::INSERT_STATEMENT,
                "INSERT HIGH_PRIORITY IGNORE INTO `my_table` (`col1`, `col2`) VALUES (NULL, DEFAULT), ('foo', 42);",
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input"),
        );

        assert!(matches!(insert.priority, Some(InsertPriority::High)));
        assert!(insert.ignore);
        assert_eq!(insert.table_name.as_str(), "my_table");
        assert_eq!(
            insert.column_names,
            vec![String::from("col1"), String::from("col2")]
        );
        assert_eq!(insert.values.len(), 2);
    }

    #[test]
    fn can_write_insert() {
        assert_eq!(
            Insert {
                priority: Some(InsertPriority::High),
                ignore: true,
                table_name: String::from("my_table"),
                column_names: vec![String::from("col1"), String::from("col2")],
                values: vec![InsertValues(vec![InsertValue::Null, InsertValue::Default]), InsertValues(vec![InsertValue::Text { value: String::from("foo") }, InsertValue::Number { value: String::from("42") }])]
            }
            .to_string()
            .as_str(),
            "INSERT HIGH_PRIORITY IGNORE INTO `my_table` (`col1`, `col2`) VALUES (NULL, DEFAULT), ('foo', 42)"
        );
    }
}
