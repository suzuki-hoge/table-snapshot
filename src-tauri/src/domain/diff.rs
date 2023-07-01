use std::collections::HashMap;

use crate::domain::diff::ColDiff::*;
use crate::domain::schema::{ColName, Hash, PrimaryColName};
use crate::domain::snapshot::{ColValue, PrimaryColValue, SnapshotId, TableSnapshot};

pub struct SnapshotDiff<'a> {
    snapshot_id1: SnapshotId,
    snapshot_id2: SnapshotId,
    tables: Vec<TableDiff<'a>>,
}

pub struct TableDiff<'a> {
    pub primary_col_name: &'a PrimaryColName,
    pub col_names: Vec<&'a ColName>,
    pub row_diff1: Vec<RowDiff<'a>>,
    pub row_diff2: Vec<RowDiff<'a>>,
}

impl<'a> TableDiff<'a> {
    pub fn init(primary_col_name: &'a PrimaryColName, col_names: Vec<&'a ColName>) -> Self {
        Self { primary_col_name, col_names, row_diff1: vec![], row_diff2: vec![] }
    }
}

type RowDiff<'a> = HashMap<&'a ColName, ColDiff<'a>>;

#[derive(Eq, PartialEq, Debug)]
pub enum ColDiff<'a> {
    NoValue,
    Stay(&'a ColValue),
    Added(&'a ColValue),
    Deleted(&'a ColValue),
}

// here

pub fn create_table_diff<'a>(
    table_snapshot1: Option<&'a TableSnapshot>,
    table_snapshot2: Option<&'a TableSnapshot>,
) -> TableDiff<'a> {
    match (table_snapshot1, table_snapshot2) {
        (Some(table_snapshot1), Some(table_snapshot2)) => take_table_snapshot_diff(table_snapshot1, table_snapshot2),
        (None, Some(table_snapshot2)) => create_missing_pair_diff(table_snapshot2, 2),
        (Some(table_snapshot1), None) => create_missing_pair_diff(table_snapshot1, 1),
        (None, None) => unreachable!(),
    }
}

type Rows<'a> = HashMap<&'a PrimaryColValue, (&'a Hash, Cols<'a>)>;
type Cols<'a> = HashMap<&'a ColName, &'a ColValue>;

fn take_table_snapshot_diff<'a>(
    table_snapshot1: &'a TableSnapshot,
    table_snapshot2: &'a TableSnapshot,
) -> TableDiff<'a> {
    let total_col_names = table_snapshot1.merge_col_names(table_snapshot2);

    let mut snapshot_diff = TableDiff::init(&table_snapshot1.primary_col_name, total_col_names.clone());

    let rows1 = parse_rows(table_snapshot1);
    let rows2 = parse_rows(table_snapshot2);

    for primary_col_value in table_snapshot1.merge_primary_col_values(table_snapshot2) {
        let row1 = rows1.get(primary_col_value);
        let row2 = rows2.get(primary_col_value);

        match (row1, row2) {
            // 2 つの行の Hash が一致している場合は、スキップする
            (Some((hash1, _)), Some((hash2, _))) if hash1 == hash2 => {}

            // 同一の主キー値が片方にしかない場合は、片方の全列を差分として登録する
            (None, Some((_, cols2))) => snapshot_diff
                .row_diff2
                .push(cols2.iter().map(|(&col_name, &col_value)| (col_name, Added(col_value))).collect()),

            // 同一の主キー値が片方にしかない場合は、片方の全列を差分として登録する
            (Some((_, cols1)), None) => snapshot_diff
                .row_diff1
                .push(cols1.iter().map(|(&col_name, &col_value)| (col_name, Deleted(col_value))).collect()),

            // 2 つの行の Hash が一致しない場合は、列ごとに差分をとる
            (Some((_, cols1)), Some((_, cols2))) => {
                let get_col_diff_f1 = |col_name| match (cols1.get(col_name), cols2.get(col_name)) {
                    (Some(&col_value1), Some(&col_value2)) if col_value1 == col_value2 => Stay(col_value1),
                    (Some(&col_value1), _) => Deleted(col_value1),
                    (None, _) => NoValue,
                };
                let get_col_diff_f2 = |col_name| match (cols1.get(col_name), cols2.get(col_name)) {
                    (Some(&col_value1), Some(&col_value2)) if col_value1 == col_value2 => Stay(col_value2),
                    (_, Some(&col_value2)) => Added(col_value2),
                    (_, None) => NoValue,
                };
                snapshot_diff
                    .row_diff1
                    .push(total_col_names.iter().map(|&col_name| (col_name, get_col_diff_f1(col_name))).collect());
                snapshot_diff
                    .row_diff2
                    .push(total_col_names.iter().map(|&col_name| (col_name, get_col_diff_f2(col_name))).collect());
            }

            (None, None) => unreachable!(),
        };
    }

    snapshot_diff
}

fn parse_rows<'a>(table_snapshot: &'a TableSnapshot) -> Rows<'a> {
    let mut rows: Rows = HashMap::new();

    for row_snapshot in &table_snapshot.row_snapshots {
        let cols: Cols<'a> = table_snapshot
            .col_names
            .iter()
            .enumerate()
            .map(|(i, col_name)| (col_name, &row_snapshot.col_values[i]))
            .collect();
        rows.insert(&row_snapshot.primary_col_value, (&row_snapshot.hash, cols));
    }

    rows
}

fn create_missing_pair_diff(table_snapshot: &TableSnapshot, n: usize) -> TableDiff {
    let mut snapshot_diff =
        TableDiff::init(&table_snapshot.primary_col_name, table_snapshot.col_names.iter().collect());

    let rows = parse_rows(table_snapshot);

    for primary_col_value in table_snapshot.get_primary_col_values() {
        if let Some((_, cols)) = rows.get(primary_col_value) {
            let row_diff = cols
                .iter()
                .map(|(&col_name, &col_value)| (col_name, if n == 1 { Deleted(col_value) } else { Added(col_value) }))
                .collect();

            if n == 1 {
                snapshot_diff.row_diff1.push(row_diff);
            } else {
                snapshot_diff.row_diff2.push(row_diff);
            }
        }
    }

    snapshot_diff
}

#[cfg(test)]
mod tests_create_snapshot_diff {
    

    use crate::domain::diff::ColDiff::*;
    use crate::domain::diff::{create_table_diff, ColDiff, RowDiff};
    use crate::domain::snapshot::ColValue::{SimpleNumber, SimpleString};
    use crate::domain::snapshot::{ColValue, RowSnapshot, TableSnapshot};

    fn n(s: &str) -> ColValue {
        SimpleNumber(s.to_string())
    }

    fn s(s: &str) -> ColValue {
        SimpleString(s.to_string())
    }

    fn mk_table_snapshot(
        table_name: &str,
        primary_col_name: &str,
        col_names: Vec<&str>,
        row_snapshots: Vec<RowSnapshot>,
    ) -> TableSnapshot {
        TableSnapshot::new(
            table_name.to_string(),
            primary_col_name.to_string(),
            col_names.iter().map(|col_name| col_name.to_string()).collect(),
            row_snapshots,
        )
    }

    fn mk_act<'a>(row_diff: &'a RowDiff, name: &str) -> &'a ColDiff<'a> {
        row_diff.get(&name.to_string()).unwrap()
    }

    #[test]
    fn test_row_0_and_row_1() {
        let rows2 = vec![RowSnapshot::new(vec![n("1"), s("John")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["name"], rows2);

        let act = create_table_diff(None, Some(&table_snapshot2));

        assert_eq!(0, act.row_diff1.len());

        assert_eq!(1, act.row_diff2.len());
        assert_eq!(&Added(&s("John")), mk_act(&act.row_diff2[0], "name"));
    }

    #[test]
    fn test_row_1_and_row_0() {
        let rows1 = vec![RowSnapshot::new(vec![n("1"), s("John")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let act = create_table_diff(Some(&table_snapshot1), None);

        assert_eq!(1, act.row_diff1.len());
        assert_eq!(&Deleted(&s("John")), mk_act(&act.row_diff1[0], "name"));

        assert_eq!(0, act.row_diff2.len());
    }

    #[test]
    fn test_row_1_and_row_1() {
        let rows1 = vec![RowSnapshot::new(vec![n("1"), s("John")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let rows2 = vec![RowSnapshot::new(vec![n("1"), s("Jane")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["name"], rows2);

        let act = create_table_diff(Some(&table_snapshot1), Some(&table_snapshot2));

        assert_eq!(1, act.row_diff1.len());
        assert_eq!(&Deleted(&s("John")), mk_act(&act.row_diff1[0], "name"));

        assert_eq!(1, act.row_diff2.len());
        assert_eq!(&Added(&s("Jane")), mk_act(&act.row_diff2[0], "name"));
    }

    #[test]
    fn test_row_2_and_row_1() {
        let rows1 = vec![RowSnapshot::new(vec![n("1"), s("John")]), RowSnapshot::new(vec![n("2"), s("Jack")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let rows2 = vec![RowSnapshot::new(vec![n("1"), s("Jane")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["name"], rows2);

        let act = create_table_diff(Some(&table_snapshot1), Some(&table_snapshot2));

        assert_eq!(2, act.row_diff1.len());
        assert_eq!(&Deleted(&s("John")), mk_act(&act.row_diff1[0], "name"));
        assert_eq!(&Deleted(&s("Jack")), mk_act(&act.row_diff1[1], "name"));

        assert_eq!(1, act.row_diff2.len());
        assert_eq!(&Added(&s("Jane")), mk_act(&act.row_diff2[0], "name"));
    }

    #[test]
    fn test_row_1_and_row_2() {
        let rows1 = vec![RowSnapshot::new(vec![n("1"), s("John")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let rows2 = vec![RowSnapshot::new(vec![n("1"), s("John")]), RowSnapshot::new(vec![n("2"), s("Jack")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["name"], rows2);

        let act = create_table_diff(Some(&table_snapshot1), Some(&table_snapshot2));

        assert_eq!(0, act.row_diff1.len());

        assert_eq!(1, act.row_diff2.len());
        assert_eq!(&Added(&s("Jack")), mk_act(&act.row_diff2[0], "name"));
    }

    #[test]
    fn test_row_1_and_row_1_nomatch_cols() {
        let rows1 = vec![RowSnapshot::new(vec![n("1"), s("John")])];
        let table_snapshot1 = mk_table_snapshot("user", "id", vec!["name"], rows1);

        let rows2 = vec![RowSnapshot::new(vec![n("1"), n("39")])];
        let table_snapshot2 = mk_table_snapshot("user", "id", vec!["age"], rows2);

        let act = create_table_diff(Some(&table_snapshot1), Some(&table_snapshot2));

        assert_eq!(1, act.row_diff1.len());
        assert_eq!(&Deleted(&s("John")), mk_act(&act.row_diff1[0], "name"));
        assert_eq!(&NoValue, mk_act(&act.row_diff1[0], "age"));

        assert_eq!(1, act.row_diff2.len());
        assert_eq!(&NoValue, mk_act(&act.row_diff2[0], "name"));
        assert_eq!(&Added(&n("39")), mk_act(&act.row_diff2[0], "age"));
    }
}
