use super::Direction;

#[derive(PartialEq, Debug)]
pub struct Point {
    x: i16,
    y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> i16 {
        self.x
    }

    pub fn get_y(&self) -> i16 {
        self.y
    }

    pub fn get_neighbor(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Point::new(self.x - 1, self.y),
            Direction::Down => Point::new(self.x + 1, self.y),
            Direction::Left => Point::new(self.x, self.y - 1),
            Direction::Right => Point::new(self.x, self.y + 1),
        }
    }

    pub fn direction_of_neighbor(&self, other: &Self) -> Result<Direction, String> {
        match (other.x - self.x, other.y - self.y) {
            (1, 0) => Ok(Direction::Down),
            (-1, 0) => Ok(Direction::Up),
            (0, 1) => Ok(Direction::Right),
            (0, -1) => Ok(Direction::Left),
            _ => Err(format!(
                "The points {:?} and {:?} are not neighbors",
                self, other
            )),
        }
    }
}

#[cfg(test)]
mod test_point {
    use super::{super::Direction, Point};

    #[test]
    fn neighbor() {
        let point = Point::new(1, 1);

        let down = point.get_neighbor(&Direction::Down);
        let up = point.get_neighbor(&Direction::Up);
        let right = point.get_neighbor(&Direction::Right);
        let left = point.get_neighbor(&Direction::Left);

        assert_eq!(down, Point { x: 2, y: 1 });
        assert_eq!(up, Point { x: 0, y: 1 });
        assert_eq!(right, Point { x: 1, y: 2 });
        assert_eq!(left, Point { x: 1, y: 0 });

        assert_eq!(point.direction_of_neighbor(&down).unwrap(), Direction::Down);
        assert_eq!(point.direction_of_neighbor(&up).unwrap(), Direction::Up);
        assert_eq!(
            point.direction_of_neighbor(&right).unwrap(),
            Direction::Right
        );
        assert_eq!(point.direction_of_neighbor(&left).unwrap(), Direction::Left);

        assert!(point.direction_of_neighbor(&Point::new(1, 3)).is_err());
    }
}
