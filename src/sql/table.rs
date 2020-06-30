pub struct Table {
    pub alias: String,
    pub name: String,
}

impl Table {
    pub fn new(name: String) -> Table {
        Table {
            alias: String::new(),
            name,
        }
    }
}
