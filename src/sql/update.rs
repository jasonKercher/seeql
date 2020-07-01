//use crate::sql::join::Join;
use crate::sql::table::Table;

pub struct UpdateStatement {
    pub update_table: Table,
    pub tables: Vec<Table>,
    pub set_column: Vec<String>,
    pub set_assignment: Vec<String>,
    pub set_value: Vec<String>,
    pub from_clause: String,
    pub where_clause: String,
    pub start: isize,
    pub stop: isize,
    pub query: String,
}

impl UpdateStatement {
    pub fn new() -> UpdateStatement {
        UpdateStatement {
            update_table: Table::new(String::from("")),
            tables: Vec::new(),
            set_column: Vec::new(),
            set_assignment: Vec::new(),
            set_value: Vec::new(),
            from_clause: String::new(),
            where_clause: String::new(),
            start: 0,
            stop: 0,
            query: String::new(),
        }
    }
}
