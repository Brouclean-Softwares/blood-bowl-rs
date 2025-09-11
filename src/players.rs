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
}

impl Player {
    pub fn movement_allowance(&self, roster: &Roster) -> Result<u8, Error> {
        let movement_allowance = self
            .position
            .definition(Some(self.version), *roster)
            .ok_or(Error::TeamCreationError(String::from("PositionNotDefined")))?
            .movement_allowance();

        Ok(movement_allowance)
    }

    pub fn strength(&self, roster: &Roster) -> Result<u8, Error> {
        let strength = self
            .position
            .definition(Some(self.version), *roster)
            .ok_or(Error::TeamCreationError(String::from("PositionNotDefined")))?
            .strength();

        Ok(strength)
    }

    pub fn agility(&self, roster: &Roster) -> Result<u8, Error> {
        let agility = self
            .position
            .definition(Some(self.version), *roster)
            .ok_or(Error::TeamCreationError(String::from("PositionNotDefined")))?
            .agility();

        Ok(agility)
    }

    pub fn passing_ability(&self, roster: &Roster) -> Result<Option<u8>, Error> {
        let passing_ability = self
            .position
            .definition(Some(self.version), *roster)
            .ok_or(Error::TeamCreationError(String::from("PositionNotDefined")))?
            .passing_ability();

        Ok(passing_ability)
    }

    pub fn armour_value(&self, roster: &Roster) -> Result<u8, Error> {
        let armour_value = self
            .position
            .definition(Some(self.version), *roster)
            .ok_or(Error::TeamCreationError(String::from("PositionNotDefined")))?
            .armour_value();

        Ok(armour_value)
    }

    pub fn skills(&self, roster: &Roster) -> Result<Vec<Skill>, Error> {
        let skills = self
            .position
            .definition(Some(self.version), *roster)
            .ok_or(Error::TeamCreationError(String::from("PositionNotDefined")))?
            .skills;

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
        let position_price = self
            .position
            .definition(Some(self.version), *roster)
            .ok_or(Error::TeamCreationError(String::from("PositionNotDefined")))?
            .cost;

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
