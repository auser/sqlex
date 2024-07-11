use crate::parser::Rule;
use pest::iterators::Pair;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct Where {
    pub column: String,
    pub operator: String,
    pub value: String,
}

impl From<Pair<'_, Rule>> for Where {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let mut inner_pair = inner.next().unwrap().into_inner();
        let column = inner_pair.next().unwrap().as_str().to_string();
        let operator = inner_pair.next().unwrap().as_str().to_string();
        let value = inner_pair.next().unwrap().as_str().to_string();

        Self {
            column,
            operator,
            value,
        }
    }
}

impl Display for Where {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {} {}", self.column, self.operator, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::MySqlParser;
    use pest::Parser;

    #[test]
    fn test_with_valid_where_stmt() {
        let sql = "WHERE id = 1";
        let mut parsed = MySqlParser::parse(Rule::WHERE_CLAUSE, sql).unwrap();
        let where_stmt = Where::from(parsed.next().unwrap());
        assert_eq!(where_stmt.column, "id");
        assert_eq!(where_stmt.operator, "=");
        assert_eq!(where_stmt.value, "1");
    }

    #[test]
    fn test_with_valid_where_stmt_with_boolean() {
        let sql = "WHERE id = true";
        let mut parsed = MySqlParser::parse(Rule::WHERE_CLAUSE, sql).unwrap();
        let where_stmt = Where::from(parsed.next().unwrap());
        assert_eq!(where_stmt.column, "id");
        assert_eq!(where_stmt.operator, "=");
        assert_eq!(where_stmt.value, "true");
    }

    #[test]
    fn test_with_greater_than() {
        let sql = "WHERE id > 1";
        let mut parsed = MySqlParser::parse(Rule::WHERE_CLAUSE, sql).unwrap();
        let where_stmt = Where::from(parsed.next().unwrap());
        assert_eq!(where_stmt.column, "id");
        assert_eq!(where_stmt.operator, ">");
        assert_eq!(where_stmt.value, "1");
    }
}
