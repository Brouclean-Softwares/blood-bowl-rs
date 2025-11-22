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

        (AdvancementChoice::ChosenSecondarySkill, 1) => 10,
        (AdvancementChoice::ChosenSecondarySkill, 2) => 12,
        (AdvancementChoice::ChosenSecondarySkill, 3) => 16,
        (AdvancementChoice::ChosenSecondarySkill, 4) => 20,
        (AdvancementChoice::ChosenSecondarySkill, 5) => 24,
        (AdvancementChoice::ChosenSecondarySkill, 6) => 34,

        (AdvancementChoice::RandomCharacteristic, 1) => 14,
        (AdvancementChoice::RandomCharacteristic, 2) => 26,
        (AdvancementChoice::RandomCharacteristic, 3) => 20,
        (AdvancementChoice::RandomCharacteristic, 4) => 24,
        (AdvancementChoice::RandomCharacteristic, 5) => 28,
        (AdvancementChoice::RandomCharacteristic, 6) => 38,

        (_, _) => 0,
    }
}
