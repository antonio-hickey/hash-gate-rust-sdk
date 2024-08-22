use crate::{
    client::HashGateClient,
    error::HashGateError,
    types::{
        requests::{self, InitPasswordResetReq, ResetPasswordReq, VerifyPasswordResetReq},
        responses::{
            self, CreateUserResp, InitPasswordResetResp, InitVerificationResp, ResetPasswordResp,
            UpdateUserPasswordResp, VerificationResp, VerifyPasswordResetResp,
        },
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

    /// Initialize a verification session for a `User`
    ///
    /// This code can be sent to the user in email or phone sms/call.
    /// Use the verification session id, to then verify the code the user gives to you.
    pub async fn init_verification(
        &mut self,
        client: &mut HashGateClient,
    ) -> Result<InitVerificationResp, HashGateError> {
        let endpoint = "user/init-verification";

        let payload = requests::InitVerificationReq { user_id: self.id };

        let resp = client.post(endpoint, &payload).await?;
        if resp.status().is_success() {
            let resp_body = resp.json::<InitVerificationResp>().await?;
            Ok(resp_body)
        } else {
            Err(HashGateError::ServerError)
        }
    }

    /// Update a `User`s password
    pub async fn update_password(
        &mut self,
        new_password: String,
        client: &mut HashGateClient,
    ) -> Result<UpdateUserPasswordResp, HashGateError> {
        let endpoint = "user/update-password";

        let payload = requests::UpdateUserPasswordReq {
            user_id: self.id,
            new_password,
        };

        let resp = client.post(endpoint, &payload).await?;

        if resp.status().is_success() {
            let resp_body = resp.json::<UpdateUserPasswordResp>().await?;
            Ok(resp_body)
        } else {
            // The only reason this can fail is due to api down I think
            // TODO: look into above
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

    /// Initialize a password reset for a user
    ///
    /// NOTE: You need to verify that whoever is
    /// requesting the reset is in fact the user.
    /// Email, or phone is recommended, but you could
    /// self verify if you want however it's not recommended.
    pub async fn init_password_reset(
        &mut self,
        email: &str,
    ) -> Result<InitPasswordResetResp, HashGateError> {
        let endpoint = "user/init-password-reset";

        let email = email.to_owned();
        let payload = InitPasswordResetReq { email };

        let resp = self.post(endpoint, &payload).await?;

        if resp.status().is_success() {
            let resp_body = resp.json::<InitPasswordResetResp>().await?;
            Ok(resp_body)
        } else {
            // The only reason this can fail is due to api down I think
            // TODO: look into above
            Err(HashGateError::ServerError)
        }
    }

    /// Verify password reset
    ///
    /// NOTE: You must have a verification session id to call this
    /// you can obtain one from `HashGateClient::init_password_reset()`.
    pub async fn verify_password_reset(
        &mut self,
        verification_session_id: &Uuid,
        verification_code: &str,
    ) -> Result<VerifyPasswordResetResp, HashGateError> {
        let endpoint = "user/verify-password-reset";

        let verification_session_id = verification_session_id.to_owned();
        let verification_code = verification_code.to_owned();

        let payload = VerifyPasswordResetReq {
            verification_session_id,
            verification_code,
        };

        let resp = self.post(endpoint, &payload).await?;

        if resp.status().is_success() {
            let resp_body = resp.json::<VerifyPasswordResetResp>().await?;
            Ok(resp_body)
        } else {
            // The only reason this can fail is due to api down I think
            // TODO: look into above
            Err(HashGateError::ServerError)
        }
    }

    /// Reset `User` password
    ///
    /// NOTE: You need a password reset session id to call this, you can
    /// obtain one by first using `HashGateClient::init_password_reset()` and
    /// then verifying the reset with `HashGateClient::verify_password_reset()`.
    pub async fn reset_user_password(
        &mut self,
        password_reset_session_id: &Uuid,
        new_password: &str,
    ) -> Result<ResetPasswordResp, HashGateError> {
        let endpoint = "user/reset-password";

        let password_reset_session_id = password_reset_session_id.to_owned();
        let new_password = new_password.to_owned();

        let payload = ResetPasswordReq {
            password_reset_session_id,
            new_password,
        };

        let resp = self.post(endpoint, &payload).await?;

        if resp.status().is_success() {
            let resp_body = resp.json::<ResetPasswordResp>().await?;
            Ok(resp_body)
        } else {
            // The only reason this can fail is due to api down I think
            // TODO: look into above
            Err(HashGateError::ServerError)
        }
    }

    /// Complete a verification session for a `User`
    pub async fn verify(
        &mut self,
        verification_session_id: Uuid,
        code: &str,
    ) -> Result<VerificationResp, HashGateError> {
        let endpoint = "user/complete-verification";

        let payload = requests::VerifyReq {
            verification_session_id,
            verification_code: code.into(),
        };

        let resp = self.post(endpoint, &payload).await?;
        if resp.status().is_success() {
            let resp_body = resp.json::<VerificationResp>().await?;
            Ok(resp_body)
        } else {
            Err(HashGateError::ServerError)
        }
    }
}
