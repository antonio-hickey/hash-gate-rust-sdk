#[derive(Debug)]
/// Hash Gate Errors
pub enum HashGateError {
    FailedSignIn,
    FailedConfig,
    NoClientToken,
    UserNotFound,
    ServerError,
    Uuid(uuid::Error),
    Request(reqwest::Error),
    CouldNotSetAttribute,
    UsernameTaken,
}
/// Implement display trait for `HashGateError`
impl std::fmt::Display for HashGateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FailedSignIn => write!(f, "Error: Sign In Attempt Failed"),
            Self::FailedConfig => write!(f, "Error: Environment Variables Not Set"),
            Self::NoClientToken => write!(f, "Error: HashGate Client Missing Auth Token"),
            Self::UserNotFound => write!(f, "Error: User Not Found"),
            Self::CouldNotSetAttribute => write!(f, "Error: Could Not Set User Attribute"),
            Self::UsernameTaken => write!(f, "Error: That Username Or Email Is Already In Use"),
            Self::ServerError => write!(
                f,
                "Error: HashGate Server Ran Into Issues With Your Request"
            ),
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
