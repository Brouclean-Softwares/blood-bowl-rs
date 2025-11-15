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
        Injury::NeckInjury,
        Injury::DislocatedShoulder,
        Injury::Dead,
    ]
}
