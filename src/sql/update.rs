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
    pub query: String,
    pub comments: String,
    pub lineno: isize,
}

impl UpdateStatement {
    pub fn new(lineno: isize) -> UpdateStatement {
        UpdateStatement {
            update_table: Table::new(String::from("")),
            tables: Vec::new(),
            set_column: Vec::new(),
            set_assignment: Vec::new(),
            set_value: Vec::new(),
            from_clause: String::new(),
            where_clause: String::new(),
            comments: String::new(),
            query: String::new(),
            lineno,
        }
    }
}
