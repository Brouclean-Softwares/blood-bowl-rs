use serde::{Deserialize, Serialize};
use crate::translation::{language_from, translate_to, LOCALES};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    TeamCreationError(String),
}

impl Error {
    fn translate_to(&self, lang_id: &str) -> String {
        let message = match self {
            Error::TeamCreationError(message) => message,
        };

        translate_to(lang_id, message)
    }
}
