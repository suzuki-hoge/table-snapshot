use std::cmp::max;
use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;

use crate::core::types::{ColName, ColValue, Hash, PrimaryValue, Table};
use crate::diff::types::ColDiff::{Added, Deleted, NoValue, Stay};
use crate::diff::types::{ColDiffs, SnapshotDiff};

mod types;

type Rows<'a> = HashMap<&'a PrimaryValue, (&'a Hash, Cols<'a>)>;
type Cols<'a> = HashMap<&'a ColName, &'a ColValue>;

pub fn create_snapshot_diff<'a>(table1: Option<&'a Table>, table2: Option<&'a Table>) -> SnapshotDiff<'a> {
    match (table1, table2) {
        (Some(table1), Some(table2)) => take_diff(table1, table2),
        (None, Some(table2)) => no_pair(table2, 2),
        (Some(table1), None) => no_pair(table1, 1),
        (None, None) => unreachable!(),
    }
}

fn no_pair(table: &Table, n: usize) -> SnapshotDiff {
    let primary_values = table.rows.iter().map(|row| &row.primary_value).collect_vec();

    let total_col_names = table.col_names.iter().collect_vec();

    let mut snapshot_diff = SnapshotDiff::init(&table.primary_col_name, total_col_names.clone());

    let rows = parse_rows(table);

    for primary_value in primary_values {
        if let Some((_, cols)) = rows.get(primary_value) {
            let diff = cols
                .iter()
                .map(|(&col_name, &col_value)| (col_name, if n == 1 { Deleted(col_value) } else { Added(col_value) }))
                .collect();

            if n == 1 {
                snapshot_diff.rows1.push(diff);
            } else {
                snapshot_diff.rows2.push(diff);
            }
        }
    }

    snapshot_diff
}

fn take_diff<'a>(table1: &'a Table, table2: &'a Table) -> SnapshotDiff<'a> {
    let primary_values = gather_primary_values(table1, table2);

    let total_col_names = merge_col_names(&table1.col_names, &table2.col_names);

    let mut snapshot_diff = SnapshotDiff::init(&table1.primary_col_name, total_col_names.clone());

    let rows1 = parse_rows(table1);
    let rows2 = parse_rows(table2);

    for primary_value in primary_values {
        let row1 = rows1.get(primary_value);
        let row2 = rows2.get(primary_value);

        let diff: (ColDiffs, ColDiffs) = match (row1, row2) {
            (Some((hash1, _)), Some((hash2, _))) if hash1 == hash2 => (HashMap::new(), HashMap::new()),
            (None, Some((_, cols2))) => {
                (HashMap::new(), cols2.iter().map(|(&col_name, &col_value)| (col_name, Added(col_value))).collect())
            }
            (Some((_, cols1)), None) => {
                (cols1.iter().map(|(&col_name, &col_value)| (col_name, Deleted(col_value))).collect(), HashMap::new())
            }
            (Some((_, cols1)), Some((_, cols2))) => {
                let deleted = |col_name| match (cols1.get(col_name), cols2.get(col_name)) {
                    (Some(&v1), Some(&v2)) if v1 == v2 => Stay(v1),
                    (Some(&v1), _) => Deleted(v1),
                    (None, _) => NoValue,
                };
                let added = |col_name| match (cols1.get(col_name), cols2.get(col_name)) {
                    (Some(&v1), Some(&v2)) if v1 == v2 => Stay(v2),
                    (_, Some(&v2)) => Added(v2),
                    (_, None) => NoValue,
                };
                (
                    total_col_names.iter().map(|&col_name| (col_name, deleted(col_name))).collect(),
                    total_col_names.iter().map(|&col_name| (col_name, added(col_name))).collect(),
                )
            }
            (None, None) => unreachable!(),
        };

        if !diff.0.is_empty() {
            snapshot_diff.rows1.push(diff.0);
        }
        if !diff.1.is_empty() {
            snapshot_diff.rows2.push(diff.1);
        }
    }

    snapshot_diff
}

fn merge_col_names<'a>(col_names1: &'a [ColName], col_names2: &'a [ColName]) -> Vec<&'a ColName> {
    let mut col_names = vec![];
    for i in 0..max(col_names1.len(), col_names2.len()) {
        if i < col_names1.len() && !col_names.contains(&&col_names1[i]) {
            col_names.push(&col_names1[i]);
        }
        if i < col_names2.len() && !col_names.contains(&&col_names2[i]) {
            col_names.push(&col_names2[i]);
        }
    }
    col_names
}

fn gather_primary_values<'a>(table1: &'a Table, table2: &'a Table) -> Vec<&'a PrimaryValue> {
    let mut set = BTreeSet::new();

    for row in &table1.rows {
        set.insert(&row.primary_value);
    }
    for row in &table2.rows {
        set.insert(&row.primary_value);
    }

    set.into_iter().collect_vec()
}

fn parse_rows<'a>(table: &'a Table) -> Rows<'a> {
    let mut rows: Rows = HashMap::new();
    for row in &table.rows {
        let cols: Cols<'a> =
            table.col_names.iter().enumerate().map(|(i, col_name)| (col_name, &row.col_values[i])).collect();
        rows.insert(&row.primary_value, (&row.hash, cols));
    }
    rows
}

#[cfg(test)]
mod tests_merge_col_names {
    use itertools::Itertools;

    use crate::diff::merge_col_names;

    #[test]
    fn test_col_2_and_col_2_same() {
        let cols1 = vec!["id", "name"].into_iter().map(|s| s.to_string()).collect_vec();

        let cols2 = vec!["id", "name"].into_iter().map(|s| s.to_string()).collect_vec();

        let act = merge_col_names(&cols1, &cols2);

        assert_eq!(vec!["id", "name"], act);
    }

    #[test]
    fn test_col_2_and_col_2() {
        let cols1 = vec!["id", "name"].into_iter().map(|s| s.to_string()).collect_vec();

        let cols2 = vec!["id", "age"].into_iter().map(|s| s.to_string()).collect_vec();

        let act = merge_col_names(&cols1, &cols2);

        assert_eq!(vec!["id", "name", "age"], act);
    }

    #[test]
    fn test_col_2_and_col_1() {
        let cols1 = vec!["id", "name"].into_iter().map(|s| s.to_string()).collect_vec();

        let cols2 = vec!["id"].into_iter().map(|s| s.to_string()).collect_vec();

        let act = merge_col_names(&cols1, &cols2);

        assert_eq!(vec!["id", "name"], act);
    }

    #[test]
    fn test_col_1_and_col_2() {
        let cols1 = vec!["name"].into_iter().map(|s| s.to_string()).collect_vec();

        let cols2 = vec!["id", "name"].into_iter().map(|s| s.to_string()).collect_vec();

        let act = merge_col_names(&cols1, &cols2);

        assert_eq!(vec!["name", "id"], act);
    }
}

#[cfg(test)]
mod tests_create_snapshot_diff {
    use std::collections::HashMap;

    use crate::core::types::ColValue::{SimpleNumber, SimpleString};
    use crate::core::types::{ColValue, Row, Table};
    use crate::diff::create_snapshot_diff;
    use crate::diff::tests_create_snapshot_diff::ColDiffKind::{KAdded, KDeleted, KNoValue, KStay};
    use crate::diff::types::ColDiff::{Added, Deleted, NoValue, Stay};
    use crate::diff::types::{ColDiff, ColDiffs};

    fn n(s: &str) -> ColValue {
        SimpleNumber(s.to_string())
    }

    fn s(s: &str) -> ColValue {
        SimpleString(s.to_string())
    }

    #[derive(Eq, PartialEq, Debug)]
    #[allow(clippy::enum_variant_names)]
    pub enum ColDiffKind {
        KNoValue,
        KStay,
        KAdded,
        KDeleted,
    }

    impl ColDiffKind {
        fn of(col_diff: &ColDiff) -> Self {
            match col_diff {
                NoValue => KNoValue,
                Stay(_) => KStay,
                Deleted(_) => KDeleted,
                Added(_) => KAdded,
            }
        }
    }

    fn mk_exp(ts: Vec<(&'static str, ColDiffKind)>) -> HashMap<&'static str, ColDiffKind> {
        ts.into_iter().collect()
    }

    fn mk_act<'a>(col_diff: &'a ColDiffs<'a>) -> HashMap<&'a str, ColDiffKind> {
        col_diff.iter().map(|(&k, v)| (k.as_str(), ColDiffKind::of(v))).collect()
    }

    #[test]
    fn test_row_0_and_row_1() {
        let rows2 = vec![Row::new("1", vec![n("1"), s("John")])];
        let table2 = Table::new("user", vec!["id", "name"], rows2);

        let act = create_snapshot_diff(None, Some(&table2));

        assert_eq!(0, act.rows1.len());
        assert_eq!(1, act.rows2.len());
        assert_eq!(mk_exp(vec![("id", KAdded), ("name", KAdded)]), mk_act(&act.rows2[0]));
    }

    #[test]
    fn test_row_1_and_row_0() {
        let rows1 = vec![Row::new("1", vec![n("1"), s("John")])];
        let table1 = Table::new("user", vec!["id", "name"], rows1);

        let act = create_snapshot_diff(Some(&table1), None);

        assert_eq!(1, act.rows1.len());
        assert_eq!(0, act.rows2.len());
        assert_eq!(mk_exp(vec![("id", KDeleted), ("name", KDeleted)]), mk_act(&act.rows1[0]));
    }

    #[test]
    fn test_row_1_and_row_1() {
        let rows1 = vec![Row::new("1", vec![n("1"), s("John")])];
        let table1 = Table::new("user", vec!["id", "name"], rows1);

        let rows2 = vec![Row::new("1", vec![n("1"), s("Jane")])];
        let table2 = Table::new("user", vec!["id", "name"], rows2);

        let act = create_snapshot_diff(Some(&table1), Some(&table2));

        assert_eq!(1, act.rows1.len());
        assert_eq!(1, act.rows2.len());
        assert_eq!(mk_exp(vec![("id", KStay), ("name", KDeleted)]), mk_act(&act.rows1[0]));
        assert_eq!(mk_exp(vec![("id", KStay), ("name", KAdded)]), mk_act(&act.rows2[0]));
    }

    #[test]
    fn test_row_2_and_row_1() {
        let rows1 = vec![Row::new("1", vec![n("1"), s("John")]), Row::new("2", vec![n("2"), s("Jack")])];
        let table1 = Table::new("user", vec!["id", "name"], rows1);

        let rows2 = vec![Row::new("1", vec![n("1"), s("Jane")])];
        let table2 = Table::new("user", vec!["id", "name"], rows2);

        let act = create_snapshot_diff(Some(&table1), Some(&table2));

        assert_eq!(2, act.rows1.len());
        assert_eq!(1, act.rows2.len());
        assert_eq!(mk_exp(vec![("id", KStay), ("name", KDeleted)]), mk_act(&act.rows1[0]));
        assert_eq!(mk_exp(vec![("id", KStay), ("name", KAdded)]), mk_act(&act.rows2[0]));
        assert_eq!(mk_exp(vec![("id", KDeleted), ("name", KDeleted)]), mk_act(&act.rows1[1]));
    }

    #[test]
    fn test_row_1_and_row_2() {
        let rows1 = vec![Row::new("1", vec![n("1"), s("John")])];
        let table1 = Table::new("user", vec!["id", "name"], rows1);

        let rows2 = vec![Row::new("1", vec![n("1"), s("John")]), Row::new("2", vec![n("2"), s("Jack")])];
        let table2 = Table::new("user", vec!["id", "name"], rows2);

        let act = create_snapshot_diff(Some(&table1), Some(&table2));

        assert_eq!(0, act.rows1.len());
        assert_eq!(1, act.rows2.len());
        assert_eq!(mk_exp(vec![("id", KAdded), ("name", KAdded)]), mk_act(&act.rows2[0]));
    }

    #[test]
    fn test_row_1_and_row_1_nomatch_cols() {
        let rows1 = vec![Row::new("1", vec![n("1"), s("John")])];
        let table1 = Table::new("user", vec!["id", "name"], rows1);

        let rows2 = vec![Row::new("1", vec![n("1"), n("39")])];
        let table2 = Table::new("user", vec!["id", "age"], rows2);

        let act = create_snapshot_diff(Some(&table1), Some(&table2));

        assert_eq!(1, act.rows1.len());
        assert_eq!(1, act.rows2.len());
        assert_eq!(mk_exp(vec![("id", KStay), ("name", KDeleted), ("age", KNoValue)]), mk_act(&act.rows1[0]));
        assert_eq!(mk_exp(vec![("id", KStay), ("name", KNoValue), ("age", KAdded)]), mk_act(&act.rows2[0]));
    }
}
