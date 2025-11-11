use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[sqlx(type_name = "varchar")]
pub enum Version {
    V4,   // LRB version from 2000
    V5,   // BB2020 Season 2
    V5S3, // BB2025 Season 3
}

impl Version {
    pub const LAST_VERSION: Self = Self::V5S3;
}

impl TypeName for Version {}
impl TranslatedName for Version {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        let result = Version::V5.type_name();
        assert_eq!(result, "V5");
    }
}
