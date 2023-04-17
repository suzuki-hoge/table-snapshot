use anyhow::anyhow;
use itertools::Itertools;
use mysql::{from_row, Conn, Opts, OptsBuilder};
use r2d2::ManageConnection;
use r2d2_mysql::MysqlConnectionManager;

use crate::database::base::Row;
use crate::database::mysql::column;

pub struct TableSchema {
    pub table_name: String,
}

#[derive(Debug)]
pub struct ColumnSchema {
    pub column_name: String,
    pub data_type: String,
    pub column_type: String,
}

impl ColumnSchema {
    pub fn as_col(&self) -> String {
        match self.data_type.as_str() {
            "bit" => format!("bin({})", self.column_name),
            _ => self.column_name.to_string(),
        }
    }
}

pub fn create_connection(user: &str, password: &str, host: &str, port: &str, schema: &str) -> anyhow::Result<Conn> {
    let url = format!("mysql://{user}:{password}@{host}:{port}/{schema}");
    let opt = Opts::from_url(&url).unwrap();
    let builder = OptsBuilder::from_opts(opt);
    let manager = MysqlConnectionManager::new(builder);
    manager.connect().map_err(|e| anyhow!(e))
}

pub fn get_table_schemata(conn: &mut Conn, schema: &str) -> anyhow::Result<Vec<TableSchema>> {
    conn.query(format!("select table_name from information_schema.tables where table_schema = '{schema}'"))
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let table_name = from_row(row);
                    TableSchema { table_name }
                })
                .collect()
        })
        .map_err(|e| anyhow!(e))
}

pub fn get_column_schemata(
    conn: &mut Conn,
    schema: &str,
    table_schema: &TableSchema,
) -> anyhow::Result<Vec<ColumnSchema>> {
    conn.query(
        format!("select column_name, data_type, column_type from information_schema.columns where table_schema = '{}' and table_name = '{}' order by ordinal_position", schema, table_schema.table_name))
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row|{
                    let (column_name, data_type, column_type) = from_row(row);
                    ColumnSchema { column_name, data_type, column_type}
                })
                .collect()
        }).map_err(|e|anyhow!(e))
}

pub fn get_rows(
    conn: &mut Conn,
    table_schema: &TableSchema,
    column_schemata: &[ColumnSchema],
) -> anyhow::Result<Vec<Row>> {
    conn.query(format!(
        "select {} from {}",
        column_schemata.iter().map(|cs| cs.as_col()).join(","),
        table_schema.table_name
    ))
    .map(|result| {
        result
            .map(|x| x.unwrap())
            .map(|row| {
                (0..column_schemata.len())
                    .map(|i| column::parse(&column_schemata[i], row.get(i).unwrap()))
                    .collect_vec()
            })
            .map(|columns| Row { columns })
            .collect()
    })
    .map_err(|e| anyhow!(e))
}
