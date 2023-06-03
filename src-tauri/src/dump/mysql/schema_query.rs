use crate::core::connector::Connector;
use anyhow::anyhow;
use itertools::Itertools;
use mysql::Value::NULL;
use mysql::{from_row, from_value, Conn, Opts, OptsBuilder, Value};
use r2d2::ManageConnection;
use r2d2_mysql::MysqlConnectionManager;

use crate::core::types::ColValue::Null;
use crate::core::types::{ColName, Row, TableName};
use crate::dump::mysql::column_parser;

pub struct TableSchema {
    pub table_name: TableName,
}

#[derive(Debug)]
pub struct ColumnSchemata {
    primary_col: ColumnSchema,
    cols: Vec<ColumnSchema>,
}

impl ColumnSchemata {
    pub fn get_col_names(self) -> (ColName, Vec<ColName>) {
        (self.primary_col.col_name, self.cols.into_iter().map(|col| col.col_name).collect())
    }

    pub fn get_col_refs(&self) -> Vec<&ColumnSchema> {
        let mut cols = self.cols.iter().collect_vec();
        cols.insert(0, &self.primary_col);
        cols
    }

    pub fn len(&self) -> usize {
        self.cols.len() + 1
    }
}

#[derive(Clone, Debug)]
pub struct ColumnSchema {
    pub col_name: ColName,
    pub data_type: String,
    pub column_type: String,
}

impl ColumnSchema {
    pub fn as_col(&self) -> String {
        match self.data_type.as_str() {
            "bit" => format!("bin({})", self.col_name),
            _ => self.col_name.to_string(),
        }
    }
}

pub fn create_connection(connector: &Connector) -> anyhow::Result<Conn> {
    let url = connector.get_url();
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

pub fn get_col_schemata(conn: &mut Conn, schema: &str, table_schema: &TableSchema) -> anyhow::Result<ColumnSchemata> {
    let unique_cols:Vec<ColumnSchema> = conn.query(
        format!("select column_name, data_type, column_type from information_schema.columns where table_schema = '{}' and table_name = '{}' and column_key = 'PRI'", schema, table_schema.table_name))
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row|{
                    let (column_name, data_type, column_type) = from_row(row);
                    ColumnSchema { col_name: column_name, data_type, column_type}
                })
                .collect_vec()
        }).map_err(|e|anyhow!(e))?;
    // todo: case [ no primary col ]
    let primary_col = unique_cols[0].clone();

    let cols:Vec<ColumnSchema> = conn.query(
        format!("select column_name, data_type, column_type from information_schema.columns where table_schema = '{}' and table_name = '{}' and column_key = '' order by ordinal_position", schema, table_schema.table_name))
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row|{
                    let (column_name, data_type, column_type) = from_row(row);
                    ColumnSchema { col_name: column_name, data_type, column_type}
                })
                .collect_vec()
        }).map_err(|e|anyhow!(e))?;

    Ok(ColumnSchemata { primary_col, cols })
}

pub fn get_rows(
    conn: &mut Conn,
    table_schema: &TableSchema,
    column_schemata: &ColumnSchemata,
) -> anyhow::Result<Vec<Row>> {
    let cols = column_schemata.get_col_refs();

    conn.query(format!("select {} from {}", cols.iter().map(|cs| cs.as_col()).join(","), table_schema.table_name))
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    (0..column_schemata.len())
                        .map(|i| {
                            let value: Value = row.get(i).unwrap();
                            if value == NULL {
                                Null
                            } else {
                                column_parser::parse(cols[i], from_value(value))
                            }
                        })
                        .collect_vec()
                })
                .map(|cols| Row::new(cols[0].clone().as_primary_value(), cols))
                .collect()
        })
        .map_err(|e| anyhow!(e))
}
