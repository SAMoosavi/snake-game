use std::io;

use crate::core::Scoreboard;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget, Wrap,
    },
};

pub struct ScoreboardTui {
    scoreboard: Scoreboard,
    exit: bool,
    state: ListState,
    board_names: Vec<String>,
}

impl Default for ScoreboardTui {
    fn default() -> Self {
        Self::new()
    }
}

impl ScoreboardTui {
    pub fn new() -> Self {
        let scoreboard = Scoreboard::new();
        let mut board_names = scoreboard.get_names();
        board_names.sort_by(|a, b| {
            let a_best = scoreboard
                .get(a)
                .and_then(|scores| scores.first().copied())
                .unwrap_or_default();
            let b_best = scoreboard
                .get(b)
                .and_then(|scores| scores.first().copied())
                .unwrap_or_default();
            b_best.cmp(&a_best).then_with(|| a.cmp(b))
        });

        let mut state = ListState::default();
        if !board_names.is_empty() {
            state.select_first();
        }

        Self {
            state,
            scoreboard,
            board_names,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.select_last(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn select_first(&mut self) {
        if !self.board_names.is_empty() {
            self.state.select(Some(0));
        }
    }

    fn select_last(&mut self) {
        if !self.board_names.is_empty() {
            self.state.select(Some(self.board_names.len() - 1));
        }
    }

    fn selected_board_name(&self) -> Option<&str> {
        self.state
            .selected()
            .and_then(|index| self.board_names.get(index))
            .map(String::as_str)
    }

    fn selected_scores(&self) -> Vec<u16> {
        self.selected_board_name()
            .and_then(|board_name| self.scoreboard.get(board_name))
            .unwrap_or_default()
    }

    fn total_runs(&self) -> usize {
        self.board_names
            .iter()
            .map(|board_name| {
                self.scoreboard
                    .get(board_name)
                    .map_or(0, |scores| scores.len())
            })
            .sum()
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let subtitle = format!(
            "boards: {} | total runs: {}",
            self.board_names.len(),
            self.total_runs()
        );
        Paragraph::new(Text::from(vec![
            Line::from("Scoreboard").bold(),
            Line::from(subtitle).fg(Color::Rgb(134, 142, 150)),
        ]))
        .centered()
        .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Arrow keys / J K: move  Home/End or g/G: first/last  Q: back")
            .centered()
            .render(area, buf);
    }

    fn render_list_of_name(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(" Boards ").centered())
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::Rgb(70, 85, 105)));

        if self.board_names.is_empty() {
            Paragraph::new("No records yet.\nPlay a game to create scoreboard data.")
                .centered()
                .block(block)
                .render(area, buf);
            return;
        }

        let items: Vec<_> = self
            .board_names
            .iter()
            .map(|board_name| {
                let scores = self.scoreboard.get(board_name).unwrap_or_default();
                let best = scores.first().copied().unwrap_or_default();
                let runs = scores.len();
                ListItem::from(Line::from(format!(
                    "{:<14} best {:>3}   runs {:>3}",
                    board_name, best, runs
                )))
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::new()
                    .fg(Color::Rgb(152, 224, 122))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(" Details ").centered())
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::Rgb(70, 85, 105)));

        let Some(board_name) = self.selected_board_name() else {
            Paragraph::new("Select a board once records exist.")
                .block(block)
                .centered()
                .render(area, buf);
            return;
        };

        let selected_scores = self.selected_scores();
        if selected_scores.is_empty() {
            Paragraph::new(format!("{} has no scores yet.", board_name))
                .block(block)
                .centered()
                .render(area, buf);
            return;
        }

        let runs = selected_scores.len();
        let best = selected_scores[0];
        let worst = *selected_scores.last().unwrap_or(&best);
        let total: usize = selected_scores.iter().map(|score| *score as usize).sum();
        let average = total as f64 / runs as f64;

        let mut lines = vec![
            Line::from(format!("Board: {}", board_name)).bold(),
            Line::from(""),
            Line::from(format!("Best:  {}", best)),
            Line::from(format!("Worst: {}", worst)),
            Line::from(format!("Runs:  {}", runs)),
            Line::from(format!("Avg:   {:.2}", average)),
            Line::from(""),
            Line::from("Top Scores").bold(),
        ];

        selected_scores
            .iter()
            .take(10)
            .enumerate()
            .for_each(|(index, score)| {
                lines.push(Line::from(format!("#{:>2}  {}", index + 1, score)));
            });

        Paragraph::new(Text::from(lines))
            .block(block)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

impl Widget for &mut ScoreboardTui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                .areas(main_area);

        self.render_header(header_area, buf);
        ScoreboardTui::render_footer(footer_area, buf);
        self.render_list_of_name(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}
