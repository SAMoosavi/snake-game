use crate::{Board, Direction};
use std::{io, time};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use tokio::time::{sleep, Duration};

struct App {
    game: Board,
    stop: bool,
    exit: bool,
}

impl App {
    pub fn new(game: Board) -> Self {
        Self {
            game,
            stop: false,
            exit: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.stop && !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(time::Duration::from_millis(10))? {
                self.handle_events()?;
            }
            self.stop = !self.game.walk();

            sleep(Duration::from_millis(80)).await;
        }

        sleep(Duration::from_secs(1)).await;

        terminal.clear()?;
        while !self.exit {
            terminal.draw(|f| {
                Paragraph::new(format!(
                    "Game Over!\nYour score is {}\nPress 'q' to quit.",
                    self.game.get_score()
                ))
                .block(Block::default().borders(Borders::ALL))
                .render(f.area(), f.buffer_mut());
            })?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Up => self.game.rotation(Direction::Up),
            KeyCode::Down => self.game.rotation(Direction::Down),
            KeyCode::Left => self.game.rotation(Direction::Left),
            KeyCode::Right => self.game.rotation(Direction::Right),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(
            format!(" Your score {} ", self.game.get_score())
                .bold()
                .blue(),
        );
        let instructions = Line::from(vec![
            " Left ".into(),
            "<\u{2190}>".blue().bold(),
            " Up ".into(),
            "<\u{2191}>".blue().bold(),
            " Right ".into(),
            "<\u{2192}>".blue().bold(),
            " Down ".into(),
            "<\u{2193}>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
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
                    .border_set(border::FULL),
            )
            .centered()
            .render(area, buf);
    }
}
pub struct Tui {}

impl Tui {
    pub async fn tui(game: Board) -> Result<(), std::io::Error> {
        let mut terminal = ratatui::init();
        let app_result = App::new(game).run(&mut terminal).await;
        ratatui::restore();
        app_result
    }
}
