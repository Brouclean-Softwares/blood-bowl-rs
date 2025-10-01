use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Success {
    PassingCompletion,
    ThrowingCompletion,
    Deflection,
    Interception,
    Casualty,
    Touchdown,
    MostValuablePlayer,
    StarPlayerPoint,
}

impl TypeName for Success {}
impl TranslatedName for Success {}
