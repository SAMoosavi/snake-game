use crate::{
    core::{Direction, Game, GameCell, GameSpeed},
    tui::board_view::{
        EMPTY_STYLE, board_rect, center_rect, fill_rect, fit_cell_width, paint_cell,
    },
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};
use std::{io, time};
use tokio::time::{Duration, sleep};

pub struct GameTui<'a> {
    game: Game<'a>,
    speed: GameSpeed,
    tick_duration: Duration,
    stop: bool,
    exit: bool,
}

impl<'a> GameTui<'a> {
    pub fn new(game: Game<'a>, speed: GameSpeed) -> Self {
        Self {
            game,
            speed,
            tick_duration: Duration::from_millis(speed.tick_millis()),
            stop: false,
            exit: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<u16> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.exit = !self.game.walk();

            if event::poll(time::Duration::from_millis(10))? {
                self.handle_events()?;
            }

            sleep(self.tick_duration).await;

            while self.stop && !self.exit {
                self.handle_events()?;
            }
        }

        Ok(self.game.get_score())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn key_event_play_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit = true,
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Char('k') => {
                self.game.rotation(Direction::Up)
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Char('j') => {
                self.game.rotation(Direction::Down)
            }
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Char('h') => {
                self.game.rotation(Direction::Left)
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Char('l') => {
                self.game.rotation(Direction::Right)
            }
            KeyCode::Esc => self.stop = true,
            _ => {}
        }
    }

    fn key_event_stop_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit = true,
            KeyCode::Esc => self.stop = false,
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if !self.stop {
                    self.key_event_play_mode(key_event)
                } else {
                    self.key_event_stop_mode(key_event)
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn render_board(&self, area: Rect, buf: &mut Buffer) {
        let status = if self.stop { "Paused" } else { "Running" };

        let board_block = Block::new()
            .title(
                Line::from(format!(
                    " Arena  Score: {}  Speed: {}  State: {} ",
                    self.game.get_score(),
                    self.speed.label(),
                    status
                ))
                .bold(),
            )
            .title_bottom(
                Line::from(format!(
                    "Arrow keys / WASD / HJKL: move   Esc: pause/resume   Q: quit   +{} points/food",
                    self.speed.score_multiplier()
                ))
                .centered(),
            )
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::Rgb(70, 85, 105)));
        let inner = board_block.inner(area);
        board_block.render(area, buf);

        let board = self.game.get_board();
        let board_size = board.len() as u16;
        if board_size == 0 || inner.width == 0 || inner.height == 0 {
            return;
        }

        let cell_width = fit_cell_width(inner, board_size, 4);
        let draw_area = board_rect(inner, board_size, cell_width);
        fill_rect(buf, draw_area, EMPTY_STYLE);

        board.iter().enumerate().for_each(|(row_index, row)| {
            row.iter().enumerate().for_each(|(col_index, cell)| {
                paint_cell(
                    buf,
                    draw_area,
                    row_index as u16,
                    col_index as u16,
                    cell_width,
                    match cell {
                        GameCell::Empty => EMPTY_STYLE,
                        GameCell::Wall => Style::new().bg(Color::Rgb(68, 73, 86)),
                        GameCell::SnakeBody => Style::new().bg(Color::Rgb(24, 160, 88)),
                        GameCell::SnakeHead => Style::new().bg(Color::Rgb(108, 222, 113)),
                        GameCell::Food => Style::new().bg(Color::Rgb(220, 80, 64)),
                    },
                );
            });
        });
    }
}

impl Widget for &GameTui<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [body_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

        self.render_board(body_area, buf);

        Paragraph::new("Wrap-around movement is enabled. Eat food, avoid walls and your body.")
            .centered()
            .render(footer_area, buf);

        if self.stop {
            let modal = center_rect(area, 34, 5);
            Clear.render(modal, buf);
            Paragraph::new("Game paused\nPress ESC to resume")
                .centered()
                .block(
                    Block::new()
                        .title(" Pause ")
                        .borders(Borders::ALL)
                        .border_style(Style::new().fg(Color::Rgb(255, 196, 91))),
                )
                .render(modal, buf);
        }
    }
}
