mod game;

use crate::core::Game;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use game::GameTui;
use ratatui::{
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal,
};
use tokio::time::{sleep, Duration};

struct App<'a> {
    game_tui: GameTui<'a>,
    exit: bool,
}

impl<'a> App<'a> {
    pub fn new(game: Game<'a>) -> Self {
        Self {
            game_tui: GameTui::new(game),
            exit: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let score = self.game_tui.run(terminal).await?;

        sleep(Duration::from_secs(1)).await;

        terminal.clear()?;
        while !self.exit {
            terminal.draw(|f| {
                Paragraph::new(format!(
                    "Game Over!\nYour score is {}\nPress 'q' to quit.",
                    score
                ))
                .block(Block::default().borders(Borders::ALL))
                .render(f.area(), f.buffer_mut());
            })?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
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

pub struct Tui {}

impl Tui {
    pub async fn tui(game: Game<'_>) -> Result<(), std::io::Error> {
        let mut terminal = ratatui::init();
        let app_result = App::new(game).run(&mut terminal).await;
        ratatui::restore();
        app_result
    }
}
