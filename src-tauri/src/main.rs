use table_snapshot::database::mysql;

fn main() -> anyhow::Result<()> {
    let user = String::from("user");
    let password = String::from("password");
    let host = String::from("127.0.0.1");
    let port = String::from("19000");
    let schema = String::from("table-snapshot");

    mysql::dump(&user, &password, &host, &port, &schema)
}
