use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Injury {
    Stunned,
    KO,
    BadlyHurt,
    SeriouslyHurt,
    SeriousInjury,
    HeadInjury,
    SmashedKnee,
    BrokenArm,
    NeckInjury,
    DislocatedShoulder,
    Dead,
}

impl TypeName for Injury {}
impl TranslatedName for Injury {}
