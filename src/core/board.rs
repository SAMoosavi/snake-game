use super::point::Point;

pub type Wall = Point;
type Walls = Vec<Wall>;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoardCell {
    Empty,
    Wall,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    name: String,
    table_size: u16,
    walls: Walls,
}

impl Board {
    pub fn new(name: String, table_size: u16, walls: Walls) -> Self {
        let table_size_i16 = table_size as i16;
        let walls = walls
            .iter()
            .map(|p| {
                Wall::new(
                    p.get_x().rem_euclid(table_size_i16),
                    p.get_y().rem_euclid(table_size_i16),
                )
            })
            .collect();

        Self {
            name,
            table_size,
            walls,
        }
    }

    pub fn copy_with_new_name(&self, name: String) -> Self {
        Self {
            name,
            table_size: self.table_size,
            walls: self.walls.clone(),
        }
    }

    pub fn default() -> Self {
        Self {
            name: "test board".to_string(),
            table_size: 10,
            walls: Vec::from([Wall::new(5, 5)]),
        }
    }

    pub fn get_size(&self) -> u16 {
        self.table_size
    }

    pub fn is_wall(&self, point: &Wall) -> bool {
        self.walls.contains(point)
    }

    pub fn add_wall(&mut self, point: Wall) -> Result<(), String> {
        let valid_range = 0..self.table_size as i16;

        if valid_range.contains(&point.get_x()) && valid_range.contains(&point.get_y()) {
            self.walls.push(point);
            Ok(())
        } else {
            Err("the point out of range".to_string())
        }
    }

    pub fn remove_wall(&mut self, point: &Wall) {
        self.walls.retain(|p| p != point);
    }

    pub fn get_board(&self) -> Vec<Vec<BoardCell>> {
        let len = self.get_size() as usize;
        let mut result = vec![vec![BoardCell::Empty; len]; len];

        self.walls.iter().for_each(|wall| {
            result[wall.get_x() as usize][wall.get_y() as usize] = BoardCell::Wall;
        });
        result
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_walls(&self) -> &[Wall] {
        &self.walls
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a Wall;
    type IntoIter = std::slice::Iter<'a, Wall>;

    fn into_iter(self) -> Self::IntoIter {
        self.walls.iter()
    }
}

impl<'a> IntoIterator for &'a mut Board {
    type Item = &'a mut Wall;
    type IntoIter = std::slice::IterMut<'a, Wall>;

    fn into_iter(self) -> Self::IntoIter {
        self.walls.iter_mut()
    }
}

#[cfg(test)]
mod test_board {
    use super::{Board, Wall};

    #[test]
    fn is_wall() {
        let board = Board::new(
            "test".to_string(),
            10,
            Vec::from([Wall::new(5, 6), Wall::new(3, 4)]),
        );

        assert!(board.is_wall(&Wall::new(5, 6)));
        assert!(board.is_wall(&Wall::new(3, 4)));

        assert!(!board.is_wall(&Wall::new(5, 4)));
    }

    #[test]
    fn check_create() {
        let board = Board::new(
            "test".to_string(),
            4,
            Vec::from([Wall::new(-5, 7), Wall::new(3, 4)]),
        );

        assert_eq!(board.walls, Vec::from([Wall::new(3, 3), Wall::new(3, 0)]));
    }
}
