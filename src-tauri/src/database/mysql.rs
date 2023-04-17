use itertools::Itertools;

use crate::database::mysql::query::{create_connection, get_column_schemata, get_rows, get_table_schemata};
use crate::database::types::Table;

mod parser;
mod query;

pub fn dump(user: &str, password: &str, host: &str, port: &str, schema: &str) -> anyhow::Result<()> {
    let tables = get_tables(user, password, host, port, schema)?;
    for table in &tables {
        println!("{}", &table.name);
        println!("    {}", &table.column_names.join(", "));
        for row in &table.rows {
            println!("    {} ( {} )", row.columns.iter().map(|c| c.show()).join(", "), row.hash);
        }
    }

    Ok(())
}

fn get_tables(user: &str, password: &str, host: &str, port: &str, schema: &str) -> anyhow::Result<Vec<Table>> {
    let mut conn = create_connection(user, password, host, port, schema)?;

    let table_schemata = get_table_schemata(&mut conn, schema)?;

    let mut tables = vec![];

    for table_schema in table_schemata {
        let column_schemata = get_column_schemata(&mut conn, schema, &table_schema)?;

        let rows = get_rows(&mut conn, &table_schema, &column_schemata)?;

        tables.push(Table {
            name: table_schema.table_name,
            column_names: column_schemata.into_iter().map(|column_schema| column_schema.column_name).collect_vec(),
            rows,
        });
    }

    Ok(tables)
}
