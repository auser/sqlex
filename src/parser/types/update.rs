use crate::parser::{
    types::{Assignment, Where},
    Rule,
};
use pest::iterators::Pair;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct Update {
    pub table_name: String,
    pub set_clauses: Vec<Assignment>,
    pub where_clauses: Vec<Where>,
}

impl Update {
    pub fn new(table_name: String, set_clauses: Vec<Assignment>) -> Self {
        Self {
            table_name,
            set_clauses,
            where_clauses: vec![],
        }
    }
}

impl From<Pair<'_, Rule>> for Update {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let table_name = inner
            .next()
            .map(|p| p.as_str().trim_matches('`').to_string())
            .expect("Expected an index name");
        let mut update_sets: Vec<Assignment> = Vec::new();
        let mut where_clauses: Vec<Where> = Vec::new();

        while let Some(pair) = inner.next() {
            match pair.as_rule() {
                Rule::ASSIGNMENT_CLAUSE => {
                    let set_clause = Assignment::from(pair);
                    update_sets.push(set_clause);
                }
                Rule::WHERE_CLAUSE => {
                    let where_clause = Where::from(pair);
                    where_clauses.push(where_clause);
                }
                _ => {}
            }
        }
        Update {
            table_name,
            set_clauses: update_sets,
            where_clauses,
        }
    }
}

impl Display for Update {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let table_name = self.table_name.clone();
        let set_clauses = self.set_clauses.clone();
        let where_clauses = self.where_clauses.clone();

        write!(
            f,
            r#"UPDATE `{table_name}` SET {set_clauses} WHERE {where_clauses};"#,
            table_name = table_name,
            set_clauses = set_clauses
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(","),
            where_clauses = where_clauses
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{MySqlParser, Rule};
    use pest::Parser;

    use super::*;

    #[test]
    fn test_can_parse_update() {
        let sql = "UPDATE `users` SET `name` = 'John' WHERE `id` = 1;";
        let mut parsed = MySqlParser::parse(Rule::UPDATE_STATEMENT, sql).unwrap();
        let update_stmt = parsed.next().unwrap();
        let update = Update::from(update_stmt);

        assert_eq!(update.to_string(), sql);
    }
}
