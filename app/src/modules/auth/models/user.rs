use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub user_name: String,
    pub n_id: String,
    pub password: String,
    pub email: String,
    pub phone: String,
    pub email_is_verified: String,
    pub phone_is_verified: String,
    pub verification_code: String,
    pub profile_id: String,
    pub group_id:String,
}
