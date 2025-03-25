use std::{io, time::Duration};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Line,
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use tokio::time::sleep;

use crate::core::Scoreboard;

#[derive(Debug)]
pub struct GameOverTui {
    score: u16,
    scoreboard: Scoreboard,
    board_name: String,
}

impl GameOverTui {
    pub fn new(board_name: String, score: u16) -> Self {
        let mut scoreboard = Scoreboard::new();
        scoreboard.add(board_name.clone(), score);

        Self {
            score,
            scoreboard,
            board_name,
        }
    }

    pub async fn run(&self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;

        sleep(Duration::from_millis(3000)).await;

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &GameOverTui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("Game Over :(!");

        let scores = self.scoreboard.get(&self.board_name).unwrap();

        let suffix = match self.score == scores[0] {
            true => "The best record.".to_string(),
            false => format!("The best record is {}", scores[0]),
        };
        let text = format!(
            "Your score is {} in the {} board.\n{}",
            self.score, self.board_name, suffix
        );

        Paragraph::new(text)
            .block(
                Block::new()
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .title(title.centered()),
            )
            .centered()
            .render(area, buf);
    }
}
