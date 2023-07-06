use anyhow::anyhow;
use mysql::{from_row, Conn};

use crate::domain::diff::SnapshotDiff;
use crate::domain::snapshot::SnapshotId;

pub fn find_snapshot_diff(conn: &mut Conn, snapshot_id1: &SnapshotId, snapshot_id2: &SnapshotId) -> anyhow::Result<Option<SnapshotDiff>> {
    conn.query(format!("select data from snapshot_diff where snapshot_id1 = '{snapshot_id1}' and snapshot_id2 = '{snapshot_id2}'"))
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let data = from_row::<String>(row);
                    let snapshot_diff: SnapshotDiff = serde_json::from_str(&data).unwrap();
                    snapshot_diff
                })
                .next()
        })
        .map_err(|e| anyhow!(e))
}

pub fn insert_snapshot_diff(conn: &mut Conn, snapshot_diff: &SnapshotDiff) -> anyhow::Result<()> {
    conn.prep_exec(
        "insert into snapshot_diff values (?, ?, ?, ?)",
        (&snapshot_diff.diff_id, &snapshot_diff.snapshot_id1, &snapshot_diff.snapshot_id2, serde_json::to_string(snapshot_diff).unwrap()),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::create_connection;
    use crate::db::diff::{find_snapshot_diff, insert_snapshot_diff};
    use crate::db::project::insert_project;
    use crate::db::snapshot::insert_snapshot_summary;
    use crate::domain::diff::ColDiff::{Deleted, NoValue};
    use crate::domain::diff::{create_diff_id, SnapshotDiff, TableDiff};
    use crate::domain::project::Rdbms::Mysql;
    use crate::domain::project::{create_project_id, Project};
    use crate::domain::snapshot::ColValue::{SimpleNumber, SimpleString};
    use crate::domain::snapshot::{create_snapshot_id, ColValue, SnapshotSummary};

    fn n(s: &str) -> ColValue {
        SimpleNumber(s.to_string())
    }

    fn s(s: &str) -> ColValue {
        SimpleString(s.to_string())
    }

    #[test]
    fn snapshot_diff() -> anyhow::Result<()> {
        // setup

        let mut conn = create_connection()?;
        conn.prep_exec("delete from project", ())?;

        let project_id = create_project_id();

        let project = Project::new(&project_id, Mysql, "user", "password", "127.0.0.1", "3306", "test-db");
        insert_project(&mut conn, &project)?;

        let snapshot_id1 = create_snapshot_id();
        let snapshot_id2 = create_snapshot_id();

        let snapshot_summary1 = SnapshotSummary::new(&snapshot_id1, "test1", "2023-07-03 08:17:52");
        let snapshot_summary2 = SnapshotSummary::new(&snapshot_id2, "test2", "2023-07-03 08:42:35");
        insert_snapshot_summary(&mut conn, &project_id, &snapshot_summary1)?;
        insert_snapshot_summary(&mut conn, &project_id, &snapshot_summary2)?;

        let _table_name = "items".to_string();

        // find
        let table_snapshot_opt = find_snapshot_diff(&mut conn, &snapshot_id1, &snapshot_id2)?;
        assert_eq!(None, table_snapshot_opt);

        // insert
        let mut table_diff = TableDiff::init(&[&n("1"), &n("2")], &"id".to_string(), vec![&"name".to_string()]);
        table_diff.row_diffs1.insert(n("1").as_primary_value(), vec![("name".to_string(), Deleted(s("John")))].into_iter().collect());
        table_diff.row_diffs2.insert(n("2").as_primary_value(), vec![("name".to_string(), NoValue)].into_iter().collect());

        let snapshot_diff = SnapshotDiff::new(&create_diff_id(), &snapshot_id1, &snapshot_id2, vec![table_diff]);
        insert_snapshot_diff(&mut conn, &snapshot_diff)?;

        let table_snapshot_opt = find_snapshot_diff(&mut conn, &snapshot_id1, &snapshot_id2)?;
        assert_eq!(Some(snapshot_diff), table_snapshot_opt);

        Ok(())
    }
}
