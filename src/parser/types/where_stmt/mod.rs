use pest::iterators::Pair;

use crate::parser::{types::TEMPLATES, Parser, Rule, Sql};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Where {
    pub column: String,
    pub operator: String,
    pub value: String,
}

impl Where {
    pub fn new(column: String, operator: String, value: String) -> Self {
        Where {
            column,
            operator,
            value,
        }
    }
}

impl From<Pair<'_, Rule>> for Where {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut pair = pair.into_inner();
        let column = pair.next().unwrap();
        println!("column: {:?}", column);
        let operator = pair.next().unwrap();
        println!("operator: {:?}", operator);
        let value = pair.next().unwrap();
        println!("value: {:?}", value);
        Where::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::MySqlParser;

    use super::*;

    #[test]
    fn can_parse_a_valid_where_stmt() {
        let sql = "WHERE id = 1";
        let mut parsed = MySqlParser::parse(Rule::WHERE_CLAUSE, sql).unwrap();
        let where_stmt = Where::from(parsed.next().unwrap());
        assert_eq!(where_stmt.column, "id");
        assert_eq!(where_stmt.operator, "=");
        assert_eq!(where_stmt.value, "1");
    }
}
