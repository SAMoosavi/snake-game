mod core;
mod tui;

use core::{Board, Direction};
use tui::Tui;

#[tokio::main]
async fn main() {
    match Board::new(20, 3) {
        Ok(game) => match Tui::tui(game).await {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        },
        Err(e) => println!("{e}"),
    }
}
