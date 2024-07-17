use crate::parser::Sql;
use pest::Parser;
#[allow(unused)]
use rayon::prelude::*;
use regex::Regex;
use sql_parse::Statement::{self, InsertReplace};
use sql_parse::{parse_statements, Expression, ParseOptions, SQLDialect};
use std::fmt::write;
use std::fs::File;
use std::path::Path;

use clap::Parser as ClapParser;

use crate::masker::Transform;
use crate::parser::statements::Insert;
use crate::parser::{MySqlParser, Rule};
// use crate::parser::MyParser;
use crate::rules::get_struct_by_name;
use crate::ExtractResult;
use crate::{settings::parse_masking_config, simple_parse, sqlparse::to_json, types::Database};
use iterate_text::file::lines::IterateFileLines;

#[allow(unused)]
static DEFAULT_JSON_FILTER: &str = r#"to_entries | map({table: .key, columns: .value.columns | map(select(.name | test("pass"; "i")))}) | map(select(.columns | length > 0))"#;

#[derive(ClapParser)]
#[command(about = format!("
Extract SQL from a text file

Usage:
--sql-file <sql_file>

--query <query>
"))]
pub struct Args {
    #[arg(short, long)]
    pub sql_file: String,

    #[arg(short, long)]
    pub query: Option<String>,

    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(ClapParser)]
pub enum Commands {
    #[command(about = "Mask PII from a SQL file")]
    MaskPII(MaskPIIArgs),
}

#[derive(ClapParser)]
pub struct MaskPIIArgs {
    #[arg(short, long)]
    masking_config: String,
}

pub fn exec() -> ExtractResult<Vec<String>> {
    let args = Args::parse();

    match args.cmd {
        Some(Commands::MaskPII(pii_args)) => {
            run_mask_pii_action(args.sql_file, &pii_args);
        }
        _ => {
            run_default_action(&args);
        }
    }
    Ok(vec![])
}

/// Mask PII from a SQL file
///  
/// 1. Parse the SQL file and print the JSON representation of the SQL.
/// 2. If the `--mask-pii` flag is provided, mask the PII in the SQL file.
///
/// Returns a list of statements to be executed.
fn run_mask_pii_action(sql_file: String, args: &MaskPIIArgs) -> ExtractResult<Vec<Statement>> {
    let sqlfile_path = Path::new(&sql_file);
    if !sqlfile_path.exists() {
        eprintln!("File {} does not exist", sqlfile_path.display());
        std::process::exit(1);
    }

    let masking_config = args.masking_config.clone();
    let config = parse_masking_config(&masking_config).expect("unable to load masking config");
    let dml_regex = Regex::new(r"^insert").unwrap();
    let mut out_sql: Vec<String> = vec![];
    let transform = Transform::new(&config);

    let file_descriptor = File::open(sqlfile_path).unwrap();
    let mut iter = IterateFileLines::from(file_descriptor);

    loop {
        let line = iter.next();
        if line.is_none() {
            break;
        }
        if dml_regex.is_match(line.as_ref().unwrap()) {
            let mut insert_block = line.clone().unwrap();
            let values = iter.next();
            insert_block.push_str(&values.unwrap());
            insert_block.retain(|c| c != '\n' && c != '\r');
            let dml_stmt = Insert::from(
                MySqlParser::parse(Rule::INSERT_STATEMENT, &insert_block)
                    .expect("Invalid input")
                    .next()
                    .expect("Unable to parse input"),
            );

            let mut dml_stmts = vec![dml_stmt];
            transform.mask_dml_stmts(dml_stmts.as_mut_slice());
            out_sql.push(dml_stmts[0].as_sql());
        } else {
            out_sql.push(line.unwrap());
        }
    }

    let out_sql = out_sql.join("");
    println!("{}", out_sql);

    Ok(vec![])
}
///
///
/// Default action.
///
/// 1. Parse the SQL file and print the JSON representation of the SQL.
/// 2. If the `--query` flag is provided, print the columns that contain the query string.
/// 3. If the `--mask-pii` flag is provided, mask the PII in the SQL file.
fn run_default_action(args: &Args) -> ExtractResult<Vec<String>> {
    let sqlfile_path = Path::new(&args.sql_file);
    if !sqlfile_path.exists() {
        eprintln!("File {} does not exist", sqlfile_path.display());
        std::process::exit(1);
    }
    let mut vals: Vec<String> = Vec::new();
    if let Some(query) = args.query.as_ref() {
        let res = simple_parse(sqlfile_path).expect("unable to load input file");
        // let input = to_json(res.clone());
        let result = find_pass_columns(&res, &query);
        println!("{}", serde_json::to_string(&result).unwrap());
    } else {
        let res = simple_parse(sqlfile_path).expect("unable to load input file");
        let input = to_json(res.clone());
        println!("{}", input.to_string());
        vals.push(input.to_string());
    }
    Ok(vals)
}

#[derive(Debug, serde::Serialize)]
struct Result {
    db_name: String,
    table_name: String,
    column_name: String,
}

fn find_pass_columns(databases: &Vec<Database>, query_str: &str) -> Vec<Result> {
    let mut result = Vec::new();

    for database in databases {
        let db_name = database.db_name.clone();
        for table in &database.tables {
            for column in &table.columns {
                if column
                    .name
                    .to_lowercase()
                    .contains(&query_str.to_lowercase())
                {
                    result.push(Result {
                        db_name: db_name.clone(),
                        table_name: table.name.clone(),
                        column_name: column.name.clone(),
                    });
                    break; // Move to the next table after finding the first matching column
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::{io::Write, path::PathBuf};
    use tempfile::TempDir;

    use super::*;

      fn create_test_masking_config(temp_dir: &TempDir) -> PathBuf {
        let temp_file_in_path = temp_dir.path().join("test.yaml");
        let test_config = r#"
columns:
    - account
patterns:
    - name: email
      regex: ^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$

rules:
    email: contact::email()"#;
        let mut file = std::fs::File::create(temp_file_in_path.clone()).unwrap();
        file.write_all(test_config.as_bytes()).unwrap();
        file.flush().unwrap();
        file.sync_data().unwrap();
        temp_file_in_path
    }

    fn create_temp_sql_with_insert(temp_dir: &TempDir) -> PathBuf {
        let temp_file_in_path = temp_dir.path().join("test.sql");
        let sql_single_insert = r#"USE `users`;\nINSERT INTO users (id, name, email, password) VALUES (1, 'John Doe', 'john.doe@example.com', 'password');"#;
        let mut file = std::fs::File::create(temp_file_in_path.clone()).unwrap();
        file.write_all(sql_single_insert.as_bytes()).unwrap();
        file.flush().unwrap();
        file.sync_data().unwrap();
        temp_file_in_path
    }
}
