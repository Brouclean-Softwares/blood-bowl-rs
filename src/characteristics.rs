use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Characteristic {
    MovementAllowance,
    Strength,
    Agility,
    ArmourValue,
    PassingAbility,
}

impl Characteristic {
    pub fn maximum(&self) -> u8 {
        match self {
            Characteristic::MovementAllowance => 9,
            Characteristic::Strength => 8,
            Characteristic::Agility => 6,
            Characteristic::PassingAbility => 6,
            Characteristic::ArmourValue => 11,
        }
    }

    pub fn minimum(&self) -> u8 {
        match self {
            Characteristic::MovementAllowance => 1,
            Characteristic::Strength => 1,
            Characteristic::Agility => 1,
            Characteristic::PassingAbility => 1,
            Characteristic::ArmourValue => 3,
        }
    }

    pub fn value_in_boundaries(&self, value: isize) -> u8 {
        if value >= self.maximum() as isize {
            self.maximum()
        } else if value <= self.minimum() as isize {
            self.minimum()
        } else {
            value as u8
        }
    }
}
