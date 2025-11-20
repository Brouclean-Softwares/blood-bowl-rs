use crate::injuries::Injury;

pub fn injuries_list() -> Vec<Injury> {
    vec![
        Injury::Stunned,
        Injury::KO,
        Injury::BadlyHurt,
        Injury::SeriouslyHurt,
        Injury::SeriousInjury,
        Injury::HeadInjury,
        Injury::SmashedKnee,
        Injury::BrokenArm,
        Injury::DislocatedHip,
        Injury::NeckInjury,
        Injury::Dead,
    ]
}

pub(crate) fn mapping_with_previous_version(injury_in_previous_version: &Injury) -> Injury {
    match injury_in_previous_version {
        Injury::DislocatedShoulder => Injury::NeckInjury,
        Injury::NeckInjury => Injury::DislocatedHip,
        injury_in_previous_version => injury_in_previous_version.clone(),
    }
}

pub(crate) fn reduces_movement_allowance() -> Injury {
    Injury::SmashedKnee
}

pub(crate) fn reduces_strength() -> Injury {
    Injury::NeckInjury
}

pub(crate) fn reduces_agility() -> Injury {
    Injury::DislocatedHip
}

pub(crate) fn reduces_passing_ability() -> Injury {
    Injury::BrokenArm
}

pub(crate) fn reduces_armour_value() -> Injury {
    Injury::HeadInjury
}
