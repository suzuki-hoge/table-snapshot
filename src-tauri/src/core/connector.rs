use crate::core::connector::Rdbms::Mysql;

#[derive(Debug)]
pub enum Rdbms {
    Mysql,
}

#[derive(Debug)]
pub struct Connector {
    pub rdbms: Rdbms,
    user: String,
    password: String,
    host: String,
    port: String,
    pub schema: String,
}

impl Connector {
    pub fn mysql<S: Into<String>>(user: S, password: S, host: S, port: S, schema: S) -> Self {
        Self {
            rdbms: Mysql,
            user: user.into(),
            password: password.into(),
            host: host.into(),
            port: port.into(),
            schema: schema.into(),
        }
    }

    pub fn get_url(&self) -> String {
        match self.rdbms {
            Mysql => format!("mysql://{}:{}@{}:{}/{}", self.user, self.password, self.host, self.port, self.schema),
        }
    }
}
