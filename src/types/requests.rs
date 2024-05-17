use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// HashGate Client Authentication Request
pub struct ClientAuthReq {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// HashGate User Registration Request
pub struct UserRegistrationReq {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// HashGate User Authentication Request
pub struct UserAuthReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// HashGate Get User By Id Request
pub struct GetUserByIdReq {
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// HashGate Get User By Token Request
pub struct GetUserByTokenReq {
    pub token: String,
}
