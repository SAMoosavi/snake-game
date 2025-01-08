use rand::Rng;
use std::collections::LinkedList;

use super::{point::Point, Board, Direction};

type Snake = LinkedList<Point>;
type Food = Point;

pub struct Game<'a> {
    board: &'a Board,
    snake: Snake,
    food: Food,
    score: u16,
    direction: Direction,
}

impl<'a> Game<'a> {
    pub fn new(board: &'a Board, length: u16) -> Self {
        let snake = Self::create_snake(board.get_size(), length);
        let food = Self::find_lunch_point(&snake, &board);

        Self {
            food,
            snake,
            score: 0,
            direction: Direction::Right,
            board,
        }
    }

    pub fn rotation(&mut self, direction: Direction) {
        if !self.direction.is_opposite(&direction) {
            self.direction = direction;
        }
    }

    pub fn get_score(&self) -> u16 {
        self.score
    }

    pub fn walk(&mut self) -> bool {
        let head = self.snake.front().unwrap();

        let mut new_head = head.get_neighbor(&self.direction);

        new_head = Point::new(
            (new_head.get_x() + self.board.get_size() as i16) % self.board.get_size() as i16,
            (new_head.get_y() + self.board.get_size() as i16) % self.board.get_size() as i16,
        );

        let collides_with_walls = self.board.is_wall(&new_head);
        let collides_with_body = Self::is_snake(&self.snake, &new_head);

        if collides_with_body || collides_with_walls {
            self.snake.pop_back();
            self.snake.pop_front();

            false
        } else if new_head == self.food {
            self.snake.push_front(new_head);
            self.score += 1;
            self.food = Self::find_lunch_point(&self.snake, &self.board);

            true
        } else {
            self.snake.push_front(new_head);
            self.snake.pop_back();

            true
        }
    }

    pub fn get_table(&self) -> Vec<Vec<String>> {
        let len = (self.board.get_size() + 2) as usize;
        let mut result = vec![vec![" ".to_string(); len]; len];

        Self::put_food(&mut result, &self.food);
        Self::put_snake(&mut result, &self.snake);
        Self::put_walls(&mut result, &self.board);
        Self::put_boarder(&mut result, len);

        result
    }
}

impl<'a> Game<'a> {
    fn put_snake(result: &mut [Vec<String>], snake: &Snake) {
        fn get_char(before: &Direction, after: &Direction) -> String {
            match (before, after) {
                (&Direction::Down, &Direction::Right) => "┌".to_string(),
                (&Direction::Right, &Direction::Down) => "┌".to_string(),
                (&Direction::Down, &Direction::Left) => "┐".to_string(),
                (&Direction::Left, &Direction::Down) => "┐".to_string(),
                (&Direction::Up, &Direction::Right) => "└".to_string(),
                (&Direction::Right, &Direction::Up) => "└".to_string(),
                (&Direction::Up, &Direction::Left) => "┘".to_string(),
                (&Direction::Left, &Direction::Up) => "┘".to_string(),
                (&Direction::None, &Direction::Left) => "─".to_string(),
                (&Direction::Left, &Direction::None) => "─".to_string(),
                (&Direction::None, &Direction::Right) => "─".to_string(),
                (&Direction::Right, &Direction::None) => "─".to_string(),
                (&Direction::Left, &Direction::Left) => "─".to_string(),
                (&Direction::Right, &Direction::Right) => "─".to_string(),
                (&Direction::Left, &Direction::Right) => "─".to_string(),
                (&Direction::Right, &Direction::Left) => "─".to_string(),
                (&Direction::Down, &Direction::Up) => "│".to_string(),
                (&Direction::Up, &Direction::Down) => "│".to_string(),
                (&Direction::None, &Direction::Down) => "│".to_string(),
                (&Direction::Down, &Direction::None) => "│".to_string(),
                (&Direction::None, &Direction::Up) => "│".to_string(),
                (&Direction::Up, &Direction::None) => "│".to_string(),
                (&Direction::Up, &Direction::Up) => "│".to_string(),
                (&Direction::Down, &Direction::Down) => "│".to_string(),
                _ => {
                    panic!("{:?} {:?}", before, after)
                }
            }
        }

        let mut iter = snake.iter();
        if let Some(first) = iter.next() {
            let mut prev_direction = Direction::None;
            let mut current = first;

            for next in iter {
                let char_to_set = get_char(&prev_direction, &current.direction_of_neighbor(next));
                result[(current.get_x() + 1) as usize][(current.get_y() + 1) as usize] =
                    char_to_set;

                prev_direction = next.direction_of_neighbor(current);
                current = next;
            }

            result[(current.get_x() + 1) as usize][(current.get_y() + 1) as usize] =
                get_char(&prev_direction, &Direction::None);
        }
    }

    fn put_walls(result: &mut [Vec<String>], board: &Board) {
        board.into_iter().for_each(|p| {
            result[(p.get_x() + 1) as usize][(p.get_y() + 1) as usize] = "█".to_string()
        });
    }

    fn put_food(result: &mut [Vec<String>], food: &Point) {
        result[(food.get_x() + 1) as usize][(food.get_y() + 1) as usize] = "●".to_string();
    }

    fn put_boarder(result: &mut [Vec<String>], len: usize) {
        result[0].fill("─".to_string());
        result[0][0] = "┌".to_string();
        result[0][(len - 1) as usize] = "┐".to_string();

        result[(len - 1) as usize].fill("─".to_string());
        result[(len - 1) as usize][0] = "└".to_string();
        result[(len - 1) as usize][(len - 1) as usize] = "┘".to_string();

        for row in &mut result[1..=(len - 2) as usize] {
            row[0] = "│".to_string();
            row[(len - 1) as usize] = "│".to_string();
        }
    }
}

impl<'a> Game<'a> {
    fn create_snake(table_size: u16, length: u16) -> Snake {
        let half = (table_size as i16 - 1) / 2;
        let offset = length as i16 / 2;

        let range = if length % 2 == 0 {
            -offset..=offset - 1
        } else {
            -offset..=offset
        };

        range.rev().map(|i| Point::new(half, half + i)).collect()
    }

    fn find_lunch_point(snake: &Snake, board: &Board) -> Food {
        let table_size = board.get_size();
        let mut rng = rand::thread_rng();
        let mut food = Self::get_head(snake);
        while Self::is_snake(snake, &food) || board.is_wall(&food) {
            food = Food::new(
                rng.gen_range(0..table_size as i16),
                rng.gen_range(0..table_size as i16),
            );
        }
        food
    }

    fn get_head(snake: &Snake) -> Point {
        (*snake.front().unwrap()).clone()
    }

    fn is_snake(snake: &Snake, point: &Point) -> bool {
        snake.contains(point)
    }
}

#[cfg(test)]
mod test_game {
    use std::collections::LinkedList;

    use crate::core::{point::Point, Board, Direction, Game};

    #[test]
    fn check_create_table() {
        let odd_n_odd_len = Game::create_snake(7, 3);
        assert_eq!(
            odd_n_odd_len,
            LinkedList::from([Point::new(3, 4), Point::new(3, 3), Point::new(3, 2)])
        );

        let even_n_odd_len = Game::create_snake(8, 3);
        assert_eq!(
            even_n_odd_len,
            LinkedList::from([Point::new(3, 4), Point::new(3, 3), Point::new(3, 2)])
        );

        let odd_n_even_len = Game::create_snake(7, 4);
        assert_eq!(
            odd_n_even_len,
            LinkedList::from([
                Point::new(3, 4),
                Point::new(3, 3),
                Point::new(3, 2),
                Point::new(3, 1)
            ])
        );

        let even_n_even_len = Game::create_snake(8, 4);
        assert_eq!(
            even_n_even_len,
            LinkedList::from([
                Point::new(3, 4),
                Point::new(3, 3),
                Point::new(3, 2),
                Point::new(3, 1)
            ])
        );
    }

    #[test]
    fn walk() {
        let board = Board::new(5, LinkedList::new());
        let mut game = Game::new(board, 3);
        game.food = Point::new(0, 0);

        assert_eq!(
            game.snake,
            LinkedList::from([Point::new(2, 3), Point::new(2, 2), Point::new(2, 1)])
        );
        assert!(game.walk());
        assert_eq!(
            game.snake,
            LinkedList::from([Point::new(2, 4), Point::new(2, 3), Point::new(2, 2)])
        );
        game.rotation(Direction::Down);

        assert!(game.walk());
        assert_eq!(
            game.snake,
            LinkedList::from([Point::new(3, 4), Point::new(2, 4), Point::new(2, 3)])
        );

        game.rotation(Direction::Left);

        assert!(game.walk());
        assert_eq!(
            game.snake,
            LinkedList::from([Point::new(3, 3), Point::new(3, 4), Point::new(2, 4)])
        );

        game.rotation(Direction::Up);

        assert!(game.walk());
        assert_eq!(
            game.snake,
            LinkedList::from([Point::new(2, 3), Point::new(3, 3), Point::new(3, 4)])
        );

        let board = Board::new(7, LinkedList::new());
        let mut game = Game::new(board, 5);
        game.food = Point::new(6, 6);

        game.snake = LinkedList::from([
            Point::new(1, 1),
            Point::new(2, 1),
            Point::new(2, 0),
            Point::new(1, 0),
            Point::new(0, 0),
        ]);
        game.direction = Direction::Left;

        assert!(!game.walk());
    }

    #[test]
    fn walk_system_test() {
        let board = Board::new(7, LinkedList::new());
        let mut game = Game::new(board, 3);

        assert_eq!(
            game.snake,
            LinkedList::from([Point::new(3, 4), Point::new(3, 3), Point::new(3, 2)])
        );

        game.food = Point::new(4, 5);

        assert!(game.walk());

        assert_eq!(
            game.snake,
            LinkedList::from([Point::new(3, 5), Point::new(3, 4), Point::new(3, 3)])
        );

        game.rotation(Direction::Down);

        assert_eq!(game.direction, Direction::Down);

        assert!(game.walk());
        assert_eq!(
            game.snake,
            LinkedList::from([
                Point::new(4, 5),
                Point::new(3, 5),
                Point::new(3, 4),
                Point::new(3, 3)
            ])
        );

        assert!(![
            Point::new(4, 5),
            Point::new(3, 5),
            Point::new(3, 4),
            Point::new(3, 3)
        ]
        .contains(&game.food));

        game.food = Point::new(0, 0);

        assert!(game.walk());
        assert_eq!(
            game.snake,
            LinkedList::from([
                Point::new(5, 5),
                Point::new(4, 5),
                Point::new(3, 5),
                Point::new(3, 4)
            ])
        );

        assert!(game.walk());
    }
}
