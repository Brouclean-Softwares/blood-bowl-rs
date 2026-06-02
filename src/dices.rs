use rand::RngExt;

pub enum Dice {
    D2,
    D3,
    D6,
    D6x2,
    D8,
    D16,
}

impl Dice {
    pub fn roll(self) -> usize {
        match self {
            Dice::D2 => rand::rng().random_range(1..=2),
            Dice::D3 => rand::rng().random_range(1..=3),
            Dice::D6 => rand::rng().random_range(1..=6),
            Dice::D6x2 => Dice::D6.roll() + Dice::D6.roll(),
            Dice::D8 => rand::rng().random_range(1..=8),
            Dice::D16 => rand::rng().random_range(1..=16),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2() {
        for _ in [0..1000] {
            let result = Dice::D2.roll();

            assert!(result > 0);
            assert!(result <= 2);
        }
    }

    #[test]
    fn d3() {
        for _ in [0..1000] {
            let result = Dice::D3.roll();

            assert!(result > 0);
            assert!(result <= 3);
        }
    }

    #[test]
    fn d6() {
        for _ in [0..1000] {
            let result = Dice::D6.roll();

            assert!(result > 0);
            assert!(result <= 6);
        }
    }

    #[test]
    fn d16() {
        for _ in [0..1000] {
            let result = Dice::D16.roll();

            assert!(result > 0);
            assert!(result <= 16);
        }
    }
}
