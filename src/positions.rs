pub mod v5;

use std::collections::HashMap;
use crate::characteristics::Characteristic;
use crate::rosters::Roster;
use crate::skills::{Skill, SkillCategory};

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum Position {
    EagleWarriorLinewoman(Roster),
    PiranhaWarriorBlitzer(Roster),
    PythonWarriorThrower(Roster),
    JaguarWarriorBlocker(Roster),
}

pub struct PositionDefinition {
    pub maximum_quantity: u8,
    pub cost: u32,
    pub characteristics: HashMap<Characteristic, u8>,
    pub skills: Vec<Skill>,
    pub primary_skill_categories: Vec<SkillCategory>,
    pub secondary_skill_categories: Vec<SkillCategory>,
    pub is_big_man: bool,
}