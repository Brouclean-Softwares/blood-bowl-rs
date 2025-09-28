use crate::dices::Dice;
use crate::translation::{TranslatedName, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Weather {
    SwelteringHeat,
    VerySunny,
    PerfectConditions,
    PouringRain,
    Blizzard,
}

impl TypeName for Weather {}
impl TranslatedName for Weather {}

impl Weather {
    pub fn roll() -> Self {
        match Dice::D6x2.roll() {
            2 => Self::SwelteringHeat,
            3 => Self::VerySunny,
            11 => Self::PouringRain,
            12 => Self::Blizzard,
            _ => Self::PerfectConditions,
        }
    }

    pub fn options_list() -> Vec<Weather> {
        vec![
            Weather::SwelteringHeat,
            Weather::VerySunny,
            Weather::PerfectConditions,
            Weather::PouringRain,
            Weather::Blizzard,
        ]
    }
}
