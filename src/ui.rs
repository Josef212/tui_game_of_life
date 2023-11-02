use ratatui::{
    backend::Backend,
    style::Color,
    widgets::*,
    prelude::*,
    Frame,
};

use crate::{app::App, cell_state::CellState};

const MAX_LIFE_CYCLES: usize = 10;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let mut cells = Vec::with_capacity((app.grid_width * app.grid_height) as usize);
    let read = app.grids.get_render_grid();
    for y in 0..app.grid_height {
        let mut row = Vec::new();

        for x in 0..app.grid_width {
            let index = y * app.grid_width + x;
            let cell = match &read[index as usize] {
                // CellState::Alive(_) => Cell::from("██").bg(Color::Black).fg(Color::White),
                CellState::Alive(c) => {
                    let c = std::cmp::min(*c, MAX_LIFE_CYCLES);
                    let dc = c as f64 / MAX_LIFE_CYCLES as f64;
                    // let dc = 1.0 - dc;
                    let col = (dc * 255.0) as u8;
                    let col = Color::Rgb(255 - col, col, col);

                    // Cell::from("  ").bg(Color::White).fg(Color::Black)
                    Cell::from("  ").bg(col).fg(Color::Black)
                }
                // CellState::Dead => Cell::from(" ").bg(Color::Black).fg(Color::White),
                CellState::Dead => Cell::from(" ").bg(Color::Reset).fg(Color::White),
            };

            row.push(cell);
        }

        let row = Row::new(row);
        cells.push(row);
    }

    let block = Block::new()
        .title("Game of life")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(block, app.layout.main_layout);

    let table = Table::new(cells)
        .block(Block::default().borders(Borders::ALL))
        .widths(&app.layout.grid_constraints)
        .column_spacing(0);

    frame.render_widget(table, app.layout.grid_panel);

    let block = Block::new()
        .title("Config")
        .padding(Padding::new(1, 0, 1, 0))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let text = Text::from(vec![
        Line::from(vec![
            Span::raw("Cycle count: "),
            Span::raw(app.cycle_count.to_string()),
        ]),
        Line::from(vec![
            Span::raw("Grid size: "),
            Span::raw(format!("{}x{}", app.layout.width(), app.layout.height())),
        ]),
        Line::from(vec![
            Span::raw("Player state: "),
            Span::raw(format!("{:?}", app.player_state)),
        ]),
        Line::from(vec![
            Span::raw("Border policy: "),
            Span::raw(format!("{:?}", app.border_policy)),
        ]),
    ]);
    let text = Paragraph::new(text).block(block);

    frame.render_widget(text, app.layout.config_panel);

    let block = Block::new()
        .title("Console")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(block, app.layout.console_panel);

    let block = Block::new()
        .title("Cheatsheat")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let shortcuts = Text::from("Q: quit  R: reset  P: play/pause  B: switch border policy");
    let shortcuts = Paragraph::new(shortcuts).block(block);
    frame.render_widget(shortcuts, app.layout.bottom_panel);
}
