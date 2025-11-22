use crate::advancements::AdvancementChoice;

pub fn star_player_points_cost(
    advancement_choice: &AdvancementChoice,
    advancement_number: usize,
) -> u32 {
    match (advancement_choice, advancement_number) {
        (AdvancementChoice::RandomPrimarySkill(_), 1) => 3,
        (AdvancementChoice::RandomPrimarySkill(_), 2) => 4,
        (AdvancementChoice::RandomPrimarySkill(_), 3) => 6,
        (AdvancementChoice::RandomPrimarySkill(_), 4) => 8,
        (AdvancementChoice::RandomPrimarySkill(_), 5) => 10,
        (AdvancementChoice::RandomPrimarySkill(_), 6) => 15,

        (AdvancementChoice::ChosenPrimarySkill | AdvancementChoice::RandomSecondarySkill(_), 1) => {
            6
        }
        (AdvancementChoice::ChosenPrimarySkill | AdvancementChoice::RandomSecondarySkill(_), 2) => {
            8
        }
        (AdvancementChoice::ChosenPrimarySkill | AdvancementChoice::RandomSecondarySkill(_), 3) => {
            12
        }
        (AdvancementChoice::ChosenPrimarySkill | AdvancementChoice::RandomSecondarySkill(_), 4) => {
            16
        }
        (AdvancementChoice::ChosenPrimarySkill | AdvancementChoice::RandomSecondarySkill(_), 5) => {
            20
        }
        (AdvancementChoice::ChosenPrimarySkill | AdvancementChoice::RandomSecondarySkill(_), 6) => {
            30
        }

        (AdvancementChoice::ChosenSecondarySkill, 1) => 12,
        (AdvancementChoice::ChosenSecondarySkill, 2) => 14,
        (AdvancementChoice::ChosenSecondarySkill, 3) => 18,
        (AdvancementChoice::ChosenSecondarySkill, 4) => 22,
        (AdvancementChoice::ChosenSecondarySkill, 5) => 26,
        (AdvancementChoice::ChosenSecondarySkill, 6) => 40,

        (AdvancementChoice::RandomCharacteristic, 1) => 18,
        (AdvancementChoice::RandomCharacteristic, 2) => 20,
        (AdvancementChoice::RandomCharacteristic, 3) => 24,
        (AdvancementChoice::RandomCharacteristic, 4) => 28,
        (AdvancementChoice::RandomCharacteristic, 5) => 32,
        (AdvancementChoice::RandomCharacteristic, 6) => 50,

        (_, _) => 0,
    }
}
