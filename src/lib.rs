pub mod cmd;
pub mod parser;
pub mod rules;
pub mod settings;
pub mod sqlparse;
pub mod types;
pub mod masker;

use anyhow::Result;
pub type ExtractResult<T = ()> = Result<T>;

pub use sqlparse::simple_parse;
