use crate::actions::Success;

pub(crate) fn star_player_points_for_success(success: &Success) -> u32 {
    match success {
        Success::PassingCompletion => 1,
        Success::ThrowingCompletion => 1,
        Success::Deflection => 1,
        Success::Interception => 2,
        Success::Casualty => 2,
        Success::Touchdown => 3,
        Success::MostValuablePlayer => 4,
        Success::StarPlayerPoint => 1,
    }
}
