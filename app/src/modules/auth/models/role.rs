use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Role {
    pub role_name: String,
    pub description: String,
    pub role_owner_id: String,
}
