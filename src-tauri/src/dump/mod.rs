use mysql::Conn;

use crate::domain::project::Project;
use crate::domain::project::Rdbms::Mysql;
use crate::domain::snapshot::{create_snapshot_id, SnapshotId, TableSnapshot};
use crate::dump::adapter::TargetDbAdapter;
use crate::dump::mysql80::TargetDbMysql80;

mod adapter;
mod mysql80;

pub fn dump(project: &Project, _conn: &mut Conn) -> anyhow::Result<SnapshotId> {
    let mut adapter = match &project.rdbms {
        Mysql => TargetDbMysql80::new(project),
    }?;

    let snapshot_id = create_snapshot_id();

    let table_schemata = adapter.get_table_schemata()?;

    for table_schema in table_schemata {
        let col_schemata = adapter.get_col_schemata(&table_schema)?;

        let row_snapshots = adapter.get_row_snapshots(&table_schema, &col_schemata)?;

        let (primary_col_name, col_names) = col_schemata.get_all_col_names();
        let _table_snapshot =
            TableSnapshot::new(table_schema.table_name.clone(), primary_col_name, col_names, row_snapshots);

        // insert_table_summary(&mut own_conn, &table_summary)?;
        // insert_rows(&mut own_conn, &table_summary, rows)?;
    }

    Ok(snapshot_id)
}
