pub enum Operation {
    Create(dyn Sql),
    Update(dyn Sql),
    Delete(dyn Sql),
    Insert(dyn Sql),
    Alter(dyn Sql),
    Drop(dyn Sql),
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub operations: Vec<Operation>,
}

impl Table {
    pub fn new(name: String) -> Self {
        Table {
            name,
            columns: Vec::new(),
            operations: Vec::new(),
        }
    }
}

impl From<CreateTable> for Table {
    fn from(create_table: CreateTable) -> Self {
        let table_name = trimmed_str(create_table.into_inner());
        Table::new(table_name)
    }
}
