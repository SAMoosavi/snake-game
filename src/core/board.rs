use super::point::Point;

type Table = Vec<Point>;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    table_size: u16,
    walls: Table,
}

impl Board {
    pub fn new(table_size: u16, walls: Table) -> Self {
        let table_size_i16 = table_size as i16;
        let walls = walls
            .iter()
            .map(|p| {
                Point::new(
                    p.get_x().rem_euclid(table_size_i16),
                    p.get_y().rem_euclid(table_size_i16),
                )
            })
            .collect();

        Self { table_size, walls }
    }

    pub fn default() -> Self {
        Self {
            table_size: 10,
            walls: Vec::from([Point::new(5, 5)]),
        }
    }

    pub fn get_size(&self) -> u16 {
        self.table_size
    }

    pub fn is_wall(&self, point: &Point) -> bool {
        self.walls.contains(point)
    }

    pub fn add_point(&mut self, point: Point) {
        self.walls.push(point);
    }

    pub fn remove_point(&mut self, point: &Point) {
        self.walls.retain(|p| p != point);
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a Point;
    type IntoIter = std::slice::Iter<'a, Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.walls.iter()
    }
}

impl<'a> IntoIterator for &'a mut Board {
    type Item = &'a mut Point;
    type IntoIter = std::slice::IterMut<'a, Point>;

    fn into_iter(self) -> Self::IntoIter {
        self.walls.iter_mut()
    }
}

#[cfg(test)]
mod test_board {
    use super::{super::point::Point, Board};

    #[test]
    fn is_wall() {
        let board = Board::new(10, Vec::from([Point::new(5, 6), Point::new(3, 4)]));

        assert!(board.is_wall(&Point::new(5, 6)));
        assert!(board.is_wall(&Point::new(3, 4)));

        assert!(!board.is_wall(&Point::new(5, 4)));
    }

    #[test]
    fn check_create() {
        let board = Board::new(4, Vec::from([Point::new(-5, 7), Point::new(3, 4)]));

        assert_eq!(board.walls, Vec::from([Point::new(3, 3), Point::new(3, 0)]));
    }
}
