use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Characteristic {
    MovementAllowance,
    Strength,
    Agility,
    ArmourValue,
    PassingAbility,
}
