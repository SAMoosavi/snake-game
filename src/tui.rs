mod create_board;
mod game;
mod select_board;

use crate::core::{Board, Game};
use std::{io, time::Duration};
use tokio::time::sleep;

use create_board::CreateBoardTui;
use game::GameTui;
use ratatui::{
    widgets::{Paragraph, Widget},
    DefaultTerminal,
};
use select_board::{SelectBoardTui, SelectBoardTuiResult};

enum State {
    SelectBoard,
    CreateBoard,
    PlayGame(Board),
    GameOver(u16),
}

struct App {
    state: State,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::SelectBoard,
            exit: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.state = match &self.state {
                State::SelectBoard => {
                    let mut select_board_tui =
                        SelectBoardTui::new("/home/moosavi/Desktop/snake_game/src/a.json");

                    match select_board_tui.run(terminal)? {
                        SelectBoardTuiResult::Board(board) => State::PlayGame(board),
                        SelectBoardTuiResult::Exit => {
                            self.exit = true;
                            State::SelectBoard
                        }
                        SelectBoardTuiResult::CreateBoard => State::CreateBoard,
                    }
                }
                State::CreateBoard => {
                    let mut create_board_tui = CreateBoardTui::new("/home/moosavi/Desktop/snake_game/src/a.json");
                    create_board_tui.run(terminal)?;
                    State::SelectBoard
                }
                State::PlayGame(board) => {
                    let mut game_tui = GameTui::new(Game::new(board, 3));
                    let score = game_tui.run(terminal).await?;
                    State::GameOver(score)
                }
                State::GameOver(score) => {
                    terminal.draw(|f| {
                        Paragraph::new(format!("Game Over!\nYour score is {}", score))
                            .render(f.area(), f.buffer_mut());
                    })?;

                    sleep(Duration::from_millis(1000)).await;

                    self.exit = true;

                    State::GameOver(*score)
                }
            };
        }

        terminal.clear()?;

        Ok(())
    }
}

pub struct Tui {}

impl Tui {
    pub async fn tui() -> Result<(), std::io::Error> {
        let mut terminal = ratatui::init();
        let app_result = App::new().run(&mut terminal).await;
        ratatui::restore();
        app_result
    }
}
