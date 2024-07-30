use crate::config::HashGateConfig;
use crate::error::HashGateError;
use crate::types::requests::ClientAuthReq;
use crate::types::responses::AuthResponse;
use reqwest::{header, Response};
use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
/// A client for interacting with your HashGate user pool.
pub struct HashGateClient {
    client_id: Uuid,
    client_secret: String,
    token: Option<String>,
    url_base: String,
    pub req_client: reqwest::Client,
}
impl HashGateClient {
    /// Try to create a `HashGateClient`
    /// TODO: Refactor into fluent builder as this grows in complexity
    pub async fn try_new(config: HashGateConfig) -> Result<HashGateClient, HashGateError> {
        let client_id = Uuid::from_str(&config.client_id)?;
        let mut client = HashGateClient {
            client_id,
            client_secret: config.client_secret,
            token: None,
            url_base: String::from("http://localhost:8083/api/"),
            req_client: reqwest::Client::new(),
        };

        client.try_authenticate().await?;

        Ok(client)
    }

    /// Try to authenticate the client with HashGate
    /// NOTE: Client tokens live for 4 hours
    async fn try_authenticate(&mut self) -> Result<(), HashGateError> {
        let client_auth_endpoint = format!("{}client/auth", self.url_base);

        let payload = ClientAuthReq {
            client_id: self.client_id.to_string(),
            client_secret: self.client_secret.clone(),
        };

        let resp = self
            .req_client
            .post(client_auth_endpoint)
            .json(&payload)
            .send()
            .await?;

        if resp.status().is_success() {
            let resp_body = resp.json::<AuthResponse>().await?;
            if let Some(token) = resp_body.token {
                self.token = Some(token);
                Ok(())
            } else {
                Err(HashGateError::FailedSignIn)
            }
        } else {
            Err(HashGateError::FailedSignIn)
        }
    }

    /// Send a post request from the client to HashGate
    pub async fn post<T: Serialize>(
        &self,
        endpoint: &str,
        payload: &T,
    ) -> Result<Response, HashGateError> {
        if let Some(token) = &self.token {
            let url = format!("{}{}", self.url_base, endpoint);

            match self
                .req_client
                .post(&url)
                .json(&payload)
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .send()
                .await
            {
                Ok(resp) => Ok(resp),
                Err(e) => {
                    eprintln!("{e:?}");
                    Err(HashGateError::Request(e))
                }
            }
        } else {
            Err(HashGateError::NoClientToken)
        }
    }

    /// Send a get request from the client to HashGate
    pub async fn get(&self, endpoint: &str) -> Result<Response, HashGateError> {
        if let Some(token) = &self.token {
            let url = format!("{}{}", self.url_base, endpoint);

            match self
                .req_client
                .get(&url)
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .send()
                .await
            {
                Ok(resp) => Ok(resp),
                Err(e) => {
                    eprintln!("{e:?}");
                    Err(HashGateError::Request(e))
                }
            }
        } else {
            Err(HashGateError::NoClientToken)
        }
    }
}
