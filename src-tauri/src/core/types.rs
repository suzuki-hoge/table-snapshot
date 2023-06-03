use std::collections::HashMap;

use itertools::Itertools;
use uuid::Uuid;

use crate::core::types::ColValue::{
    BinaryString, BitNumber, DateString, JsonString, Null, ParseError, SimpleNumber, SimpleString,
};

pub type SnapshotId = String;

pub type TableName = String;

pub type ColName = String;

pub type PrimaryValue = String;

pub type Hash = String;

#[derive(Debug)]
pub struct TableSummary {
    pub snapshot_id: SnapshotId,
    pub table_name: TableName,
    pub hash: Hash,
    pub primary_col_name: ColName,
    pub col_names: Vec<ColName>,
}

impl TableSummary {
    pub fn new(
        snapshot_id: SnapshotId,
        table_name: TableName,
        primary_col_name: ColName,
        col_names: Vec<ColName>,
        rows: &[Row],
    ) -> Self {
        let row_hashes = rows.iter().map(|row| &row.hash).join("");
        let hash = format!("{:?}", md5::compute(format!("{}{}{}", primary_col_name, col_names.join(""), row_hashes)));
        Self { snapshot_id, table_name, hash, primary_col_name, col_names }
    }
}

#[derive(Debug)]
pub struct Snapshot {
    snapshot_id: SnapshotId,
    tables: Vec<Table>,
}

impl Snapshot {
    pub fn new(tables: Vec<Table>) -> Self {
        let snapshot_id = Uuid::new_v4().to_string();
        Self { snapshot_id, tables }
    }

    pub fn show(&self) {
        println!("{}", &self.snapshot_id);
        for table in &self.tables {
            println!("{}", &table.name);
            println!("    {}", &table.col_names.join(", "));
            for row in &table.rows {
                println!("    {} ( {} )", row.col_values.iter().map(|c| c.show()).join(", "), row.hash);
            }
        }
    }
}

#[derive(Debug)]
pub struct Table {
    pub name: TableName,
    pub primary_col_name: ColName,
    pub col_names: Vec<ColName>,
    pub rows: Vec<Row>,
}

impl Table {
    pub fn new<S: Into<String>>(name: S, col_names: Vec<S>, rows: Vec<Row>) -> Self {
        // todo: id
        Self {
            name: name.into(),
            primary_col_name: "id".to_string(),
            col_names: col_names.into_iter().map(|col_name| col_name.into()).collect(),
            rows,
        }
    }

    pub fn get_rows_with_col_name(&self) -> Vec<HashMap<&ColName, &ColValue>> {
        self.rows
            .iter()
            .map(|row| self.col_names.iter().enumerate().map(|(i, col_name)| (col_name, &row.col_values[i])).collect())
            .collect()
    }
}

#[derive(Debug)]
pub struct Row {
    pub hash: Hash,
    pub primary_value: PrimaryValue,
    pub col_values: Vec<ColValue>,
}

impl Row {
    pub fn new<S: Into<PrimaryValue>>(primary_value: S, col_values: Vec<ColValue>) -> Self {
        let raws = col_values.iter().map(|c| c.raw()).join(",");
        let hash = format!("{:?}", md5::compute(raws));
        Self { hash, primary_value: primary_value.into(), col_values }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ColValue {
    SimpleNumber(String),
    BitNumber(String),
    SimpleString(String),
    DateString(String),
    BinaryString(String),
    JsonString(String),
    Null,
    ParseError,
}

impl ColValue {
    pub fn as_primary_value(&self) -> PrimaryValue {
        match self {
            SimpleNumber(v) | BitNumber(v) | SimpleString(v) | DateString(v) | JsonString(v) => v.to_string(),
            Null | BinaryString(_) | ParseError => unreachable!(),
        }
    }

    pub fn show(&self) -> String {
        match self {
            SimpleNumber(v) => v.to_string(),
            BitNumber(v) => format!("bit({v})"),
            SimpleString(v) => format!(r#""{v}""#),
            DateString(v) => format!(r#""{v}""#),
            BinaryString(_) => "binary".to_string(),
            JsonString(v) => v.to_string(),
            Null => "<null>".to_string(),
            ParseError => "parse error".to_string(),
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
            Null => format!("{:?}", md5::compute("<null>")),
            ParseError => "parse error".to_string(),
        }
    }

    pub fn serialize(&self) -> String {
        match self {
            SimpleNumber(v) => format!(r#""SimpleNumber.{v}""#),
            BitNumber(v) => format!(r#""BitNumber.{v}""#),
            SimpleString(v) => format!(r#""SimpleString.{v}""#),
            DateString(v) => format!(r#""DateString.{v}""#),
            BinaryString(v) => format!(r#""BinaryString.{v}""#),
            JsonString(v) => format!(r#""JsonString.{}""#, v.replace('"', r#"\""#)),
            Null => r#""Null.""#.to_string(),
            ParseError => r#""ParseError.""#.to_string(),
        }
    }

    pub fn deserialize(s: String) -> Self {
        let s = &s[1..s.len() - 1];
        let sp = s.split('.').collect_vec();
        let p1 = sp[0];
        let p2 = sp[1];
        match p1 {
            "SimpleNumber" => SimpleNumber(p2.to_string()),
            "BitNumber" => BitNumber(p2.to_string()),
            "SimpleString" => SimpleString(p2.to_string()),
            "DateString" => DateString(p2.to_string()),
            "BinaryString" => BinaryString(p2.to_string()),
            "JsonString" => JsonString(p2.replace('\\', "")),
            "Null" => Null,
            "ParseError" => ParseError,
            _ => unreachable!(),
        }
    }
}
