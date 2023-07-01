use itertools::Itertools;
use std::cmp::max;
use std::collections::BTreeSet;
use uuid::Uuid;

use crate::domain::schema::{ColName, Hash, PrimaryColName, PrimaryValue, TableName};
use crate::domain::snapshot::ColValue::*;

pub type SnapshotId = String;

pub fn create_snapshot_id() -> SnapshotId {
    Uuid::new_v4().to_string()
}

pub struct SnapshotSummary {
    pub snapshot_id: SnapshotId,
    pub table_name: TableName,
}

impl SnapshotSummary {
    pub fn new(snapshot_id: SnapshotId, table_name: TableName) -> Self {
        Self { snapshot_id, table_name }
    }
}

pub struct TableSnapshot {
    pub table_name: TableName,
    pub primary_col_name: PrimaryColName,
    pub col_names: Vec<ColName>,
    pub hash: Hash,
    pub row_snapshots: Vec<RowSnapshot>,
}

impl TableSnapshot {
    pub fn new(
        table_name: TableName,
        primary_col_name: PrimaryColName,
        col_names: Vec<ColName>,
        row_snapshots: Vec<RowSnapshot>,
    ) -> Self {
        let row_hashes = row_snapshots.iter().map(|row_snapshot| &row_snapshot.hash).join("");
        let hash = format!("{:?}", md5::compute(format!("{}{}{}", primary_col_name, col_names.join(""), row_hashes)));
        Self { table_name, primary_col_name, col_names, hash, row_snapshots }
    }

    pub fn get_primary_col_values(&self) -> Vec<&PrimaryColValue> {
        self.row_snapshots.iter().map(|row_snapshot| &row_snapshot.primary_col_value).collect()
    }

    pub fn merge_primary_col_values<'a>(&'a self, other: &'a Self) -> Vec<&'a PrimaryColValue> {
        let mut set = BTreeSet::new();

        for row in &self.row_snapshots {
            set.insert(&row.primary_col_value);
        }
        for row in &other.row_snapshots {
            set.insert(&row.primary_col_value);
        }

        set.into_iter().collect_vec()
    }

    pub fn merge_col_names<'a>(&'a self, other: &'a Self) -> Vec<&'a ColName> {
        let mut result = vec![];

        for i in 0..max(self.col_names.len(), other.col_names.len()) {
            if i < self.col_names.len() && !result.contains(&&self.col_names[i]) {
                result.push(&self.col_names[i]);
            }
            if i < other.col_names.len() && !result.contains(&&other.col_names[i]) {
                result.push(&other.col_names[i]);
            }
        }

        result
    }
}

pub struct RowSnapshot {
    pub primary_col_value: PrimaryColValue,
    pub col_values: Vec<ColValue>,
    pub hash: Hash,
}

impl RowSnapshot {
    pub fn new(col_values: Vec<ColValue>) -> Self {
        let col_raw_values = col_values.iter().map(|c| c.as_raw_value()).join(",");
        let hash = format!("{:?}", md5::compute(col_raw_values));

        let primary_col_value = col_values[0].clone();
        let col_values = col_values.into_iter().dropping(1).collect_vec();

        Self { primary_col_value, col_values, hash }
    }
}

pub type PrimaryColValue = ColValue;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
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
            SimpleNumber(_) | BitNumber(_) | SimpleString(_) | DateString(_) | JsonString(_) => self.as_display_value(),
            Null | BinaryString(_) | ParseError => unreachable!(),
        }
    }

    pub fn as_display_value(&self) -> String {
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

    fn as_raw_value(&self) -> String {
        match self {
            SimpleNumber(v) => v.to_string(),
            BitNumber(v) => v.to_string(),
            SimpleString(v) => v.to_string(),
            DateString(v) => v.to_string(),
            BinaryString(v) => format!("{:?}", md5::compute(v)),
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
