use crate::client::HashGateClient;
use crate::error::HashGateError;
use crate::types::{requests, responses};

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
}
