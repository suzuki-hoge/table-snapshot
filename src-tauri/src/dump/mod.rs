use crate::core::connector::Connector;
use crate::core::types::SnapshotId;
use crate::dump::mysql::dump as _mysql_dump;

mod mysql;
pub mod snapshot;

pub fn mysql_dump(snapshot_connector: &Connector, target_connector: &Connector) -> anyhow::Result<SnapshotId> {
    _mysql_dump(snapshot_connector, target_connector)
}
