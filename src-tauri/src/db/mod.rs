use anyhow::anyhow;
use mysql::{Conn, Opts, OptsBuilder};
use r2d2::ManageConnection;
use r2d2_mysql::MysqlConnectionManager;

pub mod diff;
pub mod project;
pub mod snapshot;

pub fn create_connection() -> anyhow::Result<Conn> {
    let url = "mysql://user:password@127.0.0.1:19000/table-snapshot".to_string();
    let opt = Opts::from_url(&url).unwrap();
    let builder = OptsBuilder::from_opts(opt);
    let manager = MysqlConnectionManager::new(builder);
    manager.connect().map_err(|e| anyhow!(e))
}
