use crate::dices::Dice;
use crate::prayers::PrayerToNuffle;

pub fn roll_prayer() -> PrayerToNuffle {
    let dice_result = Dice::D16.roll();
    prayers_list()[dice_result - 1].clone()
}

pub fn prayers_list() -> Vec<PrayerToNuffle> {
    todo!()
}
