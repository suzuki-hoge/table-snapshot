#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub column_names: Vec<String>,
    pub rows: Vec<Row>,
}

#[derive(Debug)]
pub struct Row {
    pub columns: Vec<Column>,
}

#[derive(Debug)]
pub enum Column {
    UNumber(u64),
    INumber(i64),
}
