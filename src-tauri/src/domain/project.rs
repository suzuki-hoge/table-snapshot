use uuid::Uuid;

pub type ProjectId = String;

pub fn create_project_id() -> ProjectId {
    Uuid::new_v4().to_string()
}

#[derive(Eq, PartialEq, Debug)]
pub enum Rdbms {
    Mysql,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Project {
    pub project_id: ProjectId,
    pub rdbms: Rdbms,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub schema: String,
}

impl Project {
    pub fn new<S: Into<String>>(project_id: &ProjectId, rdbms: Rdbms, user: S, password: S, host: S, port: S, schema: S) -> Self {
        Self {
            project_id: project_id.clone(),
            rdbms,
            user: user.into(),
            password: password.into(),
            host: host.into(),
            port: port.into(),
            schema: schema.into(),
        }
    }
}
