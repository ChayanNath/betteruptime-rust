
use diesel::prelude::*;
use crate::config::Config;

pub struct Store {
    pub conn: PgConnection
}
impl Store {
    pub fn new() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let connection = PgConnection::establish(&config.database_url)?;
        Ok(Self {
            conn: connection
        })
    }
}