use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(type_name = "varchar")]
pub enum Advancement {
    ChosenPrimarySkill,
    RandomPrimarySkill,
    ChosenSecondarySkill,
    RandomSecondarySkill,
}
