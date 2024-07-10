use lazy_static::lazy_static;
use tera::Tera;

mod column;
mod data_type;
mod database_option;
mod default_value;
mod delete;
mod foreign_key;
mod index;
mod insert;
mod insert_value;
mod insert_values;
mod on_update_value;
mod primary_key;
mod priority;
mod set;
mod table_option;
mod update;

pub use column::Column;
pub use data_type::DataType;
pub use database_option::DatabaseOption;
pub use default_value::DefaultValue;
pub use delete::Delete;
pub use foreign_key::ForeignKey;
pub use index::Index;
pub use insert::Insert;
pub use insert_value::InsertValue;
pub use insert_values::InsertValues;
pub use on_update_value::OnUpdateValue;
pub use primary_key::PrimaryKey;
pub use priority::Priority;
pub use set::Set;
pub use table_option::TableOption;
pub use update::Update;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/parser/types/**/*.sql") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error: {e}");

                ::std::process::exit(1);
            }
        }
    };
}
