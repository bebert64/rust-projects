mod enums;
mod get_only_result;

pub use get_only_result::{GetOnlyResult, GetOnlyResultExtension};

pub use {diesel, don_error};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Db {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) ip: String,
    pub(crate) port: String,
    pub(crate) name: String,
}

impl Db {
    pub fn to_url(&self) -> String {
        format!(
            "postgres://{user}:{password}@[{ip}]:{port}/{name}",
            user = self.username,
            password = self.password,
            ip = self.ip,
            port = self.port,
            name = self.name,
        )
    }
}

#[macro_export]
macro_rules! db {
    () => {
        use $crate::{
            diesel::{pg::PgConnection, prelude::*},
            don_error::*,
        };

        pub(crate) fn db() -> DonResult<PgConnection> {
            Ok(PgConnection::establish(
                &crate::config::CONFIG.postgres.to_url(),
            )?)
        }
    };
}
