use crate::database::types::Column::{
    BinaryString, BitNumber, DateString, JsonString, ParseError, SimpleNumber, SimpleString,
};
use itertools::Itertools;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub column_names: Vec<String>,
    pub rows: Vec<Row>,
}

#[derive(Debug)]
pub struct Row {
    pub columns: Vec<Column>,
    pub hash: String,
}

impl Row {
    pub fn new(columns: Vec<Column>) -> Self {
        let cols = columns.iter().map(|c| c.raw()).join(",");
        let hash = format!("{:?}", md5::compute(cols));
        Self { columns, hash }
    }
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
            BinaryString(_) => "binary".to_string(),
            JsonString(v) => v.to_string(),
            ParseError(_) => "parse error".to_string(),
        }
    }

    fn raw(&self) -> String {
        match self {
            SimpleNumber(v) => v.to_string(),
            BitNumber(v) => v.to_string(),
            SimpleString(v) => v.to_string(),
            DateString(v) => v.to_string(),
            BinaryString(v) => v.to_string(),
            JsonString(v) => v.to_string(),
            ParseError(_) => "parse error".to_string(),
        }
    }
}
