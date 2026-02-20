use std::{io, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
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
        let scores = self.scoreboard.get(&self.board_name).unwrap_or_default();
        let best = scores.first().copied().unwrap_or(self.score);
        let worst = scores.last().copied().unwrap_or(self.score);
        let runs = scores.len();
        let rank = scores
            .iter()
            .position(|score| *score == self.score)
            .map(|index| index + 1)
            .unwrap_or(runs);
        let is_best = self.score >= best;

        let frame_block = Block::new()
            .title(" Game Over ")
            .title_style(Style::new().fg(Color::Rgb(255, 134, 102)).bold())
            .title_bottom("Returning to board menu...")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::Rgb(89, 98, 114)));
        let inner = frame_block.inner(area);
        frame_block.render(area, buf);

        let [summary_area, rank_area] =
            Layout::vertical([Constraint::Length(8), Constraint::Fill(1)]).areas(inner);

        let summary = Text::from(vec![
            Line::from(format!("Board: {}", self.board_name)).bold(),
            Line::from(format!("Your score: {}", self.score)).fg(Color::Rgb(224, 242, 107)),
            Line::from(format!("Best score: {}", best)),
            Line::from(format!("Worst score: {}", worst)),
            Line::from(format!("Total runs: {}", runs)),
            Line::from(if is_best {
                "Result: New best run!".to_string()
            } else {
                format!("Result: Rank #{}", rank)
            })
            .fg(if is_best {
                Color::Rgb(123, 242, 149)
            } else {
                Color::Rgb(139, 205, 245)
            }),
        ]);

        Paragraph::new(summary)
            .block(
                Block::new()
                    .title(" Summary ")
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(Color::Rgb(89, 98, 114))),
            )
            .render(summary_area, buf);

        let mut top_lines = vec![Line::from("Top Scores").bold()];
        scores
            .iter()
            .take(5)
            .enumerate()
            .for_each(|(index, score)| {
                top_lines.push(Line::from(format!("#{:>2}  {}", index + 1, score)));
            });

        if scores.is_empty() {
            top_lines.push(Line::from("No scores yet."));
        }

        Paragraph::new(Text::from(top_lines))
            .wrap(Wrap { trim: true })
            .block(
                Block::new()
                    .title(" Leaderboard ")
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(Color::Rgb(89, 98, 114))),
            )
            .render(rank_area, buf);
    }
}
