use crate::operations::user::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub token: Option<String>,
    pub message: Option<String>,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResp {
    pub user: Option<User>,
    pub message: Option<String>,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUserResp {
    pub user: Option<User>,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreatePoolResp {
    pub user: Option<User>,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUserCustomAttributesResp {
    pub attributes: serde_json::Value,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetUserCustomAttributeResp {
    pub attribute: serde_json::Value,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct VerifyUserEmailResp {
    pub is_verified: bool,
    pub message: String,
    pub was_successful: bool,
}
