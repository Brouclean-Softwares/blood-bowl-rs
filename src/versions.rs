use crate::translation::TypeName;
use serde::{Deserialize, Serialize};

#[derive(sqlx::Type)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Version {
    V4, // LRB version from 2000
    V5, // BB2020 Season 2
}

impl TypeName for Version {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        let result = Version::V5.type_name();
        assert_eq!(result, "V5");
    }
}
