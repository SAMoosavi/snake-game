use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};

pub const EMPTY_STYLE: Style = Style::new().bg(Color::Rgb(16, 20, 24));
const CELL_WIDTH_RATIO: u16 = 2;
const CELL_HEIGHT_RATIO: u16 = 1;

fn cell_height(cell_width: u16) -> u16 {
    if cell_width <= 1 {
        1
    } else {
        (cell_width / CELL_WIDTH_RATIO).max(CELL_HEIGHT_RATIO)
    }
}

pub fn fit_cell_width(area: Rect, board_size: u16, preferred: u16) -> u16 {
    if board_size == 0 {
        return 1;
    }

    let max_width_by_space = area.width / board_size;
    let max_height_by_space = area.height / board_size;
    let max_width_by_height = max_height_by_space.saturating_mul(CELL_WIDTH_RATIO);
    let mut fitted = max_width_by_space
        .min(max_width_by_height)
        .clamp(1, preferred);

    if fitted > 1 && !fitted.is_multiple_of(2) {
        fitted -= 1;
    }

    fitted.max(1)
}

pub fn center_rect(area: Rect, width: u16, height: u16) -> Rect {
    let clipped_width = width.min(area.width);
    let clipped_height = height.min(area.height);

    let x = area.x + (area.width.saturating_sub(clipped_width)) / 2;
    let y = area.y + (area.height.saturating_sub(clipped_height)) / 2;

    Rect::new(x, y, clipped_width, clipped_height)
}

pub fn board_rect(area: Rect, board_size: u16, cell_width: u16) -> Rect {
    let cell_height = cell_height(cell_width);
    center_rect(
        area,
        board_size.saturating_mul(cell_width),
        board_size.saturating_mul(cell_height),
    )
}

pub fn fill_rect(buf: &mut Buffer, area: Rect, style: Style) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            let cell = &mut buf[(x, y)];
            cell.set_symbol(" ");
            cell.set_style(style);
        }
    }
}

pub fn paint_cell(
    buf: &mut Buffer,
    board_area: Rect,
    row: u16,
    col: u16,
    cell_width: u16,
    style: Style,
) {
    if board_area.width == 0 || board_area.height == 0 || cell_width == 0 {
        return;
    }

    let cell_height = cell_height(cell_width);

    let y = board_area.y.saturating_add(row.saturating_mul(cell_height));
    if y >= board_area.y + board_area.height {
        return;
    }

    let x = board_area.x.saturating_add(col.saturating_mul(cell_width));
    if x >= board_area.x + board_area.width {
        return;
    }

    let width = (board_area.x + board_area.width)
        .saturating_sub(x)
        .min(cell_width);

    let height = (board_area.y + board_area.height)
        .saturating_sub(y)
        .min(cell_height);

    for dy in 0..height {
        for dx in 0..width {
            let cell = &mut buf[(x + dx, y + dy)];
            cell.set_symbol(" ");
            cell.set_style(style);
        }
    }
}
