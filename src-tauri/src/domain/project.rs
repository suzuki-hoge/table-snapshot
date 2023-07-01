pub enum Rdbms {
    Mysql,
}

pub struct Project {
    pub rdbms: Rdbms,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub schema: String,
}

impl Project {
    pub fn new<S: Into<String>>(rdbms: Rdbms, user: S, password: S, host: S, port: S, schema: S) -> Self {
        Self {
            rdbms,
            user: user.into(),
            password: password.into(),
            host: host.into(),
            port: port.into(),
            schema: schema.into(),
        }
    }
}
