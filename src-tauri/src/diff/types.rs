use crate::core::types::{ColName, ColValue};
use std::collections::HashMap;

pub type ColDiffs<'a> = HashMap<&'a ColName, ColDiff<'a>>;

#[derive(Debug)]
pub struct SnapshotDiff<'a> {
    pub primary_col_name: &'a ColName,
    pub total_col_names: Vec<&'a ColName>,
    pub rows1: Vec<ColDiffs<'a>>,
    pub rows2: Vec<ColDiffs<'a>>,
}

impl<'a> SnapshotDiff<'a> {
    pub fn init(primary_col_name: &'a ColName, total_col_names: Vec<&'a ColName>) -> Self {
        Self { primary_col_name, total_col_names, rows1: vec![], rows2: vec![] }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum ColDiff<'a> {
    NoValue,
    Stay(&'a ColValue),
    Added(&'a ColValue),
    Deleted(&'a ColValue),
}
