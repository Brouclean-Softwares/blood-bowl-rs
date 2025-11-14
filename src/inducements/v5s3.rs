use crate::inducements::{Inducement, TreasuryAndPettyCash};
use crate::teams::Team;

pub(crate) fn inducements_buyable_for_team(
    team: &Team,
    money_left: &TreasuryAndPettyCash,
) -> Vec<Inducement> {
    todo!()
}

pub(crate) fn inducement_maximum_for_team(inducement: &Inducement, team: &Team) -> usize {
    todo!()
}

pub fn inducement_price_for_team(inducement: &Inducement, team: &Team) -> u32 {
    todo!()
}
