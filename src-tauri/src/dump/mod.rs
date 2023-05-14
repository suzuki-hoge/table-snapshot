use crate::core::connector::Connector;
use crate::dump::mysql::dump as _mysql_dump;

mod mysql;

pub fn mysql_dump(connector: &Connector) -> anyhow::Result<()> {
    _mysql_dump(connector)
}
