use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum Version {
    V4, // LRB version from 2000
    V5, // BB2020 Season 2
}
