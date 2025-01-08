mod core;
mod tui;

use core::{Boards, Direction, Game};
use tui::Tui;

#[tokio::main]
async fn main() {
    let a = Boards::read("/home/moosavi/Desktop/snake_game/src/a.json");
    println!("{:?}", a);
    match a.get("aa") {
        Some(x) => {
            let game = Game::new(x, 3);

            match Tui::tui(game).await {
                Ok(_) => {}
                Err(e) => println!("{e}"),
            }
        }
        None => {}
    }
}
