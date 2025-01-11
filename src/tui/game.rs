use crate::{core::Game, Direction};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span},
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
            KeyCode::Char('q') => self.exit = true,
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
            KeyCode::Char('q') => self.exit = true,
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
        let title = Line::from(vec![
            " Your score ".into(),
            format!("{} ", self.game.get_score()).bold().red(),
        ]);

        let instructions = Line::from(vec![
            " Left ".into(),
            "\u{2190}".red().bold(),
            " Up ".into(),
            "\u{2191}".red().bold(),
            " Right ".into(),
            "\u{2192}".red().bold(),
            " Down ".into(),
            "\u{2193}".red().bold(),
            " Quit ".into(),
            "Q ".red().bold(),
        ]);

        let table = self.game.get_table();
        let text = table
            .iter()
            .map(|row| Line::from(row.iter().map(Span::from).collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        Paragraph::new(text)
            .block(
                Block::bordered()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::ROUNDED)
                    .border_style(Style::default().fg(Color::Blue)),
            )
            .centered()
            .bg(Color::Black)
            .fg(Color::Green)
            .render(area, buf);
    }
}
