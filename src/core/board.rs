use super::point::Point;

type Wall = Point;
type Walls = Vec<Wall>;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board {
    table_size: u16,
    walls: Walls,
}

impl Board {
    pub fn new(table_size: u16, walls: Walls) -> Self {
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

        Self { table_size, walls }
    }

    pub fn default() -> Self {
        Self {
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

    pub fn get_table(&self) -> Vec<Vec<String>> {
        let len = (self.get_size() + 2) as usize;
        let mut result = vec![vec![" ".to_string(); len]; len];

        Self::put_walls(&mut result, &self.walls);
        Self::put_boarder(&mut result, len);

        result
    }
}

impl Board {
    fn put_walls(result: &mut [Vec<String>], walls: &Walls) {
        walls.iter().for_each(|p| {
            result[(p.get_x() + 1) as usize][(p.get_y() + 1) as usize] = "█".to_string()
        });
    }

    fn put_boarder(result: &mut [Vec<String>], len: usize) {
        let last_index = len - 1;
        result[0].fill("─".to_string());
        result[0][0] = "┌".to_string();
        result[0][last_index] = "┐".to_string();

        result[last_index].fill("─".to_string());
        result[last_index][0] = "└".to_string();
        result[last_index][last_index] = "┘".to_string();

        for row in &mut result[1..=(len - 2)] {
            row[0] = "│".to_string();
            row[last_index] = "│".to_string();
        }
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
        let board = Board::new(10, Vec::from([Wall::new(5, 6), Wall::new(3, 4)]));

        assert!(board.is_wall(&Wall::new(5, 6)));
        assert!(board.is_wall(&Wall::new(3, 4)));

        assert!(!board.is_wall(&Wall::new(5, 4)));
    }

    #[test]
    fn check_create() {
        let board = Board::new(4, Vec::from([Wall::new(-5, 7), Wall::new(3, 4)]));

        assert_eq!(board.walls, Vec::from([Wall::new(3, 3), Wall::new(3, 0)]));
    }
}
