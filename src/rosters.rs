pub mod v5;

use std::collections::HashMap;
use crate::positions::{Position, PositionDefinition};
use crate::versions::Version;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum Roster {
    Amazon(Version),
    BlackOrc(Version),
    ChaosChosen(Version),
    ChaosDwarf(Version),
    ChaosRenegade(Version),
    DarkElf(Version),
    Dwarf(Version),
    ElvenUnion(Version),
    Gnome(Version),
    Goblin(Version),
    Halfling(Version),
    HighElf(Version),
    Human(Version),
    ImperialNobility(Version),
    Khorne(Version),
    Lizardmen(Version),
    NecromanticHorror(Version),
    Norse(Version),
    Nurgle(Version),
    Ogre(Version),
    OldWorldAliance(Version),
    Orc(Version),
    ShamblingUndead(Version),
    Skaven(Version),
    Snotling(Version),
    TombKings(Version),
    UnderworldDenizens(Version),
    Vampire(Version),
    WoodElf(Version),
}

pub enum SpecialRule {
    LustrianSuperleague,
}

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
pub enum Staff {
    Cheerleader,
    AssistantCoach,
    Apothecary,
    ReRoll,
}

pub struct RosterDefinition {
    pub tier: u8,
    pub staff_prices: HashMap<Staff, u32>,
    pub positions: Vec<Position>,
    pub maximum_big_men_quantity: u8,
    pub special_rule: SpecialRule,
}