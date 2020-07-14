pub struct Table {
    pub alias: String,
    pub query: String,
    pub name: String,
}

impl Table {
    pub fn new(name: String) -> Table {
        Table {
            alias: String::new(),
            query: name.clone(),
            name,
        }
    }
}
