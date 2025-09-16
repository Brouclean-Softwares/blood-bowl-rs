use crate::errors::Error;
use crate::positions::Position;
use crate::rosters::Roster;
use crate::skills::Skill;
use crate::translation::TranslatedName;
use crate::versions::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: Option<i32>,
    pub version: Version,
    pub position: Position,
    pub name: String,
    pub star_player_points: i32,
}

impl Player {
    pub fn new(version: Version, position: Position) -> Self {
        Player {
            id: None,
            version,
            position,
            name: "".to_string(),
            star_player_points: 0,
        }
    }

    pub fn movement_allowance(&self, roster: &Roster) -> Result<u8, Error> {
        let movement_allowance = self
            .position
            .definition(self.version, *roster)?
            .movement_allowance();

        Ok(movement_allowance)
    }

    pub fn strength(&self, roster: &Roster) -> Result<u8, Error> {
        let strength = self.position.definition(self.version, *roster)?.strength();

        Ok(strength)
    }

    pub fn agility(&self, roster: &Roster) -> Result<u8, Error> {
        let agility = self.position.definition(self.version, *roster)?.agility();

        Ok(agility)
    }

    pub fn passing_ability(&self, roster: &Roster) -> Result<Option<u8>, Error> {
        let passing_ability = self
            .position
            .definition(self.version, *roster)?
            .passing_ability();

        Ok(passing_ability)
    }

    pub fn armour_value(&self, roster: &Roster) -> Result<u8, Error> {
        let armour_value = self
            .position
            .definition(self.version, *roster)?
            .armour_value();

        Ok(armour_value)
    }

    pub fn skills(&self, roster: &Roster) -> Result<Vec<Skill>, Error> {
        let skills = self.position.definition(self.version, *roster)?.skills;

        Ok(skills)
    }

    pub fn skills_names(&self, roster: &Roster, lang_id: &str) -> Result<String, Error> {
        let mut names: Vec<String> = Vec::with_capacity(self.skills(roster)?.len());

        for skill in self.skills(roster)?.clone() {
            names.push(skill.name(lang_id));
        }

        Ok(names.join(", "))
    }

    pub fn available(&self) -> bool {
        true
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
}
