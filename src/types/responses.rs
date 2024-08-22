use crate::operations::user::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
pub struct UpdateUserPasswordResp {
    pub message: String,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct InitPasswordResetResp {
    pub verification_session_id: Uuid,
    pub verification_code: String,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct InitVerificationResp {
    pub verification_session_id: Uuid,
    pub verification_code: String,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResp {
    pub is_verified: bool,
    pub user_id: Option<Uuid>,
    pub message: String,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct VerifyPasswordResetResp {
    pub password_reset_session_id: Option<Uuid>,
    pub message: String,
    pub was_successful: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordResp {
    pub message: String,
    pub was_successful: bool,
}
