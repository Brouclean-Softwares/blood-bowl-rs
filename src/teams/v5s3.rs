use crate::errors::Error;
use crate::teams::Team;

pub(crate) fn expected_remaining_treasury_at_creation(team: &Team) -> Result<i32, Error> {
    super::v5::expected_remaining_treasury_at_creation(team)
}