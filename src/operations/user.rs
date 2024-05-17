use crate::{
    client::HashGateClient,
    error::HashGateError,
    types::{requests, responses},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub is_verified: bool,
    pub creation_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl HashGateClient {
    pub async fn authenticate_user(
        &self,
        username: String,
        password: String,
    ) -> Result<String, HashGateError> {
        let endpoint = "http://localhost:8083/api/user/sign-in";

        let payload = requests::UserAuthReq { username, password };

        let resp = self.post(endpoint, &payload).await?;

        if resp.status().is_success() {
            let resp_body = resp.json::<responses::AuthResponse>().await?;
            if let Some(token) = resp_body.token {
                Ok(token)
            } else {
                Err(HashGateError::FailedSignIn)
            }
        } else {
            Err(HashGateError::FailedSignIn)
        }
    }

    pub async fn register_user(
        &self,
        username: String,
        email: Option<String>,
        password: String,
    ) -> Result<String, HashGateError> {
        let endpoint = "http://localhost:8083/api/user/create";

        let payload = requests::UserRegistrationReq {
            username,
            email,
            password,
        };

        let resp = self.post(endpoint, &payload).await?;

        if resp.status().is_success() {
            let resp_body = resp.json::<responses::AuthResponse>().await?;
            if let Some(token) = resp_body.token {
                Ok(token)
            } else {
                Err(HashGateError::FailedSignIn)
            }
        } else {
            Err(HashGateError::FailedSignIn)
        }
    }

    /// Get a user from their user id
    pub async fn get_user_by_id(&self, user_id: &Uuid) -> Result<User, HashGateError> {
        let endpoint = "http://localhost:8083/api/user/get";

        let payload = requests::GetUserByIdReq {
            user_id: user_id.to_string(),
        };

        let resp = self.post(endpoint, &payload).await?;

        if resp.status().is_success() {
            let resp_body = resp.json::<responses::GetUserResp>().await?;
            if let Some(user) = resp_body.user {
                Ok(user)
            } else {
                Err(HashGateError::UserNotFound)
            }
        } else {
            Err(HashGateError::ServerError)
        }
    }

    /// Get a user from a auth token
    pub async fn get_user_by_token(&self, token: &str) -> Result<User, HashGateError> {
        let endpoint = "http://localhost:8083/api/user/get-by-token";

        let payload = requests::GetUserByTokenReq {
            token: token.to_string(),
        };

        let resp = self.post(endpoint, &payload).await?;

        if resp.status().is_success() {
            let resp_body = resp.json::<responses::GetUserResp>().await?;
            if let Some(user) = resp_body.user {
                Ok(user)
            } else {
                Err(HashGateError::UserNotFound)
            }
        } else {
            Err(HashGateError::ServerError)
        }
    }
}
