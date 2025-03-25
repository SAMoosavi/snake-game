mod core;
mod tui;

use tui::Tui;

#[tokio::main]
async fn main() {
    match Tui::render().await {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    }
}
