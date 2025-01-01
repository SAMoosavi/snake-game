use super::{direction::Direction, point::Point};

use rand::Rng;

const BLOCK_CELL: i8 = -1;
const FOOD_CELL: i8 = -2;
const EMPTY_CELL: i8 = 0;

pub struct Board<const N: usize> {
    game_table: [[i8; N]; N],
    length: u8,
    score: usize,
    direction: Direction,
}

impl<const N: usize> Board<N> {
    pub fn new(length: u8) -> Result<Self, String> {
        if (length as usize) + 2 >= N {
            return Err(format!(
                "the table size must be grater than of start snake length + 2: {}",
                length + 2
            ));
        }

        Ok(Self {
            game_table: Self::create_table(length),
            length,
            score: 0,
            direction: Direction::Right,
        })
    }

    fn create_table(length: u8) -> [[i8; N]; N] {
        let mut game_table = [[EMPTY_CELL; N]; N];
        game_table[0].fill(BLOCK_CELL);
        game_table[N - 1].fill(BLOCK_CELL);
        for row in &mut game_table[1..N - 1] {
            row[0] = BLOCK_CELL;
            row[N - 1] = BLOCK_CELL;
        }

        let half = (N - 1) / 2;
        let offset = (length / 2) as isize;

        if length % 2 != 0 {
            for (j, i) in (-offset..=offset).rev().enumerate() {
                game_table[half][((half as isize) + i) as usize] = (j + 1) as i8;
            }
        } else {
            for (j, i) in (-offset..offset).rev().enumerate() {
                game_table[half][((half as isize) + i) as usize] = (j + 1) as i8;
            }
        }

        let Point { x, y } = Self::find_lunch_point(&game_table);
        game_table[x][y] = FOOD_CELL;

        game_table
    }

    pub fn get_table(&self) -> Vec<Vec<String>> {
        self.game_table
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&cell| match cell {
                        EMPTY_CELL => " ".to_string(),
                        FOOD_CELL => "O".to_string(),
                        BLOCK_CELL => "\u{25A0}".to_string(),
                        _ => ".".to_string(),
                    })
                    .collect()
            })
            .collect()
    }

    pub fn get_score(&self) -> &usize {
        &self.score
    }

    pub fn walk(&mut self) -> bool {
        let mut tail_point = Point { x: 0, y: 0 };
        let mut head_point = Point { x: 0, y: 0 };
        for (x, row) in &mut self.game_table.iter_mut().enumerate() {
            if x != 1 || x != N - 1 {
                for (y, cell) in row.iter_mut().enumerate() {
                    if *cell != EMPTY_CELL && *cell != BLOCK_CELL && *cell != FOOD_CELL {
                        if *cell == self.length as i8 {
                            *cell = EMPTY_CELL;
                            tail_point = Point { x, y };
                        } else {
                            *cell += 1;
                            if *cell == 2 {
                                head_point = match &self.direction {
                                    Direction::Up => Point { x: x - 1, y },
                                    Direction::Down => Point { x: x + 1, y },
                                    Direction::Left => Point { x, y: y - 1 },
                                    Direction::Right => Point { x, y: y + 1 },
                                };
                            }
                        }
                    }
                }
            }
        }

        match self.game_table[head_point.x][head_point.y] {
            FOOD_CELL => {
                self.length += 1;
                self.game_table[tail_point.x][tail_point.y] = self.length as i8;
                self.game_table[head_point.x][head_point.y] = 1;
                self.score += 1;

                let lunch_point = Self::find_lunch_point(&self.game_table);
                self.game_table[lunch_point.x][lunch_point.y] = FOOD_CELL;
                true
            }
            EMPTY_CELL => {
                self.game_table[head_point.x][head_point.y] = 1;

                true
            }
            _ => false,
        }
    }

    pub fn rotation(&mut self, direction: Direction) {
        if !self.direction.is_opposite(&direction) {
            self.direction = direction;
        }
    }

    fn find_lunch_point(game_table: &[[i8; N]; N]) -> Point {
        let mut rng = rand::thread_rng();
        let mut x;
        let mut y;
        loop {
            x = rng.gen_range(1..N - 1);
            y = rng.gen_range(1..N - 1);

            if game_table[x][y] == 0 {
                return Point { x, y };
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
