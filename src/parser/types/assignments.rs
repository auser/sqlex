use crate::parser::{parse_utils::trim_str, Rule};
use pest::iterators::{Pair, Pairs};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssignmentValue {
    String(String),
    Number(usize),
    Boolean(bool),
    Null,
}

impl Display for AssignmentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignmentValue::String(s) => write!(f, "'{}'", s),
            AssignmentValue::Number(n) => write!(f, "{}", n),
            AssignmentValue::Boolean(b) => write!(f, "{}", b),
            AssignmentValue::Null => write!(f, "NULL"),
        }
    }
}

impl From<Pair<'_, Rule>> for AssignmentValue {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::STRING_LITERAL => AssignmentValue::String(trim_str(pair)),
            Rule::BOOLEAN_LITERAL => AssignmentValue::Boolean(trim_str(pair).parse().unwrap()),
            Rule::NUMBER => {
                let value = pair.as_span().as_str();
                AssignmentValue::Number(value.parse().unwrap())
            }
            _ => AssignmentValue::String(trim_str(pair)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssignmentKey {
    At(String),
    Identifier(String),
}

impl Display for AssignmentKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignmentKey::At(s) => write!(f, "@{}", s),
            AssignmentKey::Identifier(s) => write!(f, "{}", s),
        }
    }
}

impl From<Pair<'_, Rule>> for AssignmentKey {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::AT_MARK => {
                let name = pair.into_inner().next().unwrap();
                AssignmentKey::At(trim_str(name))
            }
            Rule::IDENTIFIER => AssignmentKey::Identifier(trim_str(pair)),
            _ => AssignmentKey::Identifier(trim_str(pair)),
        }
    }
}

impl From<&mut Pairs<'_, Rule>> for AssignmentKey {
    fn from(pairs: &mut Pairs<'_, Rule>) -> Self {
        println!("pairs in AssignmentKey: {:?}", pairs);
        let next_pair = pairs.next().expect("Invalid set key");
        if next_pair.as_rule() == Rule::AT_MARK {
            let name = pairs.next().unwrap();
            return AssignmentKey::At(trim_str(name));
        }
        AssignmentKey::Identifier(trim_str(next_pair))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KVPair {
    pub key: AssignmentKey,
    pub value: AssignmentValue,
}

impl KVPair {
    pub fn new(key: AssignmentKey, value: AssignmentValue) -> Self {
        KVPair { key, value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Assignment {
    pub kv_pairs: Vec<KVPair>,
}

impl Display for Assignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = self
            .kv_pairs
            .iter()
            .map(|kv| format!("`{}` = {}", kv.key, kv.value))
            .collect::<Vec<String>>();

        write!(f, "{}", s.join(", "))
    }
}

impl From<Pair<'_, Rule>> for Assignment {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let mut kv_pairs: Vec<KVPair> = vec![];

        while let Some(key_inner) = inner.next() {
            let key = AssignmentKey::from(key_inner);
            let value_inner = inner.next().expect("invalid value");
            let value = AssignmentValue::from(value_inner);
            kv_pairs.push(KVPair::new(key, value));
        }

        Assignment { kv_pairs }
    }
}

impl From<Pairs<'_, Rule>> for Assignment {
    fn from(pairs: Pairs<'_, Rule>) -> Self {
        let assignments: Vec<Assignment> = pairs.into_iter().map(Assignment::from).collect();
        let kv_pairs = assignments
            .into_iter()
            .fold(Vec::new(), |mut acc, assignment| {
                assignment.kv_pairs.into_iter().for_each(|kv_pair| {
                    acc.push(kv_pair);
                });
                acc
            });

        Assignment { kv_pairs: kv_pairs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{MySqlParser, Rule};
    use pest::Parser;

    #[test]
    fn test_parses_assignment_into_single_kv_pair() {
        let sql = "a = 1";
        let parsed = MySqlParser::parse(Rule::ASSIGNMENT_CLAUSE, sql);
        assert!(parsed.is_ok());
        let set = Assignment::from(parsed.unwrap());
        let kvs = set.kv_pairs;
        assert_eq!(kvs.len(), 1);
        assert_eq!(kvs[0].key, AssignmentKey::Identifier("a".to_string()));
        assert_eq!(kvs[0].value, AssignmentValue::Number(1));
    }
}
