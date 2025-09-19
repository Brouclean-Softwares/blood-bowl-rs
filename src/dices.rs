use rand::Rng;

pub enum Dice {
    D3,
    D6,
    D6x2,
    D8,
    D16,
}

impl Dice {
    pub fn roll(self) -> u16 {
        match self {
            Dice::D3 => rand::rng().random_range(1..3),
            Dice::D6 => rand::rng().random_range(1..6),
            Dice::D6x2 => Dice::D6.roll() + Dice::D6.roll(),
            Dice::D8 => rand::rng().random_range(1..8),
            Dice::D16 => rand::rng().random_range(1..16),
        }
    }
}
