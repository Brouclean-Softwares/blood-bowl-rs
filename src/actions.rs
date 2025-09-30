use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Action {
    PassingCompletion,
    ThrowingCompletion,
    Deflection,
    Interception,
    Casualty,
    Touchdown,
    MostValuablePlayer,
    StarPlayerPoint,
}

impl TypeName for Action {}
impl TranslatedName for Action {}
