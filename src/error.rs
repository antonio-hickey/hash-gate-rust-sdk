/// Hash Gate Error
#[derive(Debug)]
pub enum HashGateError {
    FailedSignIn,
    FailedConfig,
    NoClientToken,
    Uuid(uuid::Error),
    Request(reqwest::Error),
}
/// Implement display trait for `HashGateError`
impl std::fmt::Display for HashGateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FailedSignIn => write!(f, "Error: Sign In Attempt Failed"),
            Self::FailedConfig => write!(f, "Error: Environment Variables Not Set"),
            Self::NoClientToken => write!(f, "Error: HashGate Client Missing Auth Token"),
            Self::Uuid(e) => write!(f, "{e:?}"),
            Self::Request(e) => write!(f, "{e:?}"),
        }
    }
}
/// Implement error conversion (`uuid::Error` -> `HashGateError`)
impl From<uuid::Error> for HashGateError {
    fn from(err: uuid::Error) -> HashGateError {
        HashGateError::Uuid(err)
    }
}
/// Implement error conversion (`reqwest::Error` -> `HashGateError`)
impl From<reqwest::Error> for HashGateError {
    fn from(err: reqwest::Error) -> HashGateError {
        HashGateError::Request(err)
    }
}
