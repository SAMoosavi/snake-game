pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn is_opposite(&self, other: &Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }
}

#[cfg(test)]
mod test_direction {
    use super::Direction;

    #[test]
    fn check_is_opposite() {
        let left = Direction::Left;
        let right = Direction::Right;
        let up = Direction::Up;
        let down = Direction::Down;

        assert!(left.is_opposite(&right));
        assert!(right.is_opposite(&left));
        assert!(down.is_opposite(&up));
        assert!(up.is_opposite(&down));

        assert!(!left.is_opposite(&left));
        assert!(!left.is_opposite(&up));
        assert!(!left.is_opposite(&down));

        assert!(!right.is_opposite(&right));
        assert!(!right.is_opposite(&up));
        assert!(!right.is_opposite(&down));

        assert!(!up.is_opposite(&up));
        assert!(!up.is_opposite(&left));
        assert!(!up.is_opposite(&right));

        assert!(!down.is_opposite(&down));
        assert!(!down.is_opposite(&left));
        assert!(!down.is_opposite(&right));
    }
}
