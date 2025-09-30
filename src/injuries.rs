use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(type_name = "varchar")]
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

impl Injury {
    pub fn list() -> Vec<Injury> {
        vec![
            Self::Stunned,
            Self::KO,
            Self::BadlyHurt,
            Self::SeriouslyHurt,
            Self::SeriousInjury,
            Self::HeadInjury,
            Self::SmashedKnee,
            Self::BrokenArm,
            Self::NeckInjury,
            Self::DislocatedShoulder,
            Self::Dead,
        ]
    }
}
