use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use crate::core::{Board, Boards, Direction, Wall};

enum State {
    Size,
    Name,
    Wall,
}
pub struct CreateBoardTui {
    name: String,
    board: Board,
    size: u16,
    exit: bool,
    finish: bool,
    wall: Wall,
    state: State,
    boards: Boards,
    error: String,
}

impl Default for CreateBoardTui {
    fn default() -> Self {
        Self::new()
    }
}

impl CreateBoardTui {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            board: Board::new(0, vec![]),
            size: 0,
            exit: false,
            finish: false,
            wall: Wall::new(0, 0),
            state: State::Size,
            boards: Boards::new(),
            error: "".to_string(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<(String, Board)> {
        while !(self.exit || self.finish) {
            terminal.draw(|frame| self.draw(frame))?;

            self.handle_events()?;
        }

        Ok((self.name.clone(), self.board.clone()))
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn key_event_put_wall(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_down(),
            KeyCode::Char('k') | KeyCode::Up => self.select_up(),
            KeyCode::Char('h') | KeyCode::Left => self.select_left(),
            KeyCode::Char('l') | KeyCode::Right => self.select_right(),
            KeyCode::Char(' ') => self.toggle_wall(),
            KeyCode::Enter => self.state = State::Name,
            _ => {}
        }
    }

    fn key_event_put_size(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char(c) if c.is_numeric() => {
                self.size = self.size * 10 + c.to_digit(10).unwrap() as u16
            }
            KeyCode::Backspace => self.size /= 10,
            KeyCode::Enter => {
                self.board = Board::new(self.size, vec![]);
                self.state = State::Wall;
            }

            _ => {}
        }
    }

    fn key_event_put_name(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => self.name.push(c),
            KeyCode::Backspace => {
                self.name.pop();
            }
            KeyCode::Enter => self.store(),

            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match self.state {
                State::Wall => self.key_event_put_wall(key_event),
                State::Size => self.key_event_put_size(key_event),
                State::Name => self.key_event_put_name(key_event),
            },
            _ => {}
        };
        Ok(())
    }

    fn select_down(&mut self) {
        self.wall = self.wall.get_neighbor(&Direction::Down, self.size);
    }

    fn select_up(&mut self) {
        self.wall = self.wall.get_neighbor(&Direction::Up, self.size);
    }

    fn select_left(&mut self) {
        self.wall = self.wall.get_neighbor(&Direction::Left, self.size);
    }

    fn select_right(&mut self) {
        self.wall = self.wall.get_neighbor(&Direction::Right, self.size);
    }

    fn toggle_wall(&mut self) {
        let wall = &self.wall;
        if self.board.is_wall(wall) {
            self.board.remove_wall(wall);
        } else {
            self.board.add_wall(wall.clone()).unwrap();
        }
    }

    fn store(&mut self) {
        match self.boards.add(self.name.clone(), self.board.clone()) {
            Ok(_) => self.finish = true,
            Err(e) => self.error = e,
        }
    }
}

impl CreateBoardTui {
    fn render_put_wall(&self, area: Rect, buf: &mut Buffer) {
        let mut selected_board = self.board.get_table();
        selected_board[(self.wall.get_x() + 1) as usize][(self.wall.get_y() + 1) as usize] =
            if selected_board[(self.wall.get_x() + 1) as usize][(self.wall.get_y() + 1) as usize]
                == " "
            {
                "■".to_string()
            } else {
                "▀".to_string()
            };
        let selected_board = selected_board.iter().map(|row| row.join("")).join("\n");

        let block = Block::new()
            .title(Line::raw(" Selected Board ").centered())
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        Paragraph::new(selected_board)
            .block(block)
            .alignment(Alignment::Center)
            .render(area, buf);
    }

    fn render_put_size(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Please enter size of board");
        let instructions = Line::from("Use Enter to save.");

        Paragraph::new(self.size.to_string())
            .block(
                Block::new()
                    .title(title.centered())
                    .title_bottom(instructions.centered()),
            )
            .centered()
            .render(area, buf);
    }

    fn render_put_name(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Please enter name of board");
        let instructions = Line::from("Use Enter to save.");

        let text = Text::from(vec![
            Line::from(self.name.clone()),
            Line::from(self.error.clone()).red(),
        ]);

        Paragraph::new(text)
            .block(
                Block::new()
                    .title(title.centered())
                    .title_bottom(instructions.centered()),
            )
            .centered()
            .render(area, buf);
    }
}

impl Widget for &mut CreateBoardTui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.state {
            State::Wall => self.render_put_wall(area, buf),
            State::Size => self.render_put_size(area, buf),
            State::Name => self.render_put_name(area, buf),
        }
    }
}
