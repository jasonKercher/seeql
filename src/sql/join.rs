use crate::sql::table::Table;

enum JoinType {
    INNER,
    LEFT,
    RIGHT,
    FULL,
}

pub struct Join {
    jointype: JoinType,
    pub table: Table,
    pub condition: String,
}
