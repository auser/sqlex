use crate::parser::{
    types::{Column, ForeignKey, Index, PrimaryKey, TableOption},
    Rule,
};
use pest::iterators::Pair;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct CreateTable {
    pub name: String,
    pub if_not_exists: bool,
    pub columns: Vec<Column>,
    pub primary_key: Option<PrimaryKey>,
    pub foreign_keys: Vec<ForeignKey>,
    pub indexes: Vec<Index>,
    pub options: Vec<TableOption>,
}

impl From<Pair<'_, Rule>> for CreateTable {
    fn from(pair: Pair<'_, Rule>) -> Self {
        let mut inner = pair.into_inner();
        let element = inner.next().expect("rule");
        let (name, if_not_exists) = match element.as_rule() {
            Rule::IF_NOT_EXISTS => (
                inner
                    .next()
                    .expect("QUOTED_IDENTIFIER")
                    .as_str()
                    .trim_matches('`')
                    .to_string(),
                true,
            ),
            _ => (element.as_str().trim_matches('`').to_string(), false),
        };
        let mut columns = Vec::new();
        let mut primary_key = None;
        let mut foreign_keys = Vec::new();
        let mut indexes = Vec::new();
        let mut options = Vec::new();

        inner
            .next()
            .expect("table specs")
            .into_inner()
            .for_each(|spec| match spec.as_rule() {
                Rule::COLUMN_DEFINITION => columns.push(Column::from(spec)),
                Rule::PRIMARY_KEY => primary_key = Some(PrimaryKey::from(spec)),
                Rule::FOREIGN_KEY => foreign_keys.push(ForeignKey::from(spec)),
                Rule::INDEX_DEFINITION => indexes.push(Index::from(spec)),
                other => panic!("{other:?} is not a valid table column spec"),
            });
        inner
            .next()
            .expect("table options")
            .into_inner()
            .for_each(|opt| options.push(TableOption::from(opt)));

        Self {
            name,
            if_not_exists,
            columns,
            primary_key,
            foreign_keys,
            indexes,
            options,
        }
    }
}

impl Display for CreateTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut table_specs = self
            .columns
            .iter()
            .map(|col| col.to_string())
            .collect::<Vec<String>>();

        self.primary_key
            .as_ref()
            .inspect(|pk| table_specs.push(pk.to_string()));
        self.foreign_keys
            .iter()
            .for_each(|fk| table_specs.push(fk.to_string()));
        self.indexes
            .iter()
            .for_each(|idx| table_specs.push(idx.to_string()));

        write!(
            f,
            "CREATE TABLE{} `{}` (\n  {}\n){}",
            if self.if_not_exists {
                " IF NOT EXISTS"
            } else {
                ""
            },
            self.name,
            table_specs.join(",\n  "),
            self.options
                .iter()
                .map(|opt| format!(" {opt}"))
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{
        types::{DataType, DefaultValue, OnUpdateValue},
        MySqlParser,
    };
    use pest::Parser;

    #[test]
    fn can_parse_create_table() {
        let create_table = CreateTable::from(
            MySqlParser::parse(
                Rule::CREATE_TABLE,
                "CREATE TABLE IF NOT EXISTS `application` (
                    `Id` int NOT NULL AUTO_INCREMENT,
                    `ProductId` int NOT NULL DEFAULT '0',
                    `Name` varchar(36) NOT NULL,
                    `SecurityToken` varchar(200) DEFAULT NULL,
                    `RoutingKey` varchar(50) DEFAULT NULL,
                    `AdminPrivilege` tinyint(1) DEFAULT '0',
                    `CreatedAt` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                    PRIMARY KEY (`Id`),
                    KEY `fk_application_product` (`ProductId`),
                    CONSTRAINT `fk_application_product` FOREIGN KEY (`ProductId`) REFERENCES `product` (`Id`)
                ) ENGINE=InnoDB AUTO_INCREMENT=101 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;"
            )
            .expect("Invalid input")
            .next()
            .expect("Unable to parse input")
        );

        assert_eq!(create_table.name.as_str(), "application");
        assert!(create_table.if_not_exists);
        assert_eq!(create_table.columns.len(), 7);
        assert!(create_table.primary_key.is_some());
        assert_eq!(create_table.foreign_keys.len(), 1);
        assert_eq!(create_table.indexes.len(), 1);
        assert_eq!(create_table.options.len(), 4);
    }

    #[test]
    fn can_write_create_table() {
        assert_eq!(
            CreateTable {
                name: String::from("application"),
                if_not_exists: true,
                columns: vec![
                    Column {
                        name: String::from("Id"),
                        data_type: DataType::Int {
                            m: None,
                            unsigned: false,
                            zerofill: false
                        },
                        nullable: false,
                        default: None,
                        on_update: None,
                        auto_increment: true,
                        comment: None,
                    },
                    Column {
                        name: String::from("ProductId"),
                        data_type: DataType::Int {
                            m: None,
                            unsigned: false,
                            zerofill: false
                        },
                        nullable: false,
                        default: Some(DefaultValue::Text { value: String::from("0") }),
                        on_update: None,
                        auto_increment: false,
                        comment: None,
                    },
                    Column {
                        name: String::from("Name"),
                        data_type: DataType::Varchar {
                            m: Some(36),
                            charset_name: None,
                            collation_name: None
                        },
                        nullable: false,
                        default: None,
                        on_update: None,
                        auto_increment: false,
                        comment: None,
                    },
                    Column {
                        name: String::from("SecurityToken"),
                        data_type: DataType::Varchar {
                            m: Some(200),
                            charset_name: None,
                            collation_name: None
                        },
                        nullable: true,
                        default: Some(DefaultValue::Null),
                        on_update: None,
                        auto_increment: false,
                        comment: None,
                    },
                    Column {
                        name: String::from("RoutingKey"),
                        data_type: DataType::Varchar {
                            m: Some(50),
                            charset_name: None,
                            collation_name: None
                        },
                        nullable: true,
                        default: Some(DefaultValue::Null),
                        on_update: None,
                        auto_increment: false,
                        comment: None,
                    },
                    Column {
                        name: String::from("AdminPrivilege"),
                        data_type: DataType::TinyInt {
                            m: Some(1),
                            unsigned: false,
                            zerofill: false
                        },
                        nullable: true,
                        default: Some(DefaultValue::Text { value: String::from("0") }),
                        on_update: None,
                        auto_increment: false,
                        comment: None,
                    },
                    Column {
                        name: String::from("CreatedAt"),
                        data_type: DataType::DateTime { fsp: None },
                        nullable: false,
                        default: Some(DefaultValue::CurrentTimestamp { value: None }),
                        on_update: Some(OnUpdateValue::CurrentTimestamp { value: None }),
                        auto_increment: false,
                        comment: None,
                    },
                ],
                primary_key: Some(PrimaryKey {
                    name: None,
                    column_names: vec![String::from("Id")]
                }),
                foreign_keys: vec![ForeignKey {
                    name: Some(String::from("fk_application_product")),
                    local_column_names: vec![String::from("ProductId")],
                    foreign_column_names: vec![String::from("Id")],
                    foreign_table_name: String::from("product"),
                    on_update: None,
                },],
                indexes: vec![Index {
                    name: String::from("fk_application_product"),
                    columns: vec![String::from("ProductId")],
                    unique: false
                },],
                options: vec![
                    TableOption::Engine {
                        value: String::from("InnoDB")
                    },
                    TableOption::CharacterSet {
                        default: true,
                        value: String::from("utf8mb4")
                    },
                    TableOption::Collate {
                        default: false,
                        value: String::from("utf8mb4_0900_ai_ci")
                    },
                ],
            }
            .to_string()
            .as_str(),
            "CREATE TABLE IF NOT EXISTS `application` (\n  `Id` INT NOT NULL AUTO_INCREMENT,\n  `ProductId` INT NOT NULL DEFAULT '0',\n  `Name` VARCHAR (36) NOT NULL,\n  `SecurityToken` VARCHAR (200) DEFAULT NULL,\n  `RoutingKey` VARCHAR (50) DEFAULT NULL,\n  `AdminPrivilege` TINYINT (1) DEFAULT '0',\n  `CreatedAt` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,\n  PRIMARY KEY (`Id`),\n  CONSTRAINT `fk_application_product` FOREIGN KEY (`ProductId`) REFERENCES `product` (`Id`),\n  KEY `fk_application_product` (`ProductId`)\n) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci"
        );
    }
}
