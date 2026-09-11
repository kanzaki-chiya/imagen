use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    Config,
    Auth,
    RateLimit,
    Network,
    Server,
    InvalidRequest,
    InvalidResponse,
    Cancelled,
}

#[derive(Debug, Serialize)]
pub struct BackendError {
    pub kind: ErrorKind,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
}

impl BackendError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            status: None,
        }
    }

    pub fn with_status(kind: ErrorKind, message: impl Into<String>, status: u16) -> Self {
        Self {
            kind,
            message: message.into(),
            status: Some(status),
        }
    }

    pub fn cancelled() -> Self {
        Self::new(ErrorKind::Cancelled, "Generation canceled.")
    }
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for BackendError {}
