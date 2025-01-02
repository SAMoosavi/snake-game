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

        let range = if length % 2 == 0 {
            -offset..=offset - 1
        } else {
            -offset..=offset
        };

        for i in range {
            game_table.push_front(Point::new(half, half + i));
        }

        game_table
    }

    pub fn get_table(&self) -> Vec<Vec<String>> {
        let mut result =
            vec![vec![":".to_string(); self.table_size as usize]; self.table_size as usize];

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

        let is_out_of_bounds = new_head.get_x() < 0
            || new_head.get_y() < 0
            || new_head.get_x() >= self.table_size as i16
            || new_head.get_y() >= self.table_size as i16;

        let collides_with_body = self.game_table.contains(&new_head);

        if is_out_of_bounds || collides_with_body {
            self.game_table.pop_back();
            self.game_table.pop_front();

            false
        } else if new_head == self.food {
            self.game_table.push_front(new_head);
            self.score += 1;
            self.food = Self::find_lunch_point(self.table_size, &self.game_table);

            true
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
                rng.gen_range(0..=table_size as i16),
                rng.gen_range(0..=table_size as i16),
            );

            if !game_table.contains(&point) {
                return point;
            }
        }
    }

    #[cfg(test)]
    pub fn print(&self) {
        self.get_table()
            .iter()
            .for_each(|row| println!("{}", row.concat()));
    }
}

#[cfg(test)]
mod test_board {
    use super::{super::point::Point, Board, Direction};
    use std::collections::LinkedList;

    #[test]
    fn check_create_size() {
        assert!(Board::new(2, 3).is_err());
        assert!(Board::new(3, 3).is_err());
        assert!(Board::new(4, 3).is_ok());

        assert!(Board::new(3, 4).is_err());
        assert!(Board::new(4, 4).is_err());
        assert!(Board::new(5, 4).is_ok());
    }

    #[test]
    fn check_create_table() {
        let odd_n_odd_len = Board::create_table(7, 3);
        assert_eq!(
            odd_n_odd_len,
            LinkedList::from([Point::new(3, 4), Point::new(3, 3), Point::new(3, 2)])
        );

        let even_n_odd_len = Board::create_table(8, 3);
        assert_eq!(
            even_n_odd_len,
            LinkedList::from([Point::new(3, 4), Point::new(3, 3), Point::new(3, 2)])
        );

        let odd_n_even_len = Board::create_table(7, 4);
        assert_eq!(
            odd_n_even_len,
            LinkedList::from([
                Point::new(3, 4),
                Point::new(3, 3),
                Point::new(3, 2),
                Point::new(3, 1)
            ])
        );

        let even_n_even_len = Board::create_table(8, 4);
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
        let mut game = Board::new(5, 3).unwrap();
        game.food = Point::new(0, 0);

        assert_eq!(
            game.game_table,
            LinkedList::from([Point::new(2, 3), Point::new(2, 2), Point::new(2, 1)])
        );
        assert!(game.walk());
        assert_eq!(
            game.game_table,
            LinkedList::from([Point::new(2, 4), Point::new(2, 3), Point::new(2, 2)])
        );
        game.rotation(Direction::Down);

        assert!(game.walk());
        assert_eq!(
            game.game_table,
            LinkedList::from([Point::new(3, 4), Point::new(2, 4), Point::new(2, 3)])
        );

        game.rotation(Direction::Left);

        assert!(game.walk());
        assert_eq!(
            game.game_table,
            LinkedList::from([Point::new(3, 3), Point::new(3, 4), Point::new(2, 4)])
        );

        game.rotation(Direction::Up);

        assert!(game.walk());
        assert_eq!(
            game.game_table,
            LinkedList::from([Point::new(2, 3), Point::new(3, 3), Point::new(3, 4)])
        );

        let mut game = Board::new(7, 5).unwrap();
        game.food = Point::new(6, 6);

        game.game_table = LinkedList::from([
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
        let mut game = Board::new(7, 3).unwrap();

        assert_eq!(
            game.game_table,
            LinkedList::from([Point::new(3, 4), Point::new(3, 3), Point::new(3, 2)])
        );

        game.food = Point::new(4, 5);

        assert!(game.walk());

        assert_eq!(
            game.game_table,
            LinkedList::from([Point::new(3, 5), Point::new(3, 4), Point::new(3, 3)])
        );

        game.rotation(Direction::Down);

        assert_eq!(game.direction, Direction::Down);

        assert!(game.walk());
        assert_eq!(
            game.game_table,
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
            game.game_table,
            LinkedList::from([
                Point::new(5, 5),
                Point::new(4, 5),
                Point::new(3, 5),
                Point::new(3, 4)
            ])
        );

        assert!(game.walk());
        assert!(!game.walk());
    }
}
