mod core;
mod tui;

use tui::Tui;

#[tokio::main]
async fn main() {
    match Tui::tui().await {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    }
}
