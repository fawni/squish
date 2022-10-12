use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    pub(crate) username: String,
    pub(crate) client_id: String,
    pub(crate) access_token: String,
}
