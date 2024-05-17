use crate::operations::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub token: Option<String>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUserResp {
    pub user: Option<User>,
    pub was_successful: bool,
}
