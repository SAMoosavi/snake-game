use std::io;

use crate::core::Scoreboard;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
    DefaultTerminal, Frame,
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
        let board_names = scoreboard.get_names();

        let mut state = ListState::default();
        state.select_first();

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
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
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

    fn selected_board(&self) -> Vec<u16> {
        let index = self.state.selected().unwrap();
        let board_name = &self.board_names[index];
        self.scoreboard.get(board_name).unwrap().clone()
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Scoreboard")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, q/Q to back.")
            .centered()
            .render(area, buf);
    }

    fn render_list_of_name(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(" Board Names ").centered())
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        let items: Vec<_> = self
            .board_names
            .iter()
            .map(|todo_item| ListItem::from(todo_item.to_string()))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let selected_scoreboard = self
            .selected_board()
            .iter()
            .enumerate()
            .map(|(index, score)| format!("{}: {}", index, score))
            .join("\n");

        let block = Block::new()
            .title(Line::raw(" Selected Board ").centered())
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        Paragraph::new(selected_scoreboard)
            .block(block)
            .alignment(Alignment::Center)
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
            Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)])
                .areas(main_area);

        ScoreboardTui::render_header(header_area, buf);
        ScoreboardTui::render_footer(footer_area, buf);
        self.render_list_of_name(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}
