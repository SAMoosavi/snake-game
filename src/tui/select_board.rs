use crate::{
    core::{Board, BoardCell, Boards, GameSpeed, Scoreboard},
    tui::board_view::{
        EMPTY_STYLE, board_rect, center_rect, fill_rect, fit_cell_width, paint_cell,
    },
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{
        Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Paragraph,
        StatefulWidget, Widget,
    },
};
use std::io;

pub enum SelectBoardTuiResult {
    Board(Board, GameSpeed),
    Exit,
    CreateBoard,
    ScoreBoards,
}

pub struct SelectBoardTui {
    exit: bool,
    selected: bool,
    create_board: bool,
    show_scoreboards: bool,
    confirm_remove: bool,
    boards: Boards,
    board_names: Vec<String>,
    state: ListState,
    speed: GameSpeed,
}

impl Default for SelectBoardTui {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectBoardTui {
    pub fn new() -> Self {
        let boards = Boards::new();
        let board_names = boards.get_names();

        let mut state = ListState::default();
        if !board_names.is_empty() {
            state.select_first();
        }

        Self {
            exit: false,
            selected: false,
            create_board: false,
            show_scoreboards: false,
            confirm_remove: false,
            state,
            boards,
            board_names,
            speed: GameSpeed::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<SelectBoardTuiResult> {
        while !(self.exit || self.selected || self.create_board || self.show_scoreboards) {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        let select_board_tui_result = if self.exit {
            SelectBoardTuiResult::Exit
        } else if self.create_board {
            SelectBoardTuiResult::CreateBoard
        } else if self.show_scoreboards {
            SelectBoardTuiResult::ScoreBoards
        } else if let Some(board) = self.selected_board() {
            SelectBoardTuiResult::Board(board, self.speed)
        } else {
            SelectBoardTuiResult::Exit
        };

        Ok(select_board_tui_result)
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn key_event(&mut self, key_event: KeyEvent) {
        if self.confirm_remove {
            self.key_event_confirm_remove_mode(key_event);
            return;
        }

        match key_event.code {
            KeyCode::Enter => {
                if self.selected_board().is_some() {
                    self.selected = true;
                }
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit = true,
            KeyCode::Char('c') | KeyCode::Char('C') => self.create_board = true,
            KeyCode::Char('s') | KeyCode::Char('S') => self.show_scoreboards = true,
            KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Delete => {
                if self.selected_board().is_some() {
                    self.confirm_remove = true;
                }
            }
            KeyCode::Left | KeyCode::Char('h') => self.speed = self.speed.slower(),
            KeyCode::Right | KeyCode::Char('l') => self.speed = self.speed.faster(),
            KeyCode::Char('j') | KeyCode::Down => self.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.select_previous(),
            _ => {}
        }
    }

    fn key_event_confirm_remove_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => self.remove_selected_board(),
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => self.confirm_remove = false,
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit = true,
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

    fn selected_index(&self) -> Option<usize> {
        self.state
            .selected()
            .filter(|index| *index < self.board_names.len())
    }

    fn selected_board(&self) -> Option<Board> {
        self.selected_index()
            .and_then(|index| self.boards.get(index))
            .cloned()
    }

    fn remove_selected_board(&mut self) {
        let Some(index) = self.selected_index() else {
            self.confirm_remove = false;
            return;
        };

        if let Some(board) = self.boards.remove(index) {
            let mut scoreboard = Scoreboard::new();
            scoreboard.remove_board(board.get_name());
        }

        self.board_names = self.boards.get_names();

        if self.board_names.is_empty() {
            self.state.select(None);
        } else {
            self.state
                .select(Some(index.min(self.board_names.len() - 1)));
        }

        self.confirm_remove = false;
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!(
            "Select Board | Speed: {} (x{})",
            self.speed.label(),
            self.speed.score_multiplier()
        ))
        .bold()
        .centered()
        .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new(
            "Arrow keys / J K: move  Arrow left/right or H/L: speed  Enter: play  R: remove  C: create  S: scores  Q: quit",
        )
        .centered()
        .render(area, buf);
    }

    fn render_list_of_name(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(" Boards ").centered())
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::Rgb(70, 85, 105)));

        if self.board_names.is_empty() {
            Paragraph::new("No boards found.\nPress C to create your first board.")
                .centered()
                .block(block)
                .render(area, buf);
            return;
        }

        let items: Vec<_> = self
            .board_names
            .iter()
            .map(|name| ListItem::from(name.to_string()))
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
        let Some(selected_board) = self.selected_board() else {
            Paragraph::new("Board preview is empty.\nCreate a board to start playing.")
                .centered()
                .block(
                    Block::new()
                        .title(Line::raw(" Preview ").centered())
                        .borders(Borders::ALL)
                        .border_style(Style::new().fg(Color::Rgb(70, 85, 105))),
                )
                .render(area, buf);
            return;
        };

        let board = selected_board.get_board();
        let block = Block::new()
            .title(Line::raw(" Preview ").centered())
            .borders(Borders::ALL)
            .border_style(Style::new().fg(Color::Rgb(70, 85, 105)));
        let inner = block.inner(area);
        block.render(area, buf);

        if board.is_empty() || inner.width == 0 || inner.height == 0 {
            return;
        }

        let board_size = selected_board.get_size();
        let cell_width = fit_cell_width(inner, board_size, 2);
        let draw_area = board_rect(inner, board_size, cell_width);
        fill_rect(buf, draw_area, EMPTY_STYLE);

        board.iter().enumerate().for_each(|(row_index, row)| {
            row.iter().enumerate().for_each(|(col_index, cell)| {
                paint_cell(
                    buf,
                    draw_area,
                    row_index as u16,
                    col_index as u16,
                    cell_width,
                    match cell {
                        BoardCell::Empty => EMPTY_STYLE,
                        BoardCell::Wall => Style::new().bg(Color::Rgb(83, 89, 102)),
                    },
                );
            });
        });
    }

    fn render_remove_confirm(&self, area: Rect, buf: &mut Buffer) {
        if !self.confirm_remove {
            return;
        }

        let board_name = self
            .selected_board()
            .map(|board| board.get_name().to_string());
        let text = if let Some(board_name) = board_name {
            format!(
                "Remove '{}'?\nThis will also remove its scoreboard data.\nY: confirm  N: cancel",
                board_name
            )
        } else {
            "No board selected.\nN: cancel".to_string()
        };

        let modal = center_rect(area, 54, 6);
        Clear.render(modal, buf);
        Paragraph::new(text)
            .centered()
            .block(
                Block::new()
                    .title(" Remove Board ")
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(Color::Rgb(245, 116, 59))),
            )
            .render(modal, buf);
    }
}

impl Widget for &mut SelectBoardTui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::horizontal([Constraint::Percentage(35), Constraint::Percentage(65)])
                .areas(main_area);

        self.render_header(header_area, buf);
        SelectBoardTui::render_footer(footer_area, buf);
        self.render_list_of_name(list_area, buf);
        self.render_selected_item(item_area, buf);
        self.render_remove_confirm(area, buf);
    }
}
