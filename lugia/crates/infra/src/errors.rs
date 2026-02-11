use domain::errors::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error("Diesel error: {0}")]
    Diesel(#[from] diesel::result::Error),

    #[error("Pool error: {0}")]
    Pool(String),
}

impl From<InfraError> for DomainError {
    fn from(err: InfraError) -> Self {
        DomainError::Repository(err.to_string())
    }
}
