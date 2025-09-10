use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Error {
    TeamCreationError(String),
}

impl Error {
    pub fn translate_to(&self, lang_id: &str) -> String {
        let message = match self {
            Error::TeamCreationError(message) => message,
        };

        crate::translation::translate_to(lang_id, message)
    }
}
