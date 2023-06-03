use crate::core::connector::Connector;
use crate::core::types::TableSummary;
use itertools::Itertools;
use uuid::Uuid;

use crate::dump::mysql::schema_query::{create_connection, get_col_schemata, get_rows, get_table_schemata};

mod column_parser;
mod schema_query;

pub fn dump(connector: &Connector) -> anyhow::Result<()> {
    let mut conn = create_connection(connector)?;

    let snapshot_id = Uuid::new_v4().to_string();

    let table_schemata = get_table_schemata(&mut conn, &connector.schema)?;

    dbg!(&snapshot_id);
    for table_schema in table_schemata {
        let col_schemata = get_col_schemata(&mut conn, &connector.schema, &table_schema)?;
        // save
        let col_names = col_schemata.get_cols().iter().map(|column_schema| &column_schema.col_name).collect_vec();
        dbg!(&col_names);

        // save
        let rows = get_rows(&mut conn, &table_schema, &col_schemata)?;
        for row in &rows {
            dbg!(&row.hash);
        }

        // create table summary
        let table_summary = TableSummary::new(&snapshot_id, &table_schema.table_name, &rows);
        dbg!(&table_summary);

        // save
    }

    Ok(())
}
