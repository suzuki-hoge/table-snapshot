use anyhow::anyhow;
use itertools::Itertools;
use mysql::{from_row, Conn, Opts, OptsBuilder};
use r2d2::ManageConnection;
use r2d2_mysql::MysqlConnectionManager;

pub fn dump(
    user: &String,
    password: &String,
    host: &String,
    port: &String,
    schema: &String,
) -> anyhow::Result<()> {
    let mut conn = create_connection(user, password, host, port, schema)?;

    let tables = get_tables(&mut conn, schema)?;

    for table in tables {
        let columns = get_columns(&mut conn, schema, &table)?;

        let rows = get_rows(&mut conn, &table, columns.len())?;

        dbg!(&rows);
    }

    Ok(())
}
fn create_connection(
    user: &String,
    password: &String,
    host: &String,
    port: &String,
    schema: &String,
) -> anyhow::Result<Conn> {
    let url = format!("mysql://{user}:{password}@{host}:{port}/{schema}");
    let opt = Opts::from_url(&url).unwrap();
    let builder = OptsBuilder::from_opts(opt);
    let manager = MysqlConnectionManager::new(builder);
    manager.connect().map_err(|e| anyhow!(e))
}

fn get_tables(conn: &mut Conn, schema: &String) -> anyhow::Result<Vec<String>> {
    conn.query(format!(
        "select table_name from information_schema.tables where table_schema = '{schema}'"
    ))
    .map::<Vec<String>, _>(|result| result.map(|x| x.unwrap()).map(from_row).collect())
    .map_err(|e| anyhow!(e))
}

fn get_columns(
    conn: &mut Conn,
    schema: &String,
    table: &String,
) -> anyhow::Result<Vec<(String, String)>> {
    conn.query(
        format!("select column_name, data_type from information_schema.columns where table_schema = '{schema}' and table_name = '{table}' order by ordinal_position"))
        .map::<Vec<(String, String)>, _>(|result| {
            result
                .map(|x| x.unwrap())
                .map(from_row)
                .collect()
        }).map_err(|e|anyhow!(e))
}

fn get_rows(
    conn: &mut Conn,
    table: &String,
    column_count: usize,
) -> anyhow::Result<Vec<Vec<String>>> {
    conn.query(format!("select * from {table}"))
        .map::<Vec<Vec<String>>, _>(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| (0..column_count).map(|i| row.get(i).unwrap()).collect_vec())
                .collect()
        })
        .map_err(|e| anyhow!(e))
}
