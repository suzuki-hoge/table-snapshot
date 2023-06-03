use table_snapshot::core::connector::Connector;
use table_snapshot::core::connector::Rdbms::Mysql;
use table_snapshot::diff::create_snapshot_diff;
use table_snapshot::dump::mysql_dump;

fn main() -> anyhow::Result<()> {
    let snapshot_connector = Connector::mysql("user", "password", "127.0.0.1", "19000", "table-snapshot");
    let target_connector = Connector::mysql("user", "password", "127.0.0.1", "19001", "testdata");

    match target_connector.rdbms {
        Mysql => mysql_dump(&snapshot_connector, &target_connector),
    }?;

    create_snapshot_diff(None, None);

    Ok(())
}
