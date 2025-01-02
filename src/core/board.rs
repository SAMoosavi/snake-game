use super::{direction::Direction, point::Point};
use std::collections::LinkedList;

use rand::Rng;

type Table = LinkedList<Point>;

pub struct Board {
    game_table: Table,
    food: Point,
    table_size: u16,
    score: u16,
    direction: Direction,
}

impl Board {
    pub fn new(table_size: u16, length: u16) -> Result<Self, String> {
        if length >= table_size {
            return Err(format!(
                "the table size must be grater than of start snake length: {}",
                length
            ));
        }

        let game_table = Self::create_table(table_size, length);

        Ok(Self {
            food: Self::find_lunch_point(table_size, &game_table),
            game_table,
            table_size,
            score: 0,
            direction: Direction::Right,
        })
    }

    fn create_table(table_size: u16, length: u16) -> Table {
        let mut game_table = Table::new();

        let half = (table_size as i16 - 1) / 2;
        let offset = length as i16 / 2;

        if length % 2 != 0 {
            for i in -offset..=offset {
                game_table.push_back(Point::new(half, half + i));
            }
        } else {
            for i in -offset..offset {
                game_table.push_back(Point::new(half, half + i));
            }
        }

        game_table
    }

    pub fn get_table(&self) -> Vec<Vec<String>> {
        let mut result =
            vec![vec![".".to_string(); self.table_size as usize]; self.table_size as usize];

        self.game_table
            .iter()
            .for_each(|p| result[(p.get_x()) as usize][(p.get_y()) as usize] = " ".to_string());

        result[(self.food.get_x()) as usize][(self.food.get_y()) as usize] = "O".to_string();

        result
    }

    pub fn get_score(&self) -> &u16 {
        &self.score
    }

    pub fn walk(&mut self) -> bool {
        let head = self.game_table.front().unwrap();

        let new_head = match &self.direction {
            Direction::Up => Point::new(head.get_x() - 1, head.get_y()),
            Direction::Down => Point::new(head.get_x() + 1, head.get_y()),
            Direction::Left => Point::new(head.get_x(), head.get_y() - 1),
            Direction::Right => Point::new(head.get_x(), head.get_y() + 1),
        };

        if new_head == self.food {
            self.game_table.push_front(new_head);
                self.score += 1;
            self.food = Self::find_lunch_point(self.table_size, &self.game_table);
            true
        } else if new_head.get_x() < 0
            || new_head.get_y() < 0
            || new_head.get_x() > self.table_size as i16
            || new_head.get_y() > self.table_size as i16
        {
            self.game_table.pop_back();
            false
        } else {
            self.game_table.push_front(new_head);
            self.game_table.pop_back();
            true
        }
    }

    pub fn rotation(&mut self, direction: Direction) {
        if !self.direction.is_opposite(&direction) {
            self.direction = direction;
        }
    }

    fn find_lunch_point(table_size: u16, game_table: &Table) -> Point {
        let mut rng = rand::thread_rng();
        let mut point;
        loop {
            point = Point::new(
                rng.gen_range(0..table_size as i16),
                rng.gen_range(0..table_size as i16),
            );

            if !game_table.iter().any(|p| p == &point) {
                return point;
            }
        }
    }
}

#[cfg(test)]
mod test_board {
    mod test_board_helper {
        use crate::core::point::Point;

        use super::super::{BLOCK_CELL, EMPTY_CELL, FOOD_CELL};

        fn is_block_row(row: &[i8]) -> bool {
            row.iter().all(|&cell| cell == BLOCK_CELL)
        }

        pub fn check_food_count<const N: usize>(table: &[[i8; N]; N]) {
            let food_count = table
                .iter()
                .flat_map(|row| row.iter())
                .filter(|&&cell| cell == FOOD_CELL)
                .count();
            assert_eq!(food_count, 1, "the number of food is not correct")
        }

        pub fn check_wall<const N: usize>(table: &[[i8; N]; N]) {
            assert!(is_block_row(&table[0]));
            assert!(is_block_row(&table[N - 1]));

            for row in &table[1..N - 1] {
                assert_eq!(row[0], BLOCK_CELL);
                assert_eq!(row[N - 1], BLOCK_CELL);
            }
        }

        pub fn change_food<const N: usize>(table: &mut [[i8; N]; N], point: Point) {
            table.iter_mut().flatten().for_each(|cell| {
                if *cell == FOOD_CELL {
                    *cell = EMPTY_CELL;
                }
            });
            table[point.x][point.y] = FOOD_CELL;
        }
    }

    use crate::core::point::Point;

    use super::{Board, Direction, BLOCK_CELL, EMPTY_CELL, FOOD_CELL};

    #[test]
    fn check_create_size() {
        assert!(Board::<4>::new(3).is_err());
        assert!(Board::<5>::new(3).is_err());
        assert!(Board::<6>::new(3).is_ok());

        assert!(Board::<5>::new(4).is_err());
        assert!(Board::<6>::new(4).is_err());
        assert!(Board::<7>::new(4).is_ok());
    }

    #[test]
    fn check_create_table() {
        let odd_n_odd_len = Board::<7>::create_table(3);
        test_board_helper::check_wall::<7>(&odd_n_odd_len);
        test_board_helper::check_food_count::<7>(&odd_n_odd_len);
        let center = odd_n_odd_len[3];
        assert_eq!(center[4], 1);
        assert_eq!(center[3], 2);
        assert_eq!(center[2], 3);

        let even_n_odd_len = Board::<8>::create_table(3);
        test_board_helper::check_wall::<8>(&even_n_odd_len);
        test_board_helper::check_food_count::<8>(&even_n_odd_len);
        let center = even_n_odd_len[3];
        assert_eq!(center[4], 1);
        assert_eq!(center[3], 2);
        assert_eq!(center[2], 3);

        let odd_n_even_len = Board::<7>::create_table(4);
        test_board_helper::check_wall::<7>(&odd_n_even_len);
        test_board_helper::check_food_count::<7>(&odd_n_even_len);
        let center = odd_n_even_len[3];
        assert_eq!(center[4], 1);
        assert_eq!(center[3], 2);
        assert_eq!(center[2], 3);
        assert_eq!(center[1], 4);

        let even_n_even_len = Board::<8>::create_table(4);
        test_board_helper::check_wall::<8>(&even_n_even_len);
        test_board_helper::check_food_count::<8>(&even_n_even_len);
        let center = even_n_even_len[3];
        assert_eq!(center[4], 1);
        assert_eq!(center[3], 2);
        assert_eq!(center[2], 3);
        assert_eq!(center[1], 4);
    }

    #[test]
    fn walk() {
        let mut game = Board::<7>::new(3).unwrap();

        test_board_helper::change_food(&mut game.game_table, Point { x: 4, y: 5 });
        assert_eq!(
            game.game_table,
            [
                [
                    BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL,
                    BLOCK_CELL
                ],
                [BLOCK_CELL, EMPTY_CELL, 3, 2, 1, EMPTY_CELL, BLOCK_CELL],
                [
                    BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, FOOD_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL,
                    BLOCK_CELL
                ],
            ]
        );
        assert!(game.walk());

        assert_eq!(
            game.game_table,
            [
                [
                    BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL,
                    BLOCK_CELL
                ],
                [BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, 3, 2, 1, BLOCK_CELL],
                [
                    BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, FOOD_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL,
                    BLOCK_CELL
                ],
            ]
        );

        game.rotation(Direction::Down);

        assert_eq!(game.direction, Direction::Down);

        assert!(game.walk());

        test_board_helper::check_food_count(&game.game_table);

        test_board_helper::change_food(&mut game.game_table, Point { x: 1, y: 1 });

        assert_eq!(
            game.game_table,
            [
                [
                    BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, FOOD_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL,
                    BLOCK_CELL
                ],
                [BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, 4, 3, 2, BLOCK_CELL],
                [BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, 1, BLOCK_CELL],
                [
                    BLOCK_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL, EMPTY_CELL,
                    BLOCK_CELL
                ],
                [
                    BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL, BLOCK_CELL,
                    BLOCK_CELL
                ]
            ]
        );
    }
}
