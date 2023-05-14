use crate::core::connector::Connector;
use itertools::Itertools;

use crate::core::types::Table;
use crate::dump::mysql::schema_query::{create_connection, get_column_schemata, get_rows, get_table_schemata};

mod column_parser;
mod schema_query;

pub fn dump(connector: &Connector) -> anyhow::Result<()> {
    let tables = get_tables(connector)?;
    for table in &tables {
        println!("{}", &table.name);
        println!("    {}", &table.column_names.join(", "));
        for row in &table.rows {
            println!("    {} ( {} )", row.columns.iter().map(|c| c.show()).join(", "), row.hash);
        }
    }

    Ok(())
}

fn get_tables(connector: &Connector) -> anyhow::Result<Vec<Table>> {
    let mut conn = create_connection(connector)?;

    let table_schemata = get_table_schemata(&mut conn, &connector.schema)?;

    let mut tables = vec![];

    for table_schema in table_schemata {
        let column_schemata = get_column_schemata(&mut conn, &connector.schema, &table_schema)?;

        let rows = get_rows(&mut conn, &table_schema, &column_schemata)?;

        tables.push(Table {
            name: table_schema.table_name,
            column_names: column_schemata.into_iter().map(|column_schema| column_schema.column_name).collect_vec(),
            rows,
        });
    }

    Ok(tables)
}
