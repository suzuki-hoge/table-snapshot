use anyhow::anyhow;
use itertools::Itertools;
use mysql::{from_row, Conn};

use crate::domain::project::ProjectId;
use crate::domain::snapshot::{SnapshotId, SnapshotSummary, TableSnapshot};

pub fn all_snapshot_summaries(conn: &mut Conn, project_id: &ProjectId) -> anyhow::Result<Vec<SnapshotSummary>> {
    conn.query(format!("select snapshot_id, snapshot_name, create_at from snapshot_summary where project_id = '{project_id}' order by create_at"))
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (snapshot_id, snapshot_name, create_at) = from_row::<(SnapshotId, String, String)>(row);
                    SnapshotSummary::new(&snapshot_id, snapshot_name, create_at)
                })
                .collect()
        })
        .map_err(|e| anyhow!(e))
}

pub fn insert_snapshot_summary(conn: &mut Conn, project_id: &ProjectId, snapshot_summary: &SnapshotSummary) -> anyhow::Result<()> {
    conn.prep_exec(
        "insert into snapshot_summary values (?, ?, ?, ?)",
        (&snapshot_summary.snapshot_id, project_id, &snapshot_summary.snapshot_name, &snapshot_summary.create_at),
    )?;
    Ok(())
}

pub fn update_snapshot_summary(conn: &mut Conn, snapshot_summary: &SnapshotSummary) -> anyhow::Result<()> {
    conn.prep_exec(
        "update snapshot_summary set snapshot_name = ? where snapshot_id = ?",
        (&snapshot_summary.snapshot_name, &snapshot_summary.snapshot_id),
    )?;
    Ok(())
}

pub fn delete_snapshot_summary(conn: &mut Conn, snapshot_id: &SnapshotId) -> anyhow::Result<()> {
    conn.prep_exec("delete from snapshot_summary where snapshot_id = ?", vec![snapshot_id])?;
    Ok(())
}

pub fn find_table_snapshots(conn: &mut Conn, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<TableSnapshot>> {
    conn.query(format!("select data from table_snapshot where snapshot_id = '{snapshot_id}'"))
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let data = from_row::<String>(row);
                    let table_snapshot: TableSnapshot = serde_json::from_str(&data).unwrap();
                    table_snapshot
                })
                .collect_vec()
        })
        .map_err(|e| anyhow!(e))
}

pub fn insert_table_snapshot(conn: &mut Conn, snapshot_id: &SnapshotId, table_snapshot: &TableSnapshot) -> anyhow::Result<()> {
    conn.prep_exec(
        "insert into table_snapshot values (?, ?, ?)",
        (&snapshot_id, &table_snapshot.table_name, &serde_json::to_string(table_snapshot).unwrap()),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::db::create_connection;
    use crate::db::project::insert_project;
    use crate::db::snapshot::{
        all_snapshot_summaries, delete_snapshot_summary, find_table_snapshots, insert_snapshot_summary, insert_table_snapshot,
        update_snapshot_summary,
    };
    use crate::domain::project::Rdbms::Mysql;
    use crate::domain::project::{create_project_id, Project};
    use crate::domain::snapshot::ColValue::{SimpleNumber, SimpleString};
    use crate::domain::snapshot::{create_snapshot_id, ColValue, RowSnapshot, SnapshotSummary, TableSnapshot};

    #[test]
    fn snapshot_summary() -> anyhow::Result<()> {
        // setup

        let mut conn = create_connection()?;
        conn.prep_exec("delete from project", ())?;

        let project_id = create_project_id();

        let project = Project::new(&project_id, "test-project", Mysql, "user", "password", "127.0.0.1", "3306", "test-db");
        insert_project(&mut conn, &project)?;

        // all
        let snapshot_summaries = all_snapshot_summaries(&mut conn, &project_id)?;
        assert_eq!(0, snapshot_summaries.len());

        let snapshot_id = create_snapshot_id();

        // insert
        let snapshot_summary1 = SnapshotSummary::new(&snapshot_id, "test", "2023-07-03 08:17:52");
        insert_snapshot_summary(&mut conn, &project_id, &snapshot_summary1)?;

        let snapshot_summaries = all_snapshot_summaries(&mut conn, &project_id)?;
        assert_eq!(1, snapshot_summaries.len());
        assert_eq!(&snapshot_summary1, &snapshot_summaries[0]);

        // update
        let snapshot_summary2 = SnapshotSummary::new(&snapshot_id, "test2", "2023-07-03 08:17:52");
        update_snapshot_summary(&mut conn, &snapshot_summary2)?;

        let snapshot_summaries = all_snapshot_summaries(&mut conn, &project_id)?;
        assert_eq!(1, snapshot_summaries.len());
        assert_eq!(&snapshot_summary2, &snapshot_summaries[0]);

        // delete
        delete_snapshot_summary(&mut conn, &snapshot_id)?;

        let snapshot_summaries = all_snapshot_summaries(&mut conn, &project_id)?;
        assert_eq!(0, snapshot_summaries.len());

        Ok(())
    }

    fn n(s: &str) -> ColValue {
        SimpleNumber(s.to_string())
    }

    fn s(s: &str) -> ColValue {
        SimpleString(s.to_string())
    }

    #[test]
    fn table_snapshot() -> anyhow::Result<()> {
        // setup

        let mut conn = create_connection()?;
        conn.prep_exec("delete from project", ())?;

        let project_id = create_project_id();

        let project = Project::new(&project_id, "test-project", Mysql, "user", "password", "127.0.0.1", "3306", "test-db");
        insert_project(&mut conn, &project)?;

        let snapshot_id = create_snapshot_id();

        let snapshot_summary = SnapshotSummary::new(&snapshot_id, "test", "2023-07-03 08:17:52");
        insert_snapshot_summary(&mut conn, &project_id, &snapshot_summary)?;

        let table_name = "items".to_string();

        // find
        let table_snapshots = find_table_snapshots(&mut conn, &snapshot_id)?;
        assert_eq!(0, table_snapshots.len());

        // insert
        let row_snapshot1 = RowSnapshot::new(vec![n("1"), s("123"), n("1200")]);
        let row_snapshot2 = RowSnapshot::new(vec![n("2"), s("456"), n("560")]);
        let table_snapshot =
            TableSnapshot::new(&table_name, "id".to_string(), vec!["code".to_string(), "price".to_string()], vec![row_snapshot1, row_snapshot2]);
        insert_table_snapshot(&mut conn, &snapshot_id, &table_snapshot)?;

        let table_snapshots = find_table_snapshots(&mut conn, &snapshot_id)?;
        assert_eq!(vec![table_snapshot], table_snapshots);

        Ok(())
    }
}
