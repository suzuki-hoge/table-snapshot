use anyhow::anyhow;
use itertools::Itertools;
use mysql::{from_row, Conn};

use crate::domain::schema::TableName;
use crate::domain::snapshot::{RowSnapshot, SnapshotId, SnapshotSummary, TableSnapshot};

pub fn _insert_table_snapshot(
    conn: &mut Conn,
    snapshot_id: &SnapshotId,
    table_snapshot: &TableSnapshot,
) -> anyhow::Result<()> {
    conn.prep_exec(
        "insert into table_snapshot values (?, ?, ?, ?, ?)",
        (
            snapshot_id,
            &table_snapshot.table_name,
            &table_snapshot.primary_col_name,
            serde_json::to_string(&table_snapshot.col_names).unwrap(),
            &table_snapshot.hash,
        ),
    )?;
    Ok(())
}

pub fn _insert_row_snapshots(
    conn: &mut Conn,
    snapshot_id: &SnapshotId,
    table_name: &TableName,
    row_snapshots: Vec<RowSnapshot>,
) -> anyhow::Result<()> {
    for row in row_snapshots {
        conn.prep_exec(
            "insert into row_snapshot values (?, ?, ?, ?, ?)",
            (
                snapshot_id,
                &row.hash,
                &row.primary_col_value.serialize(),
                serde_json::to_string(&row.col_values.iter().map(|col_value| col_value.serialize()).collect_vec())
                    .unwrap(),
                table_name,
            ),
        )?;
    }
    Ok(())
}

pub fn _find_snapshot_summaries(conn: &mut Conn) -> anyhow::Result<Vec<SnapshotSummary>> {
    conn.query("select snapshot_id, table_name from table_snapshot order by create_at")
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (snapshot_id, table_name) = from_row::<(SnapshotId, TableName)>(row);
                    SnapshotSummary::new(snapshot_id, table_name)
                })
                .collect()
        })
        .map_err(|e| anyhow!(e))
}

// pub fn find_rows(conn: &mut Conn, snapshot_id: &SnapshotId, table_name: &TableName) -> anyhow::Result<Vec<Row>> {
//     conn.query(format!(
//         "select hash, primary_value, col_values from row_snapshot where snapshot_id = '{snapshot_id}' and table_name = '{table_name}' order by primary_value"
//     ))
//         .map(|result| {
//             result
//                 .map(|x| x.unwrap())
//                 .map(|row| {
//                     let (hash, primary_value, col_values) = from_row::<(Hash, PrimaryValue, String)>(row);
//                     let col_values:Vec<String> = serde_json::from_str(&col_values).unwrap();
//                     let col_values = col_values.into_iter().map(ColValue::deserialize).collect();
//                     Row { hash, primary_value, col_values }
//                 })
//                 .collect()
//         })
//         .map_err(|e| anyhow!(e))
// }
