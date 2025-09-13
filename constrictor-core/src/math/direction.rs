#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// Get the [`Direction`] that is 90 degrees counter-clockwise from `self`.
    pub const fn ccw(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    /// Get the [`Direction`] that is 180 degrees from `self`.
    pub const fn flip(self) -> Self {
        self.ccw().ccw() // lol
    }

    /// Get the [`Direction`] that is 90 degrees clockwise from `self`.
    pub const fn cw(self) -> Self {
        self.flip().ccw() // double lol
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Direction;

    #[test]
    fn ccw_works() {
        assert_eq!(Direction::Up.ccw(), Direction::Left);
        assert_eq!(Direction::Left.ccw(), Direction::Down);
        assert_eq!(Direction::Down.ccw(), Direction::Right);
        assert_eq!(Direction::Right.ccw(), Direction::Up);
    }

    #[test]
    fn flip_works() {
        assert_eq!(Direction::Up.flip(), Direction::Down);
        assert_eq!(Direction::Left.flip(), Direction::Right);
        assert_eq!(Direction::Down.flip(), Direction::Up);
        assert_eq!(Direction::Right.flip(), Direction::Left);
    }

    #[test]
    fn cw_works() {
        assert_eq!(Direction::Up.cw(), Direction::Right);
        assert_eq!(Direction::Right.cw(), Direction::Down);
        assert_eq!(Direction::Down.cw(), Direction::Left);
        assert_eq!(Direction::Left.cw(), Direction::Up);
    }
}
