use crate::core::{Direction, Game};

use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::{io, time};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use tokio::time::{sleep, Duration};

pub struct GameTui<'a> {
    game: Game<'a>,
    stop: bool,
    exit: bool,
}

impl<'a> GameTui<'a> {
    pub fn new(game: Game<'a>) -> Self {
        Self {
            game,
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

            sleep(Duration::from_millis(80)).await;

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
            KeyCode::Up | KeyCode::Char('k') => self.game.rotation(Direction::Up),
            KeyCode::Down | KeyCode::Char('j') => self.game.rotation(Direction::Down),
            KeyCode::Left | KeyCode::Char('h') => self.game.rotation(Direction::Left),
            KeyCode::Right | KeyCode::Char('l') => self.game.rotation(Direction::Right),
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
}

impl<'a> Widget for &GameTui<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!("Your score {}", self.game.get_score()));
        let instructions =
            Line::from("Use 🠀 🠂 🠁 🠃 or h j k l to move, esc to stop/play, q/Q to quit game.");

        let table = self.game.get_table();
        let text = table.iter().map(|row| row.join("")).join("\n");

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
