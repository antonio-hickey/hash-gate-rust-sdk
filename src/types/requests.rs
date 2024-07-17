use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub group_id: Option<i32>,
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// HashGate Set User Custom Attribute Request
pub struct SetUserCustomAttributeReq {
    pub user_id: Uuid,
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// HashGate Get a `User`s Custom Attributes Request
pub struct GetUserCustomAttributesReq {
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// HashGate Get a `User`s Specific Custom Attribute Request
pub struct GetUserCustomAttributeReq {
    pub user_id: Uuid,
    pub key: String,
}
