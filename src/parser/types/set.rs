use crate::parser::{parse_utils::trim_str, Rule};
use pest::iterators::{Pair, Pairs};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, PartialEq)]
pub enum SetValue {
    String(String),
    Number(usize),
    Boolean(bool),
    Null,
}

impl Display for SetValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetValue::String(s) => write!(f, "'{}'", s),
            SetValue::Number(n) => write!(f, "{}", n),
            SetValue::Boolean(b) => write!(f, "{}", b),
            SetValue::Null => write!(f, "NULL"),
        }
    }
}

impl From<Pair<'_, Rule>> for SetValue {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::STRING_LITERAL => SetValue::String(trim_str(pair)),
            Rule::BOOLEAN_LITERAL => SetValue::Boolean(trim_str(pair).parse().unwrap()),
            Rule::NUMBER => {
                let value = pair.as_span().as_str();
                SetValue::Number(value.parse().unwrap())
            }
            _ => SetValue::String(trim_str(pair)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetKey {
    At(String),
    Identifier(String),
}

impl Display for SetKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            SetKey::At(s) => write!(f, "@{}", s),
            SetKey::Identifier(s) => write!(f, "{}", s),
        }
    }
}

impl From<Pair<'_, Rule>> for SetKey {
    fn from(pair: Pair<'_, Rule>) -> Self {
        match pair.as_rule() {
            Rule::AT_MARK => {
                let name = pair.into_inner().next().unwrap();
                SetKey::At(trim_str(name))
            }
            Rule::IDENTIFIER => SetKey::Identifier(trim_str(pair)),
            _ => SetKey::Identifier(trim_str(pair)),
        }
    }
}

impl From<&mut Pairs<'_, Rule>> for SetKey {
    fn from(pairs: &mut Pairs<'_, Rule>) -> Self {
        let next_pair = pairs.next().expect("Invalid set key");
        if next_pair.as_rule() == Rule::AT_MARK {
            let name = pairs.next().unwrap();
            return SetKey::At(trim_str(name));
        }
        SetKey::Identifier(trim_str(next_pair))
    }
}

#[derive(Debug, Clone)]
pub struct KVPair {
    pub key: SetKey,
    pub value: SetValue,
}

impl KVPair {
    pub fn new(key: SetKey, value: SetValue) -> Self {
        KVPair { key, value }
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub kv_pairs: Vec<KVPair>,
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let s = self
            .kv_pairs
            .iter()
            .map(|kv| format!("{}={}", kv.key, SetValue::from(kv.value.clone())))
            .collect::<Vec<String>>();

        write!(f, "SET {}", s.join(", "))
    }
}

impl From<Pair<'_, Rule>> for Set {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let inner = pair.into_inner();
        let kv_pairs: Vec<KVPair> = inner
            .into_iter()
            .map(|p| {
                let mut part = p.into_inner();
                let name = SetKey::from(&mut part);
                let value = SetValue::from(part.next().unwrap());
                KVPair::new(name, value)
            })
            .collect();
        Set { kv_pairs }
    }
}

impl From<Pairs<'_, Rule>> for Set {
    fn from(pairs: Pairs<'_, Rule>) -> Self {
        let set: Vec<Set> = pairs.into_iter().map(Set::from).collect();
        set[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{MySqlParser, Rule};
    use pest::Parser;

    #[test]
    fn test_parses_set_into_single_kv_pair() {
        let sql = "SET @a = 1";
        let parsed = MySqlParser::parse(Rule::SET_STATEMENT, sql);

        assert!(parsed.is_ok());

        let set = Set::from(parsed.unwrap());
        let kvs = set.kv_pairs;

        assert_eq!(kvs.len(), 1);
        assert_eq!(kvs[0].key, SetKey::At("a".to_string()));
        assert_eq!(kvs[0].value, SetValue::Number(1));
    }

    #[test]
    fn test_parses_set_into_k_v_pairs() {
        let sql = "SET @a = 1, @b = true, name = 'John'";
        let parsed = MySqlParser::parse(Rule::SET_STATEMENT, sql);

        assert!(parsed.is_ok());

        let set = Set::from(parsed.unwrap());
        let kvs = set.kv_pairs;

        assert_eq!(kvs.len(), 3);
        assert_eq!(kvs[0].key, SetKey::At("a".to_string()));
        assert_eq!(kvs[0].value, SetValue::Number(1));
        assert_eq!(kvs[1].key, SetKey::At("b".to_string()));
        assert_eq!(kvs[1].value, SetValue::Boolean(true));
        assert_eq!(kvs[2].key, SetKey::Identifier("name".to_string()));
        assert_eq!(kvs[2].value, SetValue::String("John".to_string()));
    }

    #[test]
    fn test_set_back_to_sql() {
        let sql = "SET @a = 1, @b = true, name = 'John'";
        let parsed = MySqlParser::parse(Rule::SET_STATEMENT, sql);

        assert!(parsed.is_ok());

        let set = Set::from(parsed.unwrap());
        let sql = set.to_string();

        assert_eq!(sql, "SET @a=1, @b=true, name='John'");
    }

    #[test]
    fn test_errors_with_invalid_set_stmt() {
        let sql = "SET @a > 1";
        let parsed = MySqlParser::parse(Rule::SET_STATEMENT, sql);

        assert!(parsed.is_err());
    }
}
