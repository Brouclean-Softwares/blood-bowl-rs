use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum Characteristic {
    MovementAllowance,
    Strength,
    Agility,
    ArmourValue,
    PassingAbility,
}
