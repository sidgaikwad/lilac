use crate::serialize_secret;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, getset::Getters, getset::Setters)]
#[getset(get = "pub", set = "pub")]
pub struct SnowflakeConnector {
    username: String,
    #[serde(serialize_with = "serialize_secret")]
    password: SecretString,
    account: String,
    warehouse: Option<String>,
    database: Option<String>,
    schema: Option<String>,
    role: Option<String>,
}

impl SnowflakeConnector {
    pub fn new(
        username: String,
        password: SecretString,
        account: String,
        warehouse: Option<String>,
        database: Option<String>,
        schema: Option<String>,
        role: Option<String>,
    ) -> Self {
        Self {
            username,
            password,
            account,
            warehouse,
            database,
            schema,
            role,
        }
    }
}
