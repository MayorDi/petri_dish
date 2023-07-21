#[derive(derive_new::new, Debug, Clone, Copy)]
pub struct Direction(i8);

impl From<i8> for Direction {
    fn from(value: i8) -> Self {
        Direction(lim(value))
    }
}

impl std::ops::Add for Direction {
    type Output = Direction;

    fn add(self, rhs: Self) -> Self::Output {
        Direction::from(lim(self.0 + rhs.0))
    }
}

impl std::ops::Sub for Direction {
    type Output = Direction;

    fn sub(self, rhs: Self) -> Self::Output {
        Direction::from(lim(self.0 - rhs.0))
    }
}

fn lim(n: i8) -> i8 { ((n % 4) + 4) % 4 }