use crate::advancements::{Advancement, AdvancementChoice};
use crate::dices::Dice;
use crate::errors::Error;
use crate::players::Player;
use rand::Rng;

pub fn advancement_choices_that_could_be_available_for_player(
    player: &Player,
) -> Vec<AdvancementChoice> {
    let mut choices_available = vec![];

    if let Some(position_definition) = player.position_definition() {
        for skill_category in position_definition.primary_skill_categories.iter() {
            choices_available.push(AdvancementChoice::RandomPrimarySkill(
                skill_category.clone(),
            ));
        }

        for choice in [
            AdvancementChoice::ChosenPrimarySkill,
            AdvancementChoice::ChosenSecondarySkill,
            AdvancementChoice::RandomCharacteristic,
        ] {
            choices_available.push(choice);
        }
    }

    choices_available
}

pub fn roll_advancements_to_choose_for_player(
    advancement_choice: &AdvancementChoice,
    player: &Player,
) -> Vec<Advancement> {
    match advancement_choice {
        AdvancementChoice::RandomSecondarySkill(_) => Vec::new(),

        AdvancementChoice::RandomPrimarySkill(skill_category) => {
            let potential_skills = skill_category.skills_available_for_player(player);
            let skill_position_1 = rand::rng().random_range(0..potential_skills.len());
            let skill_position_2 = rand::rng().random_range(0..potential_skills.len());

            vec![
                Advancement::RandomSkill(potential_skills[skill_position_1]),
                Advancement::RandomSkill(potential_skills[skill_position_2]),
            ]
        }

        AdvancementChoice::ChosenPrimarySkill => {
            Advancement::primary_skill_advancements_available_for_player(player)
        }

        AdvancementChoice::ChosenSecondarySkill => {
            Advancement::secondary_skill_advancements_available_for_player(player)
        }

        AdvancementChoice::RandomCharacteristic => {
            let dice_result = Dice::D8.roll();

            if dice_result == 1 {
                [
                    vec![Advancement::ArmourValue],
                    Advancement::primary_skill_advancements_available_for_player(player),
                    Advancement::secondary_skill_advancements_available_for_player(player),
                ]
                .concat()
            } else if dice_result == 2 {
                [
                    vec![Advancement::ArmourValue, Advancement::PassingAbility],
                    Advancement::primary_skill_advancements_available_for_player(player),
                    Advancement::secondary_skill_advancements_available_for_player(player),
                ]
                .concat()
            } else if dice_result == 3 || dice_result == 4 {
                [
                    vec![
                        Advancement::ArmourValue,
                        Advancement::MovementAllowance,
                        Advancement::PassingAbility,
                    ],
                    Advancement::primary_skill_advancements_available_for_player(player),
                    Advancement::secondary_skill_advancements_available_for_player(player),
                ]
                .concat()
            } else if dice_result == 5 {
                [
                    vec![Advancement::MovementAllowance, Advancement::PassingAbility],
                    Advancement::primary_skill_advancements_available_for_player(player),
                    Advancement::secondary_skill_advancements_available_for_player(player),
                ]
                .concat()
            } else if dice_result == 6 {
                [
                    vec![Advancement::Agility, Advancement::MovementAllowance],
                    Advancement::primary_skill_advancements_available_for_player(player),
                    Advancement::secondary_skill_advancements_available_for_player(player),
                ]
                .concat()
            } else if dice_result == 7 {
                [
                    vec![Advancement::Agility, Advancement::Strength],
                    Advancement::primary_skill_advancements_available_for_player(player),
                    Advancement::secondary_skill_advancements_available_for_player(player),
                ]
                .concat()
            } else if dice_result == 8 {
                [
                    vec![
                        Advancement::ArmourValue,
                        Advancement::MovementAllowance,
                        Advancement::PassingAbility,
                        Advancement::Agility,
                        Advancement::Strength,
                    ],
                    Advancement::primary_skill_advancements_available_for_player(player),
                    Advancement::secondary_skill_advancements_available_for_player(player),
                ]
                .concat()
            } else {
                Vec::new()
            }
        }
    }
}

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

pub fn added_value_for_player(advancement: &Advancement, player: &Player) -> Result<u32, Error> {
    match advancement {
        Advancement::ChosenSkill(skill) | Advancement::RandomSkill(skill) => {
            if skill.is_primary_for_player(player) {
                if skill.is_elite(&player.version) {
                    Ok(30000)
                } else {
                    Ok(20000)
                }
            } else if skill.is_secondary_for_player(player) {
                if skill.is_elite(&player.version) {
                    Ok(50000)
                } else {
                    Ok(40000)
                }
            } else {
                Err(Error::SkillNotAvailableForPlayer)
            }
        }

        Advancement::MovementAllowance => Ok(20000),
        Advancement::Strength => Ok(60000),
        Advancement::Agility => Ok(30000),
        Advancement::PassingAbility => Ok(20000),
        Advancement::ArmourValue => Ok(10000),
    }
}
