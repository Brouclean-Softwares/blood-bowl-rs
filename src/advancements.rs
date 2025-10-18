use crate::skills::Skill;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Advancement {
    ChosenPrimarySkill(Skill),
    RandomPrimarySkill(Skill),
    ChosenSecondarySkill(Skill),
    RandomSecondarySkill(Skill),
    MovementAllowance,
    Strength,
    Agility,
    PassingAbility,
    ArmourValue,
}
