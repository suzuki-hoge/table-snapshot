use crate::database::base::Column::{
    BinaryString, BitNumber, DateString, JsonString, ParseError, SimpleNumber, SimpleString,
};

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

#[derive(Eq, PartialEq, Debug)]
pub enum Column {
    SimpleNumber(String),
    BitNumber(String),
    SimpleString(String),
    DateString(String),
    BinaryString(String),
    JsonString(String),
    ParseError(String),
}

impl Column {
    pub fn show(&self) -> String {
        match self {
            SimpleNumber(v) => v.to_string(),
            BitNumber(v) => format!("bit({v})"),
            SimpleString(v) => format!(r#""{v}""#),
            DateString(v) => format!(r#""{v}""#),
            BinaryString(v) => v.to_string(),
            JsonString(v) => v.to_string(),
            ParseError(_) => "parse error".to_string(),
        }
    }
}
