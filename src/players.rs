use crate::advancements::Advancement;
use crate::characteristics::Characteristic;
use crate::errors::Error;
use crate::injuries::Injury;
use crate::positions::Position;
use crate::rosters::Roster;
use crate::skills::Skill;
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStatistics {
    pub passing_completions: u32,
    pub throwing_completions: u32,
    pub deflections: u32,
    pub interceptions: u32,
    pub casualties: u32,
    pub touchdowns: u32,
    pub most_valuable_player: u32,
    pub star_player_points: u32,
}

impl PlayerStatistics {
    pub(crate) fn new() -> Self {
        Self {
            passing_completions: 0,
            throwing_completions: 0,
            deflections: 0,
            interceptions: 0,
            casualties: 0,
            touchdowns: 0,
            most_valuable_player: 0,
            star_player_points: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Player {
    pub id: i32,
    pub version: Version,
    pub position: Position,
    pub name: String,
    pub star_player_points: i32,
    pub is_journeyman: bool,
    pub is_star_player: bool,
    pub miss_next_game: bool,
    pub advancements: Vec<Advancement>,
    pub injuries: Vec<Injury>,
    pub niggling_injuries_number: usize,
}

impl Player {
    pub fn new(version: Version, position: Position) -> Self {
        Player {
            id: -1,
            version,
            position,
            name: "".to_string(),
            star_player_points: 0,
            is_journeyman: false,
            is_star_player: false,
            miss_next_game: false,
            advancements: vec![],
            injuries: vec![],
            niggling_injuries_number: 0,
        }
    }

    pub fn new_journeyman(id: i32, version: Version) -> Self {
        Player {
            id,
            version,
            position: Position::Journeyman,
            name: Position::Journeyman.type_name(),
            star_player_points: 0,
            is_journeyman: true,
            is_star_player: false,
            miss_next_game: false,
            advancements: vec![],
            injuries: vec![],
            niggling_injuries_number: 0,
        }
    }

    pub fn new_star_player(id: i32, version: Version, position: Position) -> Self {
        Player {
            id,
            version,
            position,
            name: position.type_name(),
            star_player_points: 0,
            is_journeyman: false,
            is_star_player: true,
            miss_next_game: false,
            advancements: vec![],
            injuries: vec![],
            niggling_injuries_number: 0,
        }
    }

    pub fn movement_allowance(&self, roster: &Roster) -> Result<u8, Error> {
        let mut value: isize = self.movement_allowance_from_position(roster)? as isize;

        value += self
            .advancements
            .iter()
            .filter(|&advancement| matches!(advancement, Advancement::MovementAllowance))
            .count() as isize;

        value -= self
            .injuries
            .iter()
            .filter(|&injury| matches!(injury, Injury::SmashedKnee))
            .count() as isize;

        Ok(Characteristic::MovementAllowance.value_in_boundaries(value))
    }

    pub fn movement_allowance_from_position(&self, roster: &Roster) -> Result<u8, Error> {
        let movement_allowance = self
            .position
            .definition(self.version, *roster)?
            .movement_allowance();

        Ok(movement_allowance)
    }

    pub fn strength(&self, roster: &Roster) -> Result<u8, Error> {
        let mut value: isize = self.strength_from_position(roster)? as isize;

        value += self
            .advancements
            .iter()
            .filter(|&advancement| matches!(advancement, Advancement::Strength))
            .count() as isize;

        value -= self
            .injuries
            .iter()
            .filter(|&injury| matches!(injury, Injury::DislocatedShoulder))
            .count() as isize;

        Ok(Characteristic::Strength.value_in_boundaries(value))
    }

    pub fn strength_from_position(&self, roster: &Roster) -> Result<u8, Error> {
        let strength = self.position.definition(self.version, *roster)?.strength();

        Ok(strength)
    }

    pub fn agility(&self, roster: &Roster) -> Result<u8, Error> {
        let mut value: isize = self.agility_from_position(roster)? as isize;

        value -= self
            .advancements
            .iter()
            .filter(|&advancement| matches!(advancement, Advancement::Agility))
            .count() as isize;

        value += self
            .injuries
            .iter()
            .filter(|&injury| matches!(injury, Injury::NeckInjury))
            .count() as isize;

        Ok(Characteristic::Agility.value_in_boundaries(value))
    }

    pub fn agility_from_position(&self, roster: &Roster) -> Result<u8, Error> {
        let agility = self.position.definition(self.version, *roster)?.agility();

        Ok(agility)
    }

    pub fn passing_ability(&self, roster: &Roster) -> Result<Option<u8>, Error> {
        if let Some(initial_value) = self.passing_ability_from_position(roster)? {
            let mut value: isize = initial_value as isize;

            value -= self
                .advancements
                .iter()
                .filter(|&advancement| matches!(advancement, Advancement::PassingAbility))
                .count() as isize;

            value += self
                .injuries
                .iter()
                .filter(|&injury| matches!(injury, Injury::BrokenArm))
                .count() as isize;

            Ok(Some(
                Characteristic::PassingAbility.value_in_boundaries(value),
            ))
        } else {
            Ok(None)
        }
    }

    pub fn passing_ability_from_position(&self, roster: &Roster) -> Result<Option<u8>, Error> {
        let passing_ability = self
            .position
            .definition(self.version, *roster)?
            .passing_ability();

        Ok(passing_ability)
    }

    pub fn armour_value(&self, roster: &Roster) -> Result<u8, Error> {
        let mut value: isize = self.armour_value_from_position(roster)? as isize;

        value += self
            .advancements
            .iter()
            .filter(|&advancement| matches!(advancement, Advancement::ArmourValue))
            .count() as isize;

        value -= self
            .injuries
            .iter()
            .filter(|&injury| matches!(injury, Injury::HeadInjury))
            .count() as isize;

        Ok(Characteristic::ArmourValue.value_in_boundaries(value))
    }

    pub fn armour_value_from_position(&self, roster: &Roster) -> Result<u8, Error> {
        let armour_value = self
            .position
            .definition(self.version, *roster)?
            .armour_value();

        Ok(armour_value)
    }

    pub fn added_skills(&self, roster: &Roster) -> Result<Vec<Skill>, Error> {
        let mut added_skills: Vec<Skill> = vec![];
        let initial_skills = self.skills_from_position(roster)?;

        for advancement in self.advancements.iter() {
            match advancement {
                Advancement::ChosenPrimarySkill(skill)
                | Advancement::RandomPrimarySkill(skill)
                | Advancement::ChosenSecondarySkill(skill)
                | Advancement::RandomSecondarySkill(skill) => {
                    if !initial_skills.contains(&skill) {
                        added_skills.push(skill.clone());
                    }
                }

                Advancement::MovementAllowance
                | Advancement::Strength
                | Advancement::Agility
                | Advancement::PassingAbility
                | Advancement::ArmourValue => {}
            }
        }

        Ok(added_skills)
    }

    pub fn skills(&self, roster: &Roster) -> Result<Vec<Skill>, Error> {
        Ok(vec![
            self.skills_from_position(roster)?,
            self.added_skills(roster)?,
        ]
        .concat())
    }

    pub fn skills_from_position(&self, roster: &Roster) -> Result<Vec<Skill>, Error> {
        let mut skills = self.position.definition(self.version, *roster)?.skills;

        if self.is_journeyman {
            skills.push(Skill::Loner(4));
        }

        Ok(skills)
    }

    pub fn skills_names(&self, roster: &Roster, lang_id: &str) -> Result<String, Error> {
        let mut names: Vec<String> = Vec::with_capacity(self.skills(roster)?.len());

        for skill in self.skills(roster)?.iter() {
            names.push(skill.name(lang_id));
        }

        Ok(names.join(", "))
    }

    pub fn receive_injury(&mut self, injury: Injury) {
        self.injuries.push(injury);
    }

    pub fn remove_injury(&mut self, injury_to_remove: Injury) {
        let index = self
            .injuries
            .iter()
            .position(|injury| injury_to_remove.eq(injury));

        if let Some(index) = index {
            self.injuries.remove(index);
        }
    }

    pub fn injuries_names(&self, lang_id: &str) -> Result<String, Error> {
        let mut names: Vec<String> = Vec::with_capacity(self.injuries.len());

        for injury in self.injuries.iter() {
            names.push(injury.name(lang_id));
        }

        Ok(names.join(", "))
    }

    pub fn dead(&self) -> bool {
        self.injuries.contains(&Injury::Dead)
    }

    pub fn available(&self) -> bool {
        !self.miss_next_game
    }

    pub fn value(&self, roster: &Roster) -> Result<u32, Error> {
        let position_price = self.position.definition(self.version, *roster)?.cost;

        Ok(position_price)
    }

    pub fn current_value(&self, roster: &Roster) -> Result<u32, Error> {
        if self.available() {
            Ok(self.value(roster)?)
        } else {
            Ok(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_wood_elf_wardancer_is_ok() {
        let player = Player::new(Version::V5, Position::Wardancer);
        assert_eq!(player.version, Version::V5);
        assert_eq!(player.position, Position::Wardancer);
        assert_eq!(player.movement_allowance(&Roster::WoodElf).unwrap(), 8);
        assert_eq!(player.strength(&Roster::WoodElf).unwrap(), 3);
        assert_eq!(player.agility(&Roster::WoodElf).unwrap(), 2);
        assert_eq!(
            player.passing_ability(&Roster::WoodElf).unwrap().unwrap(),
            4
        );
        assert_eq!(player.armour_value(&Roster::WoodElf).unwrap(), 8);
        assert_eq!(player.skills(&Roster::WoodElf).unwrap().len(), 3);
        assert_eq!(player.value(&Roster::WoodElf).unwrap(), 125000);
    }

    #[test]
    fn no_amazon_wardancer() {
        let player = Player::new(Version::V5, Position::Wardancer);
        assert!(player.value(&Roster::Amazon).is_err());
    }

    #[test]
    fn journey_man() {
        let player = Player::new_journeyman(-1, Version::V5);
        assert_eq!(player.id, -1);
        assert!(
            player
                .skills(&Roster::WoodElf)
                .unwrap()
                .contains(&Skill::Loner(4))
        );
    }
}
