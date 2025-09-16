use crate::errors::Error;
use crate::teams::Team;

pub(crate) fn expected_remaining_treasury_at_creation(team: &Team) -> Result<i32, Error> {
    let roster_definition = team.roster.definition(team.version)?;

    Ok(Team::initial_treasury(&team.version) as i32
        - team.value()? as i32
        - (team.dedicated_fans as i32 - 1)
            * roster_definition.dedicated_fans_information.price as i32)
}
