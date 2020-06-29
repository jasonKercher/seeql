use crate::query::join::Join;
use crate::query::table::Table;

pub struct UpdateStatement {
    pub update_table: Table,
    pub join_tables: Vec<Join>,
    pub set_column: Vec<String>,
    pub set_value: Vec<String>,
    pub from_clause: String,
    pub where_clause: String,
}
