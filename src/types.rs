use std::collections::HashMap;

use pest::iterators::Pair;
use serde::ser::SerializeStruct;
use serde::Serialize;
use serde::Serializer;
use sql_parse::Type;

use crate::parser::parse_utils::trimmed_str;
use crate::parser::Rule;

#[derive(Clone, Debug, Serialize)]
pub struct Database {
    pub db_name: String,
    pub tables: HashMap<String, Table>,
}

impl Database {
    pub fn new(db_name: String) -> Self {
        Database {
            db_name,
            tables: HashMap::new(),
        }
    }
}

impl<'a> From<Pair<'a, Rule>> for Database {
    fn from(pair: Pair<'a, Rule>) -> Self {
        let mut database_pairs = pair.into_inner();
        let db_name = trimmed_str(
            database_pairs
                .next()
                .expect("unable to unwrap database name"),
        );
        Database {
            db_name,
            tables: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub constraints: Option<Vec<Constraint>>,
}

impl Table {
    pub fn into_json(&self) -> String {
        let mut map = HashMap::new();
        map.insert("name", self.name.clone());
        map.insert(
            "columns",
            self.columns
                .iter()
                .map(|c| serde_json::to_string(c).unwrap())
                .collect(),
        );
        serde_json::to_string(&map).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Column {
    pub name: String,
    pub type_: ColumnType,
}

#[derive(Clone, Debug, Serialize)]
pub enum ColumnType {
    String,
    Int,
    BigInt,
    Boolean,
    Float,
    Double,
    Decimal,
    Char,
    Timestamp,
    Date,
    DateTime,
    Time,
}

impl Serialize for Column {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Column", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("type", &self.type_)?;
        state.end()
    }
}

impl<'a> From<Type<'a>> for ColumnType {
    fn from(data_type: Type<'a>) -> Self {
        match data_type {
            Type::Boolean => ColumnType::Boolean,
            Type::Integer(..) => ColumnType::Int,
            Type::BigInt(..) => ColumnType::BigInt,
            Type::Float(..) => ColumnType::Float,
            Type::Double(..) => ColumnType::Double,
            Type::TinyInt(..) => ColumnType::Decimal,
            Type::SmallInt(..) => ColumnType::Decimal,
            Type::Char(..) => ColumnType::Char,
            Type::VarChar(..) => ColumnType::String,
            Type::Text(..) => ColumnType::String,
            Type::Timestamp(..) => ColumnType::Timestamp,
            Type::DateTime(..) => ColumnType::DateTime,
            Type::Time(..) => ColumnType::Time,
            _ => ColumnType::String,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Constraint {
    pub name: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct TableInsertReplace {
    pub columns: Vec<String>,
    pub values: Vec<String>,
}
