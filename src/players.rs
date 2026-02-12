use crate::advancements::Advancement;
use crate::characteristics::Characteristic;
use crate::errors::Error;
use crate::injuries::Injury;
use crate::positions::{Keyword, Position, PositionDefinition};
use crate::rosters::{Roster, SpecialRule};
use crate::skills::Skill;
use crate::translation::{TranslatedName, TypeName};
use crate::versions::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PlayerType {
    FromRoster,
    Journeyman,
    Star,
    MegaStar,
    FamousCoachingStaff,
}

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
    pub fn new() -> Self {
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
    pub roster: Roster,
    pub name: String,
    pub star_player_points: i32,
    pub player_type: PlayerType,
    pub miss_next_game: bool,
    pub advancements: Vec<Advancement>,
    pub injuries: Vec<Injury>,
    pub hatred: Vec<Keyword>,
    pub is_captain: bool,
}

impl Player {
    pub fn new(version: Version, position: Position, roster: Roster) -> Self {
        Player {
            id: -1,
            version,
            position,
            roster,
            name: "".to_string(),
            star_player_points: 0,
            player_type: PlayerType::FromRoster,
            miss_next_game: false,
            advancements: Vec::new(),
            injuries: Vec::new(),
            hatred: Vec::new(),
            is_captain: false,
        }
    }

    pub fn new_journeyman(id: i32, version: Version, roster: Roster) -> Self {
        Player {
            id,
            version,
            position: Position::Journeyman,
            roster,
            name: Position::Journeyman.type_name(),
            star_player_points: 0,
            player_type: PlayerType::Journeyman,
            miss_next_game: false,
            advancements: Vec::new(),
            injuries: Vec::new(),
            hatred: Vec::new(),
            is_captain: false,
        }
    }

    pub fn name(&self, lang_id: &str) -> String {
        match self.player_type {
            PlayerType::FromRoster => self.name.clone(),
            PlayerType::Journeyman
            | PlayerType::Star
            | PlayerType::MegaStar
            | PlayerType::FamousCoachingStaff => self.position.name(lang_id),
        }
    }

    pub fn position_definition(&self) -> Option<PositionDefinition> {
        if matches!(self.position, Position::Journeyman) {
            self.roster
                .definition(self.version)?
                .journeyman_position
                .definition(self.version, self.roster)
        } else {
            self.position.definition(self.version, self.roster)
        }
    }

    pub fn position_for_next_version(&self) -> Option<Position> {
        match self.version.next()? {
            Version::V1 | Version::V2 | Version::V3 | Version::V4 | Version::V5 => None,
            Version::V5S3 => {
                crate::positions::v5s3::mapping_with_previous_version(&self.roster, &self.position)
            }
        }
    }

    pub fn movement_allowance(&self) -> Option<u8> {
        let mut value: isize = self.movement_allowance_from_position()? as isize;

        value += self
            .advancements
            .iter()
            .filter(|&advancement| matches!(advancement, Advancement::MovementAllowance))
            .count() as isize;

        value -= self
            .injuries
            .iter()
            .filter(|&injury| injury.eq(&Injury::reduces_movement_allowance(&self.version)))
            .count() as isize;

        Some(Characteristic::MovementAllowance.value_in_boundaries(value))
    }

    pub fn movement_allowance_from_position(&self) -> Option<u8> {
        self.position_definition()?
            .characteristic_value(Characteristic::MovementAllowance)
    }

    pub fn strength(&self) -> Option<u8> {
        let mut value: isize = self.strength_from_position()? as isize;

        value += self
            .advancements
            .iter()
            .filter(|&advancement| matches!(advancement, Advancement::Strength))
            .count() as isize;

        value -= self
            .injuries
            .iter()
            .filter(|&injury| injury.eq(&Injury::reduces_strength(&self.version)))
            .count() as isize;

        Some(Characteristic::Strength.value_in_boundaries(value))
    }

    pub fn strength_from_position(&self) -> Option<u8> {
        self.position_definition()?
            .characteristic_value(Characteristic::Strength)
    }

    pub fn agility(&self) -> Option<u8> {
        let mut value: isize = self.agility_from_position()? as isize;

        value -= self
            .advancements
            .iter()
            .filter(|&advancement| matches!(advancement, Advancement::Agility))
            .count() as isize;

        value += self
            .injuries
            .iter()
            .filter(|&injury| injury.eq(&Injury::reduces_agility(&self.version)))
            .count() as isize;

        Some(Characteristic::Agility.value_in_boundaries(value))
    }

    pub fn agility_from_position(&self) -> Option<u8> {
        self.position_definition()?
            .characteristic_value(Characteristic::Agility)
    }

    pub fn passing_ability(&self) -> Option<u8> {
        let mut value: isize = self.passing_ability_from_position()? as isize;

        value -= self
            .advancements
            .iter()
            .filter(|&advancement| matches!(advancement, Advancement::PassingAbility))
            .count() as isize;

        value += self
            .injuries
            .iter()
            .filter(|&injury| injury.eq(&Injury::reduces_passing_ability(&self.version)))
            .count() as isize;

        Some(Characteristic::PassingAbility.value_in_boundaries(value))
    }

    pub fn passing_ability_from_position(&self) -> Option<u8> {
        self.position_definition()?
            .characteristic_value(Characteristic::PassingAbility)
    }

    pub fn armour_value(&self) -> Option<u8> {
        let mut value: isize = self.armour_value_from_position()? as isize;

        value += self
            .advancements
            .iter()
            .filter(|&advancement| matches!(advancement, Advancement::ArmourValue))
            .count() as isize;

        value -= self
            .injuries
            .iter()
            .filter(|&injury| injury.eq(&Injury::reduces_armour_value(&self.version)))
            .count() as isize;

        Some(Characteristic::ArmourValue.value_in_boundaries(value))
    }

    pub fn armour_value_from_position(&self) -> Option<u8> {
        self.position_definition()?
            .characteristic_value(Characteristic::ArmourValue)
    }

    pub fn added_skills(&self) -> Vec<Skill> {
        let mut added_skills: Vec<Skill> = Vec::new();
        let initial_skills = self.skills_from_position();

        for advancement in self.advancements.iter() {
            match advancement {
                Advancement::ChosenSkill(skill) | Advancement::RandomSkill(skill) => {
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

        for keyword in &self.hatred {
            added_skills.push(Skill::Hatred(keyword.clone()));
        }

        added_skills
    }

    pub fn skills(&self) -> Vec<Skill> {
        vec![self.skills_from_position(), self.added_skills()].concat()
    }

    pub fn skills_from_position(&self) -> Vec<Skill> {
        let mut skills = Vec::new();

        if let Some(position_definition) = self.position_definition() {
            skills = position_definition.skills;

            if matches!(self.player_type, PlayerType::Journeyman) {
                skills.push(Skill::Loner(4));
            }
        }

        if self.is_captain {
            skills.push(Skill::Pro);
        }

        skills
    }

    pub fn skills_names(&self, lang_id: &str) -> String {
        self.skills()
            .iter()
            .map(|skill| skill.name(lang_id))
            .collect::<Vec<String>>()
            .join(", ")
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

    pub fn injuries_names(&self, lang_id: &str) -> String {
        self.injuries
            .iter()
            .map(|skill| skill.name(lang_id))
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn niggling_injuries_number(&self) -> usize {
        self.injuries
            .iter()
            .filter(|&injury| matches!(injury, Injury::SeriousInjury))
            .count()
    }

    pub fn available_hatred(&self) -> Vec<Keyword> {
        let mut available_keywords = Keyword::list(&self.version);

        available_keywords.retain(|keyword| {
            !self.hatred.contains(keyword)
                && keyword.ne(&Keyword::BigGuy)
                && keyword.ne(&Keyword::Blitzer)
                && keyword.ne(&Keyword::Blocker)
                && keyword.ne(&Keyword::Catcher)
                && keyword.ne(&Keyword::Lineman)
                && keyword.ne(&Keyword::Runner)
                && keyword.ne(&Keyword::Special)
                && keyword.ne(&Keyword::Thrower)
        });

        available_keywords
    }

    pub fn available_hatred_names(&self, lang_id: &str) -> Vec<Keyword> {
        let mut available_keywords = self.available_hatred();

        available_keywords.sort_by(|a, b| a.name(lang_id).cmp(&b.name(lang_id)));

        available_keywords
    }

    pub fn receive_hatred(&mut self, keyword: Keyword) {
        if self.available_hatred().contains(&keyword) {
            self.hatred.push(keyword);
        }
    }

    pub fn remove_hatred(&mut self, keyword_to_remove: Keyword) {
        let index = self
            .hatred
            .iter()
            .position(|keyword| keyword_to_remove.eq(keyword));

        if let Some(index) = index {
            self.hatred.remove(index);
        }
    }

    pub fn is_dead(&self) -> bool {
        self.injuries.contains(&Injury::Dead)
    }

    pub fn available(&self) -> bool {
        !self.miss_next_game
    }

    fn added_value_from_advancements(&self) -> Result<u32, Error> {
        let mut value = 0;

        for advancement in self.advancements.iter() {
            value += advancement.added_value_for_player(self)?;
        }

        Ok(value)
    }

    pub fn value(&self) -> Result<u32, Error> {
        let roster_definition = self
            .roster
            .definition(self.version)
            .ok_or(Error::RosterNotExist)?;

        let position_definition = self
            .position_definition()
            .ok_or(Error::PositionNotDefined)?;

        let position_value = if roster_definition
            .special_rules
            .contains(&SpecialRule::LowCostLinemen)
            && position_definition.maximum_quantity > 12
        {
            0
        } else {
            position_definition.cost
        };

        Ok(position_value + self.added_value_from_advancements()?)
    }

    pub fn current_value(&self) -> Result<u32, Error> {
        if self.available() {
            Ok(self.value()?)
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
        let player = Player::new(Version::V5, Position::Wardancer, Roster::WoodElf);
        assert_eq!(player.version, Version::V5);
        assert_eq!(player.position, Position::Wardancer);
        assert_eq!(player.movement_allowance().unwrap(), 8);
        assert_eq!(player.strength().unwrap(), 3);
        assert_eq!(player.agility().unwrap(), 2);
        assert_eq!(player.passing_ability().unwrap(), 4);
        assert_eq!(player.armour_value().unwrap(), 8);
        assert_eq!(player.skills().len(), 3);
        assert_eq!(player.value().unwrap(), 125000);
    }

    #[test]
    fn no_amazon_wardancer() {
        let player = Player::new(Version::V5, Position::Wardancer, Roster::Amazon);
        assert!(player.value().is_err());
    }

    #[test]
    fn journey_man() {
        let player = Player::new_journeyman(-1, Version::V5, Roster::WoodElf);
        assert_eq!(player.id, -1);
        assert!(player.skills().contains(&Skill::Loner(4)));
    }
}
