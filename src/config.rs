use crate::error::HashGateError;
use std::env;

/// Configuration For Hash Gate Clients
pub struct HashGateConfig {
    pub client_id: String,
    pub client_secret: String,
}
impl HashGateConfig {
    /// Try to set up HashGateConfig automatically from your environment
    /// NOTE: `HASHGATE_CLIENT_ID` environment variable MUST be set.
    /// NOTE: `HASHGATE_CLIENT_SECRET` environment variable MUST be set.
    pub fn try_from_env() -> Result<HashGateConfig, HashGateError> {
        let client_id = env::var("HASHGATE_CLIENT_ID").map_err(|_| HashGateError::FailedConfig)?;
        let client_secret =
            env::var("HASHGATE_CLIENT_SECRET").map_err(|_| HashGateError::FailedConfig)?;

        Ok(HashGateConfig {
            client_id,
            client_secret,
        })
    }
}
