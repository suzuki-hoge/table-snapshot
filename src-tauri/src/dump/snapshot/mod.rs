use anyhow::anyhow;
use itertools::Itertools;
use mysql::{from_row, Conn};

use crate::core::types::{ColName, ColValue, Hash, PrimaryValue, Row, SnapshotId, TableName, TableSummary};

pub fn insert_table_summary(conn: &mut Conn, table_summary: &TableSummary) -> anyhow::Result<()> {
    conn.prep_exec(
        "insert into table_summary values (?, ?, ?, ?, ?)",
        (
            &table_summary.snapshot_id,
            &table_summary.table_name,
            &table_summary.hash,
            &table_summary.primary_col_name,
            serde_json::to_string(&table_summary.col_names).unwrap(),
        ),
    )?;
    Ok(())
}

pub fn insert_rows(conn: &mut Conn, table_summary: &TableSummary, rows: Vec<Row>) -> anyhow::Result<()> {
    for row in rows {
        conn.prep_exec(
            "insert into row_data values (?, ?, ?, ?, ?)",
            (
                &table_summary.snapshot_id,
                &table_summary.table_name,
                &row.hash,
                &row.primary_value,
                serde_json::to_string(&row.col_values.iter().map(|col_value| col_value.serialize()).collect_vec())
                    .unwrap(),
            ),
        )?;
    }
    Ok(())
}

pub fn _find_table_summaries(conn: &mut Conn, snapshot_id: &SnapshotId) -> anyhow::Result<Vec<TableSummary>> {
    conn.query(
        format!("select snapshot_id, table_name, hash, primary_col_name, col_names from table_summary where snapshot_id = '{snapshot_id}' order by table_name"))
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row|{
                    let (snapshot_id, table_name, hash, primary_col_name, col_names) = from_row::<(SnapshotId,TableName,Hash,ColName,String)>(row);
                    let col_names = serde_json::from_str(&col_names).unwrap();
                    TableSummary {snapshot_id, table_name, hash, primary_col_name, col_names}
                })
                .collect_vec()
        }).map_err(|e|anyhow!(e))
}

pub fn _find_rows(conn: &mut Conn, snapshot_id: &SnapshotId, table_name: &TableName) -> anyhow::Result<Vec<Row>> {
    conn.query(format!(
        "select hash, primary_value, col_values from row_data where snapshot_id = '{snapshot_id}' and table_name = '{table_name}' order by primary_value"
    ))
    .map(|result| {
        result
            .map(|x| x.unwrap())
            .map(|row| {
                let (hash, primary_value, col_values) = from_row::<(Hash, PrimaryValue, String)>(row);
                let col_values:Vec<String> = serde_json::from_str(&col_values).unwrap();
                let col_values = col_values.into_iter().map(ColValue::deserialize).collect();
                Row { hash, primary_value, col_values }
            })
            .collect_vec()
    })
    .map_err(|e| anyhow!(e))
}
