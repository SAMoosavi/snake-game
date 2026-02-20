use rand::RngExt;
use std::collections::VecDeque;

use super::{Board, BoardCell, Direction, point::Point};

type Snake = VecDeque<Point>;
type Food = Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCell {
    Empty,
    Wall,
    SnakeBody,
    SnakeHead,
    Food,
}

pub struct Game<'a> {
    board: &'a Board,
    snake: Snake,
    food: Food,
    score: u16,
    score_per_food: u16,
    direction: Direction,
}

impl<'a> Game<'a> {
    pub fn new(board: &'a Board, length: u16, score_per_food: u16) -> Self {
        let snake = Self::create_snake(board.get_size(), length);
        let food = Self::find_lunch_point(&snake, board);

        Self {
            food,
            snake,
            score: 0,
            score_per_food: score_per_food.max(1),
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
        let new_head = head.get_neighbor(&self.direction, self.board.get_size());

        let collides_with_walls = self.board.is_wall(&new_head);
        let collides_with_body = Self::is_snake(&self.snake, &new_head);

        if collides_with_body || collides_with_walls {
            self.snake.pop_back();
            self.snake.pop_front();

            false
        } else if new_head == self.food {
            self.snake.push_front(new_head);
            self.score = self.score.saturating_add(self.score_per_food);
            self.food = Self::find_lunch_point(&self.snake, self.board);

            true
        } else {
            self.snake.push_front(new_head);
            self.snake.pop_back();

            true
        }
    }

    pub fn get_board(&self) -> Vec<Vec<GameCell>> {
        let mut result: Vec<Vec<GameCell>> = self
            .board
            .get_board()
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        BoardCell::Empty => GameCell::Empty,
                        BoardCell::Wall => GameCell::Wall,
                    })
                    .collect()
            })
            .collect();

        result[self.food.get_x() as usize][self.food.get_y() as usize] = GameCell::Food;

        if let Some(head) = self.snake.front() {
            result[head.get_x() as usize][head.get_y() as usize] = GameCell::SnakeHead;
        }

        self.snake.iter().skip(1).for_each(|segment| {
            result[segment.get_x() as usize][segment.get_y() as usize] = GameCell::SnakeBody;
        });

        result
    }
}

impl Game<'_> {
    fn create_snake(table_size: u16, length: u16) -> Snake {
        let half = (table_size as i16 - 1) / 2;
        let offset = length as i16 / 2;

        let range = if length.is_multiple_of(2) {
            -offset..=offset - 1
        } else {
            -offset..=offset
        };

        range.rev().map(|i| Point::new(half, half + i)).collect()
    }

    fn find_lunch_point(snake: &Snake, board: &Board) -> Food {
        let table_size = board.get_size();
        let mut rng = rand::rng();
        let mut food = Self::get_head(snake);
        while Self::is_snake(snake, &food) || board.is_wall(&food) {
            food = Food::new(
                rng.random_range(0..table_size as i16),
                rng.random_range(0..table_size as i16),
            );
        }
        food
    }

    fn get_head(snake: &Snake) -> Point {
        snake.front().unwrap().clone()
    }

    fn is_snake(snake: &Snake, point: &Point) -> bool {
        snake.contains(point)
    }
}

#[cfg(test)]
mod test_game {
    use std::collections::VecDeque;

    use crate::core::{Board, Direction, Game, point::Point};

    #[test]
    fn check_create_table() {
        let odd_n_odd_len = Game::create_snake(7, 3);
        assert_eq!(
            odd_n_odd_len,
            VecDeque::from([Point::new(3, 4), Point::new(3, 3), Point::new(3, 2)])
        );

        let even_n_odd_len = Game::create_snake(8, 3);
        assert_eq!(
            even_n_odd_len,
            VecDeque::from([Point::new(3, 4), Point::new(3, 3), Point::new(3, 2)])
        );

        let odd_n_even_len = Game::create_snake(7, 4);
        assert_eq!(
            odd_n_even_len,
            VecDeque::from([
                Point::new(3, 4),
                Point::new(3, 3),
                Point::new(3, 2),
                Point::new(3, 1)
            ])
        );

        let even_n_even_len = Game::create_snake(8, 4);
        assert_eq!(
            even_n_even_len,
            VecDeque::from([
                Point::new(3, 4),
                Point::new(3, 3),
                Point::new(3, 2),
                Point::new(3, 1)
            ])
        );
    }

    #[test]
    fn walk() {
        let board = Board::new("test".to_string(), 5, Vec::new());
        let mut game = Game::new(&board, 3, 1);
        game.food = Point::new(0, 0);

        assert_eq!(
            game.snake,
            VecDeque::from([Point::new(2, 3), Point::new(2, 2), Point::new(2, 1)])
        );
        assert!(game.walk());
        assert_eq!(
            game.snake,
            VecDeque::from([Point::new(2, 4), Point::new(2, 3), Point::new(2, 2)])
        );
        game.rotation(Direction::Down);

        assert!(game.walk());
        assert_eq!(
            game.snake,
            VecDeque::from([Point::new(3, 4), Point::new(2, 4), Point::new(2, 3)])
        );

        game.rotation(Direction::Left);

        assert!(game.walk());
        assert_eq!(
            game.snake,
            VecDeque::from([Point::new(3, 3), Point::new(3, 4), Point::new(2, 4)])
        );

        game.rotation(Direction::Up);

        assert!(game.walk());
        assert_eq!(
            game.snake,
            VecDeque::from([Point::new(2, 3), Point::new(3, 3), Point::new(3, 4)])
        );

        let board = Board::new("test".to_string(), 7, Vec::new());
        let mut game = Game::new(&board, 5, 1);
        game.food = Point::new(6, 6);

        game.snake = VecDeque::from([
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
        let board = Board::new("test".to_string(), 7, Vec::new());
        let mut game = Game::new(&board, 3, 1);

        assert_eq!(
            game.snake,
            VecDeque::from([Point::new(3, 4), Point::new(3, 3), Point::new(3, 2)])
        );

        game.food = Point::new(4, 5);

        assert!(game.walk());

        assert_eq!(
            game.snake,
            VecDeque::from([Point::new(3, 5), Point::new(3, 4), Point::new(3, 3)])
        );

        game.rotation(Direction::Down);

        assert_eq!(game.direction, Direction::Down);

        assert!(game.walk());
        assert_eq!(
            game.snake,
            VecDeque::from([
                Point::new(4, 5),
                Point::new(3, 5),
                Point::new(3, 4),
                Point::new(3, 3)
            ])
        );

        assert!(
            ![
                Point::new(4, 5),
                Point::new(3, 5),
                Point::new(3, 4),
                Point::new(3, 3)
            ]
            .contains(&game.food)
        );

        game.food = Point::new(0, 0);

        assert!(game.walk());
        assert_eq!(
            game.snake,
            VecDeque::from([
                Point::new(5, 5),
                Point::new(4, 5),
                Point::new(3, 5),
                Point::new(3, 4)
            ])
        );

        assert!(game.walk());
    }

    #[test]
    fn score_with_multiplier() {
        let board = Board::new("test".to_string(), 5, Vec::new());
        let mut game = Game::new(&board, 3, 3);

        game.food = Point::new(2, 4);
        assert!(game.walk());
        assert_eq!(game.get_score(), 3);
    }
}
