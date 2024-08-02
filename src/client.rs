use crate::{
    config::HashGateConfig,
    error::HashGateError,
    types::{requests::ClientAuthReq, responses::AuthResponse},
};
use reqwest::{header, Response};
use serde::Serialize;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone)]
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

    /// Send a request to HashGate API
    /// NOTE: This is seperated kind of as middlewear to handle re-authing the client
    pub async fn send_request<F, T>(&mut self, request_fn: F) -> Result<Response, HashGateError>
    where
        F: Fn(&str) -> T,
        T: std::future::Future<Output = Result<Response, reqwest::Error>>,
    {
        match request_fn(self.token.as_mut().unwrap()).await {
            Ok(resp) => {
                // Check if the client gets a 401 unauthorized to try and re auth the client
                // this happens when auth token expires.
                if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
                    println!("80: {:?}", self.token);
                    self.try_authenticate().await?;
                    println!("82: {:?}", self.token);
                    let retry_response = request_fn(self.token.as_mut().unwrap()).await?;
                    Ok(retry_response)
                } else {
                    Ok(resp)
                }
            }
            Err(e) => Err(HashGateError::Request(e)),
        }
    }

    /// Send a post request from the client to HashGate
    pub async fn post<T: Serialize>(
        &mut self,
        endpoint: &str,
        payload: &T,
    ) -> Result<Response, HashGateError> {
        if self.token.is_some() {
            let url = format!("{}{}", self.url_base, endpoint);
            let resp = self
                .clone()
                .send_request(|token| {
                    self.req_client
                        .post(&url)
                        .json(&payload)
                        .header(header::AUTHORIZATION, format!("Bearer {token}"))
                        .send()
                })
                .await?;

            Ok(resp)
        } else {
            Err(HashGateError::NoClientToken)
        }
    }

    /// Send a get request from the client to HashGate
    pub async fn get(&mut self, endpoint: &str) -> Result<Response, HashGateError> {
        if self.token.is_some() {
            let url = format!("{}{}", self.url_base, endpoint);

            let resp = self
                .clone()
                .send_request(|token| {
                    self.req_client
                        .get(&url)
                        .header(header::AUTHORIZATION, format!("Bearer {token}"))
                        .send()
                })
                .await?;

            Ok(resp)
        } else {
            Err(HashGateError::NoClientToken)
        }
    }
}
