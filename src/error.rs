use std::fmt::{Display, Formatter};

use thiserror::Error;
use twilight_http::{response::DeserializeBodyError, Error};
use twilight_validate::message::MessageValidationError;

use crate::operation::DiscordOperation;

#[derive(Debug, Error)]
pub enum InnerTwilightError {
    TwilightError(#[from] Error),
    DeserializeError(#[from] DeserializeBodyError),
    ValidationError(#[from] MessageValidationError),
}

impl Display for InnerTwilightError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TwilightError(e) => write!(f, "Twilight error ({})", e),
            Self::DeserializeError(e) => write!(f, "Deserialize error ({})", e),
            Self::ValidationError(e) => write!(f, "Validation error ({})", e),
        }
    }
}

#[derive(Debug, Error)]
pub struct DiscordError {
    pub inner: InnerTwilightError,
    pub operation: DiscordOperation,
}

impl DiscordError {
    pub(crate) fn new(inner: InnerTwilightError, operation: DiscordOperation) -> Self {
        Self { inner, operation }
    }
}

impl Display for DiscordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error while performing operation \"{}\":\n{}",
            self.operation, self.inner
        )
    }
}
