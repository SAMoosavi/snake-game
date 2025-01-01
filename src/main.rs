mod core;
mod tui;

use core::{Board, Direction};
use tui::Tui;

#[tokio::main]
async fn main() {
    match Board::<20>::new(3) {
        Ok(game) => match Tui::new(game).await {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        },
        Err(e) => println!("{e}"),
    }
}
