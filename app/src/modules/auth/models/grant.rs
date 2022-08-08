use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Grant {
    pub grant_name: String,
    pub description: String,
    pub role_id: String,
}
