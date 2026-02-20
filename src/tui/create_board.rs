use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{
    core::{Board, BoardCell, Boards, Direction, Wall},
    tui::board_view::{EMPTY_STYLE, board_rect, fill_rect, fit_cell_width, paint_cell},
};

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
            name: String::new(),
            board: Board::new(String::new(), 0, vec![]),
            size: 0,
            exit: false,
            finish: false,
            wall: Wall::new(0, 0),
            state: State::Size,
            boards: Boards::new(),
            error: String::new(),
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
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit = true,
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
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit = true,
            KeyCode::Char(c) if c.is_numeric() => {
                self.size = self.size.saturating_mul(10) + c.to_digit(10).unwrap() as u16;
            }
            KeyCode::Backspace => self.size /= 10,
            KeyCode::Enter => {
                if self.size < 5 {
                    self.error = "Board size must be at least 5.".to_string();
                } else {
                    self.board = Board::new(String::new(), self.size, vec![]);
                    self.wall = Wall::new(0, 0);
                    self.error.clear();
                    self.state = State::Wall;
                }
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
        if self.name.trim().is_empty() {
            self.error = "Board name cannot be empty.".to_string();
            return;
        }

        self.board = self.board.copy_with_new_name(self.name.clone());
        match self.boards.add(self.name.clone(), self.board.clone()) {
            Ok(_) => self.finish = true,
            Err(e) => self.error = e,
        }
    }

    fn render_put_wall(&self, area: Rect, buf: &mut Buffer) {
        let [board_area, info_area] =
            Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(area);

        let board_block = Block::new()
            .title(" Edit Walls ".bold())
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::Rgb(70, 85, 105)));
        let board_inner = board_block.inner(board_area);
        board_block.render(board_area, buf);

        let board = self.board.get_board();
        if !board.is_empty() && board_inner.width > 0 && board_inner.height > 0 {
            let cell_width = fit_cell_width(board_inner, self.board.get_size(), 2);
            let draw_area = board_rect(board_inner, self.board.get_size(), cell_width);
            fill_rect(buf, draw_area, EMPTY_STYLE);

            board.iter().enumerate().for_each(|(row_index, row)| {
                row.iter().enumerate().for_each(|(col_index, cell)| {
                    let is_cursor = self.wall.get_x() == row_index as i16
                        && self.wall.get_y() == col_index as i16;
                    let style = match (cell, is_cursor) {
                        (BoardCell::Empty, false) => EMPTY_STYLE,
                        (BoardCell::Wall, false) => Style::new().bg(Color::Rgb(87, 93, 109)),
                        (BoardCell::Empty, true) => Style::new().bg(Color::Rgb(215, 166, 51)),
                        (BoardCell::Wall, true) => Style::new().bg(Color::Rgb(245, 116, 59)),
                    };
                    paint_cell(
                        buf,
                        draw_area,
                        row_index as u16,
                        col_index as u16,
                        cell_width,
                        style,
                    );
                });
            });
        }

        let info = Text::from(vec![
            Line::from("Controls").bold(),
            Line::from("Arrow keys / H J K L: move cursor"),
            Line::from("Space: toggle wall"),
            Line::from("Enter: name board"),
            Line::from("Q: cancel"),
            Line::from(""),
            Line::from(format!(
                "Cursor: ({}, {})",
                self.wall.get_x(),
                self.wall.get_y()
            )),
            Line::from(format!("Walls: {}", self.board.get_walls().len())),
        ]);

        Paragraph::new(info)
            .block(
                Block::new()
                    .title(" Guide ".bold())
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(Color::Rgb(70, 85, 105))),
            )
            .render(info_area, buf);
    }

    fn render_put_size(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Create Board: Set Size");
        let instructions = Line::from("Type digits, Enter to continue, Q to cancel.");

        let text = Text::from(vec![
            Line::from(format!("Size: {}", self.size)),
            Line::from(self.error.clone()).fg(Color::Rgb(220, 90, 90)),
        ]);

        Paragraph::new(text)
            .block(
                Block::new()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .borders(Borders::ALL),
            )
            .centered()
            .render(area, buf);
    }

    fn render_put_name(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Create Board: Set Name");
        let instructions = Line::from("Type name, Enter to save.");

        let text = Text::from(vec![
            Line::from(self.name.clone()),
            Line::from(self.error.clone()).fg(Color::Rgb(220, 90, 90)),
        ]);

        Paragraph::new(text)
            .block(
                Block::new()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .borders(Borders::ALL),
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
