use crate::{
    client::HashGateClient,
    error::HashGateError,
    types::{
        requests,
        responses::{self, CreateUserResp, SendVerificationEmailResp},
    },
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
    pub custom_attributes: serde_json::Value,
}
impl User {
    /// Try to get a `User` from a user id
    pub async fn try_from_id(id: Uuid, client: &mut HashGateClient) -> Result<User, HashGateError> {
        let endpoint = "user/get";

        let payload = requests::GetUserByIdReq {
            user_id: id.to_string(),
        };

        match client.post(endpoint, &payload).await {
            Ok(resp) => {
                let resp_body = resp.json::<responses::GetUserResp>().await?;
                if let Some(user) = resp_body.user {
                    Ok(user)
                } else {
                    Err(HashGateError::UserNotFound)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Try to get a `User` from a auth token
    pub async fn try_from_token(
        token: &str,
        client: &mut HashGateClient,
    ) -> Result<User, HashGateError> {
        let endpoint = "user/get-by-token";

        let payload = requests::GetUserByTokenReq {
            token: token.to_string(),
        };

        let resp = client.post(endpoint, &payload).await?;

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

    /// Set a custom attribute for a `User`
    pub async fn set_custom_attribute(
        &self,
        client: &mut HashGateClient,
        key: &str,
        value: &serde_json::Value,
    ) -> Result<(), HashGateError> {
        let endpoint = "user/set-attribute";

        let payload = requests::SetUserCustomAttributeReq {
            user_id: self.id,
            key: key.to_string(),
            value: value.clone(),
        };

        match client.post(endpoint, &payload).await {
            Ok(_) => Ok(()),
            Err(_) => Err(HashGateError::CouldNotSetAttribute),
        }
    }

    /// Get all custom attributes for a `User`
    pub async fn get_custom_attributes(
        &self,
        client: &mut HashGateClient,
    ) -> Result<serde_json::Value, HashGateError> {
        let endpoint = "user/get-attributes";

        let payload = requests::GetUserCustomAttributesReq { user_id: self.id };

        match client.post(endpoint, &payload).await {
            Ok(resp) => {
                let resp_body = resp
                    .json::<responses::GetUserCustomAttributesResp>()
                    .await?;

                Ok(resp_body.attributes)
            }
            Err(e) => Err(e),
        }
    }

    /// Get a specific custom attribute for a `User`
    pub async fn get_custom_attribute(
        &self,
        client: &mut HashGateClient,
        key: &str,
    ) -> Result<serde_json::Value, HashGateError> {
        let endpoint = "user/get-attribute";

        let payload = requests::GetUserCustomAttributeReq {
            user_id: self.id,
            key: key.to_string(),
        };

        match client.post(endpoint, &payload).await {
            Ok(resp) => {
                let resp_body = resp.json::<responses::GetUserCustomAttributeResp>().await?;

                Ok(resp_body.attribute)
            }
            Err(e) => Err(e),
        }
    }

    /// Verify a `User`s email address
    pub async fn verify_user_email(
        &mut self,
        client: &mut HashGateClient,
        code: &str,
    ) -> Result<bool, HashGateError> {
        let endpoint = "user/verify-email";

        let payload = requests::VerifyUserEmailReq {
            user_id: self.id,
            code: code.to_string(),
        };

        let resp = client.post(endpoint, &payload).await?;
        if resp.status().is_success() {
            let resp_body = resp.json::<responses::VerifyUserEmailResp>().await?;
            if resp_body.is_verified {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Err(HashGateError::ServerError)
        }
    }

    /// Verify a `User`s email address
    pub async fn send_verification_email(
        &mut self,
        client: &mut HashGateClient,
    ) -> Result<SendVerificationEmailResp, HashGateError> {
        let endpoint = "user/send-verification-email";

        let payload = requests::SendVerificationEmailReq { user_id: self.id };

        let resp = client.post(endpoint, &payload).await?;
        if resp.status().is_success() {
            let resp_body = resp.json::<responses::SendVerificationEmailResp>().await?;
            Ok(resp_body)
        } else {
            Err(HashGateError::ServerError)
        }
    }
}

impl HashGateClient {
    pub async fn authenticate_user(
        &mut self,
        username: String,
        password: String,
    ) -> Result<String, HashGateError> {
        let endpoint = "user/sign-in";

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
        &mut self,
        username: String,
        email: Option<String>,
        password: String,
    ) -> Result<CreateUserResp, HashGateError> {
        let endpoint = "user/create";

        let payload = requests::UserRegistrationReq {
            username,
            email,
            password,
            group_id: None,
        };

        let resp = self.post(endpoint, &payload).await?;
        if resp.status().is_success() {
            let resp_body = resp.json::<responses::CreateUserResp>().await?;
            Ok(resp_body)
        } else {
            Err(HashGateError::ServerError)
        }
    }

    pub async fn create_admin(
        &mut self,
        username: String,
        email: Option<String>,
        password: String,
    ) -> Result<String, HashGateError> {
        let endpoint = "user/create";

        let payload = requests::UserRegistrationReq {
            username,
            email,
            password,
            group_id: Some(1),
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
}
