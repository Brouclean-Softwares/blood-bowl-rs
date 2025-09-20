use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coach {
    pub id: Option<i32>,
    pub name: String,
}

impl Coach {
    pub fn from_name(name: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
        }
    }
}

impl Default for Coach {
    fn default() -> Self {
        Self {
            id: None,
            name: "".to_string(),
        }
    }
}

impl PartialEq for Coach {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(id), Some(other_id)) = (self.id, other.id) {
            id == other_id
        } else {
            self.name == other.name
        }
    }
}

impl Eq for Coach {}

impl Hash for Coach {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(id) = self.id {
            id.hash(state);
        } else {
            self.name.hash(state);
        }
    }
}
