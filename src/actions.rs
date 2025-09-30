use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Action {}

impl TypeName for Action {}
impl TranslatedName for Action {}
