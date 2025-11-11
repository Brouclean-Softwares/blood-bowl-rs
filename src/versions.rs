use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[sqlx(type_name = "varchar")]
pub enum Version {
    V1,   // 1987
    V2,   // 1991
    V3,   // 1994
    V4,   // LRB version from 2000
    V5,   // BB2020 Season 2
    V5S3, // BB2025 Season 3
}

impl Version {
    pub const LAST_VERSION: Self = *Self::LIST.last().expect("Versions are missing");

    pub const LIST: [Self; 6] = [Self::V1, Self::V2, Self::V3, Self::V4, Self::V5, Self::V5S3];

    pub fn next(&self) -> Option<Self> {
        Self::LIST
            .iter()
            .position(|version| self.eq(version))
            .and_then(|position| Self::LIST.get(position + 1).map(|version| version.clone()))
    }
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
