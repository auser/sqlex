use pest_derive::Parser;

pub(crate) mod parse_utils;
pub mod statements;
pub mod types;

#[derive(Parser)]
#[grammar = "parser/sql.pest"]
struct MySqlParser;

// #[derive(Debug)]
// pub struct MyParser {
//     pub databases: HashMap<String, Database>,
//     pub current_database: Option<Database>,
// }

// impl MyParser {
//     pub fn new() -> Self {
//         Self {
//             databases: HashMap::new(),
//             current_database: None,
//         }
//     }

//     // pub fn with_parse(input: &str) -> ExtractResult<Self> {
//     //     let parser = Self::new();
//     //     let parsed_parser = parser.parse_mysqldump(input)?;
//     //     // self.databases.extend(parsed_databases);

//     //     Ok(parsed_parser)
//     // }

//     // pub fn parse(self, input: &str) -> ExtractResult<Self> {
//     //     Ok(self.parse_mysqldump(input)?)
//     // }

//     pub fn get_databases(&self) -> Vec<Database> {
//         self.databases.values().cloned().collect()
//     }

//     pub fn set_current_database(mut self, name: &str) -> Self {
//         self.current_database = self.databases.get(name).cloned();

//         self
//     }

//     // pub fn parse_mysqldump(mut self, input: &str) -> ExtractResult<Self> {
//     //     let mut parse_result = MySqlParser::parse(Rule::MYSQL_DUMP, input)
//     //         .context("invalid input")
//     //         .unwrap();
//     //     let mysqldump = parse_result
//     //         .next()
//     //         .context("unable to parse input")
//     //         .unwrap();
//     //     let mut current_database: Option<Database> = self.current_database.clone();

//     //     for pair in mysqldump.into_inner() {
//     //         match pair.as_rule() {
//     //             Rule::SQL_STATEMENT => {
//     //                 for inner_pair in pair.into_inner() {
//     //                     match inner_pair.as_rule() {
//     //                         Rule::CREATE_DATABASE => {
//     //                             let database = Database::from(inner_pair);

//     //                             if let Some(db) = current_database.take() {
//     //                                 if db.name != database.name {
//     //                                     self.insert_database(db);
//     //                                 }
//     //                             }

//     //                             current_database = Some(database);
//     //                         }
//     //                         Rule::USE_DATABASE => {
//     //                             let name = inner_pair
//     //                                 .into_inner()
//     //                                 .next()
//     //                                 .expect("unable to unwrap use_database name")
//     //                                 .as_str()
//     //                                 .trim_matches('`')
//     //                                 .to_string();

//     //                             current_database = Some(Database::new(name.to_string()));
//     //                         }
//     //                         Rule::CREATE_TABLE => {
//     //                             if let Some(ref mut db) = current_database {
//     //                                 let table = CreateTable::from(inner_pair);

//     //                                 db.tables.insert(table.name.clone(), table);
//     //                             }
//     //                         }
//     //                         Rule::ALTER_TABLE => {
//     //                             if let Some(ref mut db) = current_database {
//     //                                 parse_alter_table(inner_pair, db);
//     //                             }
//     //                         }
//     //                         Rule::DROP_TABLE => {
//     //                             if let Some(ref mut db) = current_database {
//     //                                 let table_name = inner_pair
//     //                                     .clone() // Clone the pair here
//     //                                     .into_inner()
//     //                                     .last()
//     //                                     .expect("unable to extract table name")
//     //                                     .as_str()
//     //                                     .trim_matches('`')
//     //                                     .to_string();

//     //                                 db.tables.remove(&table_name);
//     //                             }
//     //                         }
//     //                         // Rule::INSERT_STATEMENT => {
//     //                         //     if let Some(ref mut db) = current_database {
//     //                         //         let mut inner = inner_pair.into_inner();
//     //                         //         let table_name = inner
//     //                         //             .next()
//     //                         //             .unwrap()
//     //                         //             .as_str()
//     //                         //             .trim_matches('`')
//     //                         //             .to_string();

//     //                         //         if let Some(table) = db.tables.get_mut(&table_name) {
//     //                         //             table.inserts.push(parse_insert_statement(inner));
//     //                         //         }
//     //                         //     }
//     //                         // }
//     //                         // Rule::UPDATE_STATEMENT => {
//     //                         //     if let Some(ref mut db) = current_database {
//     //                         //         let update = parse_update_statement(inner_pair.into_inner());

//     //                         //         if let Some(table) = db.tables.get_mut(&update.table_name) {
//     //                         //             table.updates.push(update);
//     //                         //         }
//     //                         //     }
//     //                         // }
//     //                         // Rule::DELETE_STATEMENT => {
//     //                         //     let delete = parse_delete_statement(inner_pair.into_inner());

//     //                         //     if let Some(ref mut db) = current_database {
//     //                         //         if let Some(table) = db.tables.get_mut(&delete.table_name) {
//     //                         //             table.deletes.push(delete);
//     //                         //         }
//     //                         //     }
//     //                         // }
//     //                         // Rule::set_statement => {
//     //                         //     let set = parse_set_statement(statement);
//     //                         //     db.set_variables.insert(set.variable, set.value);
//     //                         // }
//     //                         // ... existing code ...
//     //                         _ => {}
//     //                     }
//     //                 }
//     //             }
//     //             _ => {}
//     //         }
//     //     }

//     //     if let Some(ref db) = current_database {
//     //         self.insert_database(db.clone());
//     //     }

//     //     // self.databases.extend(databases);
//     //     self.current_database = current_database;

//     //     Ok(self)
//     // }

//     fn insert_database(&mut self, db: Database) {
//         self.databases.insert(db.name.clone(), db);
//     }
// }

// fn parse_insert_statement(mut pairs: pest::iterators::Pairs<Rule>) -> Insert {
//     let column_pairs = pairs.next().expect("invalid insert statement").into_inner();
//     let value_pairs = pairs.next().expect("invalid insert statement").into_inner();
//     let columns: Vec<String> = column_pairs
//         .into_iter()
//         .map(|col| col.as_str().trim_matches('`').to_string())
//         .collect();
//     let values: Vec<String> = value_pairs
//         .into_iter()
//         .map(|value_list| {
//             value_list
//                 .into_inner()
//                 .map(|value| value.as_str().trim_matches('\'').to_string())
//                 .collect::<Vec<String>>()
//         })
//         .flatten()
//         .collect();

//     Insert::new(columns, values)
// }

// fn parse_update_statement(mut pairs: pest::iterators::Pairs<Rule>) -> Update {
// let table_pairs = pairs.next().expect("invalid update statement");
// let set_statement_pairs = pairs.next().expect("invalid update statement");
// let _where_statement_pairs = pairs.next().expect("invalid update statement");
// let table_name = table_pairs.as_str().trim_matches('`').to_string();
// let mut hm = HashMap::new();
// let mut set_statements = set_statement_pairs.into_inner();

// while let Some(ss) = set_statements.next() {
//     let var = ss.as_str().trim_matches('`').to_string();
//     let val = set_statements.next().unwrap().as_str().trim_matches('\'');
//     hm.insert(var, val.to_string());
// }

// Update::new(table_name, hm)
// }

// fn parse_delete_statement(mut pairs: pest::iterators::Pairs<Rule>) -> Delete {
//     let table_pairs = pairs.next().expect("invalid delete statement");
//     let _where_statement_pairs = pairs.next().expect("invalid delete statement");
//     let table_name = table_pairs.as_str().trim_matches('`').to_string();

//     Delete::new(table_name, None)
// }

// fn parse_alter_table(pair: pest::iterators::Pair<Rule>, db: &mut Database) {
//     let mut inner = pair.into_inner();
//     let table_name = inner.next().unwrap().as_str().trim_matches('`').to_string();

//     if let Some(table) = db.tables.get_mut(&table_name) {
//         for alter_spec in inner {
//             match alter_spec.as_rule() {
//                 Rule::ALTER_SPECIFICATION => {
//                     let mut spec_inner = alter_spec.into_inner();
//                     let action = spec_inner.next().unwrap().as_str();

//                     match action {
//                         "ADD" => {
//                             if spec_inner.peek().unwrap().as_rule() == Rule::COLUMN_DEFINITION {
//                                 let column = Column::from(spec_inner.next().unwrap());

//                                 table.columns.push(column);
//                             } else {
//                                 let index = Index::from(spec_inner.next().unwrap());

//                                 table.indexes.push(index);
//                             }
//                         }
//                         "MODIFY" => {
//                             let column = Column::from(spec_inner.next().unwrap());

//                             if let Some(existing_column) =
//                                 table.columns.iter_mut().find(|c| c.name == column.name)
//                             {
//                                 *existing_column = column;
//                             }
//                         }
//                         "DROP" => {
//                             let drop_type = spec_inner.next().unwrap().as_str();

//                             if drop_type == "COLUMN" {
//                                 let column_name = spec_inner
//                                     .next()
//                                     .unwrap()
//                                     .as_str()
//                                     .trim_matches('`')
//                                     .to_string();

//                                 table.columns.retain(|c| c.name != column_name);
//                             } else if drop_type == "INDEX" {
//                                 let index_name = spec_inner
//                                     .next()
//                                     .unwrap()
//                                     .as_str()
//                                     .trim_matches('`')
//                                     .to_string();
//                                 table.indexes.retain(|i| i.name != index_name);
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_create_database() {
    //     let input = "CREATE DATABASE `test_db`;";
    //     let result = MyParser::with_parse(input).unwrap();

    //     assert_eq!(
    //         result.databases.len(),
    //         1,
    //         "Expected 1 database, got {}",
    //         result.databases.len()
    //     );

    //     let databases = result.get_databases();

    //     if !databases.is_empty() {
    //         assert_eq!(databases[0].name, "test_db", "Database name mismatch");
    //         assert!(databases[0].tables.is_empty(), "Expected no tables");
    //         println!("OPTS> {:?}", databases[0].options);
    //         assert!(databases[0].options.len() == 0, "Expected no options");
    //     }
    // }

    // #[test]
    // fn test_create_database_with_constraints() {
    //     let input =
    //         "CREATE DATABASE `namedmanager` DEFAULT CHARACTER SET utf8 COLLATE utf8_general_ci;";
    //     let result = MyParser::with_parse(input).unwrap();

    //     assert_eq!(
    //         result.databases.len(),
    //         1,
    //         "Expected 1 database, got {}",
    //         result.databases.len()
    //     );

    //     let databases = result.get_databases();
    //     let database = databases.first().unwrap();

    //     assert!(database.options.len() == 2);

    //     let options = &database.options;

    //     assert_eq!(
    //         options.len(),
    //         2,
    //         "Expected 2 set variables, got {}",
    //         options.len()
    //     );
    //     assert_eq!(
    //         options[0].as_sql(),
    //         "CHARACTER_SET utf8",
    //         "Expected CHARACTER_SET to be utf8, got {}",
    //         options[0].as_sql(),
    //     );
    // }

    // #[test]
    // fn test_create_table() {
    //     let input = r#"
    //     --
    //     -- Table structure for table `config`
    //     --
    //     CREATE DATABASE `test_db`;
    //     USE `test_db`;
    //     CREATE TABLE IF NOT EXISTS `config` (
    //       `name` varchar(255) NOT NULL,
    //       `value` text NOT NULL,
    //       PRIMARY KEY  (`name`)
    //     ) ENGINE=InnoDB DEFAULT CHARSET=utf8;
    //     "#;
    //     let result = MyParser::with_parse(input).unwrap();

    //     assert_eq!(
    //         result.databases.len(),
    //         1,
    //         "Expected 1 database, got {}",
    //         result.databases.len()
    //     );
    //     let databases = result.get_databases();

    //     if !databases.is_empty() {
    //         assert_eq!(databases[0].name, "test_db", "Database name mismatch");
    //     }

    //     let db = databases[0].clone();

    //     assert_eq!(
    //         db.tables.len(),
    //         1,
    //         "Expected 1 table, got {}",
    //         db.tables.len()
    //     );

    //     if let Some(ref table) = db.tables.get("config") {
    //         assert_eq!(
    //             table.columns.len(),
    //             2,
    //             "Expected 2 columns, got {}",
    //             table.columns.len()
    //         );
    //     }
    // }

    // #[test]
    // fn test_create_table_with_primary_key() {
    //     let sql = r#"
    //     --
    //     -- Table structure for table `dns_record_types`
    //     --
    //     CREATE DATABASE `test_db`;
    //     USE `test_db`;

    //     CREATE TABLE IF NOT EXISTS `dns_record_types` (
    //       `id` int(10) unsigned NOT NULL auto_increment,
    //       `type` varchar(6) NOT NULL,
    //       `user_selectable` tinyint(1) NOT NULL default '0',
    //       PRIMARY KEY  (`id`)
    //     ) ENGINE=InnoDB  DEFAULT CHARSET=utf8 AUTO_INCREMENT=8 ;
    //     "#;
    //     let result = MyParser::with_parse(sql).unwrap();

    //     assert_eq!(
    //         result.databases.len(),
    //         1,
    //         "Expected 1 database, got {}",
    //         result.databases.len()
    //     );

    //     let databases = result.get_databases();

    //     if !databases.is_empty() {
    //         assert_eq!(databases[0].name, "test_db", "Database name mismatch");
    //     }

    //     let db = databases[0].clone();

    //     assert_eq!(
    //         db.tables.len(),
    //         1,
    //         "Expected 1 table, got {}",
    //         db.tables.len()
    //     );

    //     if let Some(ref table) = db.tables.get("dns_record_types") {
    //         assert_eq!(
    //             table.columns.len(),
    //             3,
    //             "Expected 3 columns, got {}",
    //             table.columns.len()
    //         );
    //     }
    // }

    // #[test]
    // fn test_insert_into_table() {
    //     let input = r#"
    //     CREATE DATABASE `test_db`;
    //     USE `test_db`;
    //     CREATE TABLE `users` (
    //         `id` INT NOT NULL AUTO_INCREMENT,
    //         `name` VARCHAR(255) NOT NULL,
    //         `email` VARCHAR(255) NOT NULL,
    //         PRIMARY KEY (`id`)
    //     );
    //     INSERT INTO `users` (`name`, `email`) VALUES ('John Doe', 'john.doe@example.com');
    //     "#;
    //     let result = MyParser::with_parse(input).unwrap();
    //     let databases = result.get_databases();

    //     assert_eq!(databases.len(), 1);

    //     let db = &databases[0];

    //     assert_eq!(db.name, "test_db");
    //     assert_eq!(db.tables.keys().len(), 1);

    //     let table = db.tables.get("users").unwrap();

    //     assert_eq!(table.columns.len(), 3);
    //     // assert_eq!(table.inserts.len(), 1);
    // }

    // #[test]
    // fn test_update_record_in_table() {
    //     let input = "UPDATE `users` SET `name` = 'Jane Doe' WHERE `id` = 1;";
    //     let parsed = get_test_database_and_table()
    //         .set_current_database("test_db")
    //         .parse(input)
    //         .unwrap();
    //     let databases = parsed.get_databases();

    //     assert_eq!(databases.len(), 1);

    //     let db = &databases[0];

    //     assert_eq!(db.name, "test_db");
    //     assert_eq!(db.tables.keys().len(), 1);

    //     let table = db.tables.get("users").unwrap();

    //     // assert_eq!(table.updates.len(), 1);
    // }

    // #[test]
    // fn test_delete_record_from_table() {
    //     let input = "DELETE FROM `users` WHERE `id` = 1;";
    //     let parsed = get_test_database_and_table()
    //         .set_current_database("test_db")
    //         .parse(input)
    //         .unwrap();
    //     let databases = parsed.get_databases();

    //     assert_eq!(databases.len(), 1);

    //     let db = &databases[0];

    //     assert_eq!(db.name, "test_db");
    //     assert_eq!(db.tables.keys().len(), 1);

    //     let table = db.tables.get("users").unwrap();

    //     // assert_eq!(table.deletes.len(), 1);
    // }

    // #[test]
    // fn test_multiple_statements() {
    //     let input = r#"
    //     INSERT INTO `users` (`name`, `email`, `password`) VALUES ('John Doe', 'john.doe@example.com', 'password');
    //     UPDATE `users` SET `name` = 'Jane Doe' WHERE `id` = 1;
    //     DELETE FROM `users` WHERE `id` = 1;
    //     SET @last_id = 1;
    //     "#;
    //     let result = get_test_database_and_table().parse(input).unwrap();
    //     let databases = result.get_databases();

    //     assert_eq!(databases.len(), 1);

    //     let db = &databases[0];

    //     assert_eq!(db.name, "test_db");
    //     assert_eq!(db.tables.keys().len(), 1);

    //     let table = db.tables.get("users").unwrap();

    //     assert_eq!(table.columns.len(), 4);
    //     // assert_eq!(table.inserts.len(), 1);
    //     // println!("insert in multiple_Statements test: {:#?}", table.inserts);
    //     // assert_eq!(table.updates.len(), 1);
    //     // assert_eq!(table.deletes.len(), 1);
    // }

    // #[test]
    // fn test_more_complicated_table() {
    //     let input = r#"
    //     CREATE TABLE IF NOT EXISTS `journal` (
    //         `id` int(11) NOT NULL auto_increment,
    //         `locked` tinyint(1) NOT NULL default '0',
    //         `journalname` varchar(50) NOT NULL,
    //         `type` varchar(20) NOT NULL,
    //         `userid` int(11) NOT NULL default '0',
    //         `customid` int(11) NOT NULL default '0',
    //         `timestamp` bigint(20) unsigned NOT NULL default '0',
    //         `content` text NOT NULL,
    //         `title` varchar(255) NOT NULL,
    //         PRIMARY KEY  (`id`),
    //         KEY `journalname` (`journalname`)
    //       ) ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=1 ;
    //     "#;
    //     let result = get_test_database_and_table().parse(input).unwrap();
    //     let databases = result.get_databases();

    //     assert_eq!(databases.len(), 1);
    // }

    // #[test]
    // fn test_character_following_varchar() {
    //     let input = r#"
    //     CREATE TABLE IF NOT EXISTS `name_servers` (
    //         `id` int(11) NOT NULL auto_increment,
    //         `server_primary` tinyint(1) NOT NULL,
    //         `server_name` varchar(255) character set latin1 NOT NULL,
    //         `server_description` text character set latin1 NOT NULL,
    //         `server_type` varchar(20) NOT NULL,
    //         `api_auth_key` varchar(255) character set latin1 NOT NULL,
    //         `api_sync_config` bigint(20) NOT NULL,
    //         `api_sync_log` bigint(20) NOT NULL,
    //         PRIMARY KEY  (`id`)
    //       ) ENGINE=InnoDB  DEFAULT CHARSET=utf8 AUTO_INCREMENT=1 ;
    //       "#;
    //     let result = get_test_database_and_table().parse(input).unwrap();
    //     let databases = result.get_databases();

    //     assert_eq!(databases.len(), 1);
    // }

    // fn get_test_database_and_table() -> MyParser {
    //     let input = r#"
    //     CREATE DATABASE `test_db`;
    //     USE `test_db`;
    //     CREATE TABLE `users` (
    //         `id` INT NOT NULL AUTO_INCREMENT,
    //         `name` VARCHAR(255) NOT NULL,
    //         `email` VARCHAR(255) NOT NULL,
    //         `password` VARCHAR(255) NOT NULL,
    //         PRIMARY KEY (`id`)
    //     );
    //     "#;
    //     let mut my_parser = MyParser::new();

    //     my_parser = my_parser.parse(input).unwrap();
    //     my_parser
    // }
}
