use super::Direction;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
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

    pub fn get_neighbor(&self, direction: &Direction, table_size: u16) -> Self {
        let table_size_i16 = table_size as i16;

        let (new_x, new_y) = match direction {
            Direction::Up => (self.x - 1, self.y),
            Direction::Down => (self.x + 1, self.y),
            Direction::Left => (self.x, self.y - 1),
            Direction::Right => (self.x, self.y + 1),
            Direction::None => (self.x, self.y),
        };

        Point::new(
            new_x.rem_euclid(table_size_i16),
            new_y.rem_euclid(table_size_i16),
        )
    }

    pub fn direction_of_neighbor(&self, other: &Self) -> Direction {
        match (other.x - self.x, other.y - self.y) {
            (1, 0) => Direction::Down,
            (x, 0) if x > 0 => Direction::Up,
            (-1, 0) => Direction::Up,
            (x, 0) if x < 0 => Direction::Down,
            (0, 1) => Direction::Right,
            (0, y) if y > 0 => Direction::Left,
            (0, -1) => Direction::Left,
            (0, y) if y < 0 => Direction::Right,
            _ => Direction::None,
        }
    }
}

#[cfg(test)]
mod test_point {
    use super::{super::Direction, Point};

    #[test]
    fn neighbor() {
        let table_size = 5;
        let point = Point::new(1, 1);

        let down = point.get_neighbor(&Direction::Down, table_size);
        let up = point.get_neighbor(&Direction::Up, table_size);
        let right = point.get_neighbor(&Direction::Right, table_size);
        let left = point.get_neighbor(&Direction::Left, table_size);

        assert_eq!(down, Point { x: 2, y: 1 });
        assert_eq!(up, Point { x: 0, y: 1 });
        assert_eq!(right, Point { x: 1, y: 2 });
        assert_eq!(left, Point { x: 1, y: 0 });

        assert_eq!(point.direction_of_neighbor(&down), Direction::Down);
        assert_eq!(point.direction_of_neighbor(&up), Direction::Up);
        assert_eq!(point.direction_of_neighbor(&right), Direction::Right);
        assert_eq!(point.direction_of_neighbor(&left), Direction::Left);

        assert_eq!(
            point.direction_of_neighbor(&Point::new(2, 3)),
            Direction::None
        );
    }
}
