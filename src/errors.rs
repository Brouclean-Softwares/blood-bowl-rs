use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    TeamError(String),
    TeamCreationError(String),
    TeamUpdateError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.translate_to("en"))
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn translate_to(&self, lang_id: &str) -> String {
        let message = match self {
            Error::TeamError(message) => format!("Soucis avec l'équipe: {}", message),
            Error::TeamCreationError(message) => {
                format!("Impossible de créer l'équipe: {}", message)
            }
            Error::TeamUpdateError(message) => {
                format!("Impossible de modifier l'équipe: {}", message)
            }
        };

        crate::translation::translate_to(lang_id, &*message)
    }
}
