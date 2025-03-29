mod create_board;
mod game;
mod game_over;
mod scoreboard;
mod select_board;

use crate::core::{Board, Game};

use std::io;

use create_board::CreateBoardTui;
use game::GameTui;
use game_over::GameOverTui;
use ratatui::DefaultTerminal;
use scoreboard::ScoreboardTui;
use select_board::{SelectBoardTui, SelectBoardTuiResult};

enum State {
    SelectBoard,
    CreateBoard,
    PlayGame(Board),
    GameOver(u16),
    Scoreboard,
}

struct App {
    state: State,
    exit: bool,
    board_name: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            state: State::SelectBoard,
            exit: false,
            board_name: "".to_string(),
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.state = match &self.state {
                State::SelectBoard => {
                    let mut select_board_tui = SelectBoardTui::new();

                    match select_board_tui.run(terminal)? {
                        SelectBoardTuiResult::Board(board_name, board) => {
                            self.board_name = board_name;
                            State::PlayGame(board)
                        }
                        SelectBoardTuiResult::Exit => {
                            self.exit = true;
                            State::SelectBoard
                        }
                        SelectBoardTuiResult::CreateBoard => State::CreateBoard,
                        SelectBoardTuiResult::ScoreBoards => State::Scoreboard,
                    }
                }
                State::CreateBoard => {
                    let mut create_board_tui = CreateBoardTui::new();
                    create_board_tui.run(terminal)?;
                    State::SelectBoard
                }
                State::PlayGame(board) => {
                    let mut game_tui = GameTui::new(Game::new(board, 3));
                    let score = game_tui.run(terminal).await?;
                    State::GameOver(score)
                }
                State::GameOver(score) => {
                    let game_over_tui = GameOverTui::new(self.board_name.clone(), *score);
                    game_over_tui.run(terminal).await?;

                    State::SelectBoard
                }
                State::Scoreboard => {
                    let mut scoreboard = ScoreboardTui::new();
                    scoreboard.run(terminal)?;
                    State::SelectBoard
                }
            };
        }

        terminal.clear()?;

        Ok(())
    }
}

pub struct Tui {}

impl Tui {
    pub async fn render() -> Result<(), std::io::Error> {
        let mut terminal = ratatui::init();
        let app_result = App::new().run(&mut terminal).await;
        ratatui::restore();
        app_result
    }
}
