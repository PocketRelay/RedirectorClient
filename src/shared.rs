use std::fmt;

/// The default host to redirect to
pub const DEFAULT_HOST: &str = "127.0.0.1";
/// The default port for the Main server to redirect to
pub const DEFAULT_PORT: u16 = 14219;

pub struct SharedState {
    pub status: AppStatus,
    pub host: String,
    pub port: u16,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            status: AppStatus::Redirecting {
                host: String::from(DEFAULT_HOST),
                port: DEFAULT_PORT
            },
            host: String::from(DEFAULT_HOST),
            port: DEFAULT_PORT,
        }
    }
}

#[derive(Debug)]
pub enum AppStatus {
    Redirecting { host: String, port: u16 },
    Error(AppError),
}


impl fmt::Display for AppStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Redirecting { host, port } => f.write_str(&format!("Redirecting ({}:{})", host, port)),
            Self::Error(err) => f.write_str(&format!("Error ({})", err))
        }
    }
}


#[derive(Debug, Clone)]
pub enum AppError {
    InvalidPort,
    MissingAddress,
    MissingPort,
    FailedServerStart,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidPort => f.write_str("Invalid port"),
            AppError::MissingAddress => f.write_str("Missing address value"),
            AppError::MissingPort => f.write_str("Missing port value"),
            AppError::FailedServerStart => f.write_str("Failed to start server")
        }
    }
}
