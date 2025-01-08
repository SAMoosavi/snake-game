mod core;
mod tui;

use core::{Board, Direction, Game};
use tui::Tui;

#[tokio::main]
async fn main() {
    let board = Board::default();
    let game = Game::new(board, 3);

    match Tui::tui(game).await {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    }
}
