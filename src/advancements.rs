use crate::errors::Error;
use crate::players::Player;
use crate::skills::Skill;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Advancement {
    ChosenSkill(Skill),
    RandomSkill(Skill),
    MovementAllowance,
    Strength,
    Agility,
    PassingAbility,
    ArmourValue,
}

impl Advancement {
    pub const MAXIMUM: usize = 6;

    pub fn is_available_for_player(
        &self,
        player: &Player,
        advancement_number: usize,
    ) -> Result<bool, Error> {
        let player_can_have_advancement = match self {
            Advancement::ChosenSkill(skill) | Advancement::RandomSkill(skill) => {
                skill.is_primary_for_player(player)? || skill.is_secondary_for_player(player)?
            }
            Advancement::MovementAllowance => true,
            Advancement::Strength => true,
            Advancement::Agility => true,
            Advancement::PassingAbility => player.passing_ability()?.is_some(),
            Advancement::ArmourValue => true,
        };

        Ok(advancement_number <= Self::MAXIMUM
            && player_can_have_advancement
            && player.star_player_points
                >= self.star_player_points_cost_for_player(player, player.advancements.len() + 1)?
                    as i32)
    }

    pub fn added_value_for_player(&self, player: &Player) -> Result<u32, Error> {
        Ok(match self {
            Advancement::ChosenSkill(skill) => {
                if skill.is_primary_for_player(player)? {
                    20000
                } else if skill.is_secondary_for_player(player)? {
                    40000
                } else {
                    0
                }
            }

            Advancement::RandomSkill(skill) => {
                if skill.is_primary_for_player(player)? {
                    10000
                } else if skill.is_secondary_for_player(player)? {
                    20000
                } else {
                    0
                }
            }

            Advancement::MovementAllowance => 20000,
            Advancement::Strength => 80000,
            Advancement::Agility => 40000,
            Advancement::PassingAbility => 20000,
            Advancement::ArmourValue => 10000,
        })
    }

    pub fn star_player_points_cost_for_player(
        &self,
        player: &Player,
        advancement_number: usize,
    ) -> Result<u32, Error> {
        let mut cost = 0;

        match self {
            Advancement::ChosenSkill(skill) => {
                if skill.is_primary_for_player(player)? {
                    match advancement_number {
                        1 => cost = 6,
                        2 => cost = 8,
                        3 => cost = 12,
                        4 => cost = 16,
                        5 => cost = 20,
                        6 => cost = 30,
                        _ => {}
                    }
                } else if skill.is_secondary_for_player(player)? {
                    match advancement_number {
                        1 => cost = 12,
                        2 => cost = 14,
                        3 => cost = 18,
                        4 => cost = 22,
                        5 => cost = 26,
                        6 => cost = 40,
                        _ => {}
                    }
                }
            }

            Advancement::RandomSkill(skill) => {
                if skill.is_primary_for_player(player)? {
                    match advancement_number {
                        1 => cost = 3,
                        2 => cost = 4,
                        3 => cost = 6,
                        4 => cost = 8,
                        5 => cost = 10,
                        6 => cost = 15,
                        _ => {}
                    }
                } else if skill.is_secondary_for_player(player)? {
                    match advancement_number {
                        1 => cost = 6,
                        2 => cost = 8,
                        3 => cost = 12,
                        4 => cost = 16,
                        5 => cost = 20,
                        6 => cost = 30,
                        _ => {}
                    }
                }
            }

            Advancement::MovementAllowance
            | Advancement::Strength
            | Advancement::Agility
            | Advancement::PassingAbility
            | Advancement::ArmourValue => match advancement_number {
                1 => cost = 18,
                2 => cost = 20,
                3 => cost = 24,
                4 => cost = 28,
                5 => cost = 32,
                6 => cost = 50,
                _ => {}
            },
        }

        Ok(cost)
    }
}
