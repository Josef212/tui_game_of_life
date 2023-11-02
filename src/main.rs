mod player_state;
mod cell_state;
mod border_policy;
mod app_layout;
mod double_buffer_grid;
mod app;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::stdout;

use player_state::PlayerState;
use cell_state::CellState;
use app::App;

const MAX_LIFE_CYCLES: usize = 10;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let size = terminal.size()?.clone();

    let mut app = App::new(size);
    app.randomize_cells();

    let mut should_quit = false;
    while !should_quit {
        should_quit = handle_events(&mut app)?;
        logic_update(&mut app)?;
        terminal.draw(|frame| ui(&app, frame))?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn handle_events(app: &mut App) -> anyhow::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(match key.code {
                    KeyCode::Char('q') => true,
                    KeyCode::Char('r') => {
                        app.randomize_cells();
                        app.cycle_count = 0;
                        false
                    }
                    KeyCode::Char('p') => {
                        app.player_state.switch();
                        false
                    }
                    KeyCode::Char('b') => {
                        app.border_policy.switch();
                        false
                    }
                    _ => false,
                });
            }
        }
    }

    Ok(false)
}

fn logic_update(app: &mut App) -> anyhow::Result<()> {
    match app.player_state {
        PlayerState::Play => {}
        PlayerState::Pause => return Ok(()),
    };

    app.cycle_count += 1;
    app.grids.add_cycle();

    for y in 0..app.grid_height {
        for x in 0..app.grid_width {
            let index = (y * app.grid_width + x) as usize;
            let alive_neighbours = app.get_alive_neighbours_at_point(x, y);
            let cell = &app.grids.get_read_grid()[index].clone();

            let write = app.grids.get_write_grid();
            match cell {
                CellState::Dead => {
                    write[index] = if alive_neighbours == 3 {
                        CellState::Alive(0)
                    } else {
                        CellState::Dead
                    };
                }
                CellState::Alive(c) => {
                    write[index] = if alive_neighbours == 2 || alive_neighbours == 3 {
                        CellState::Alive(c + 1)
                    } else {
                        CellState::Dead
                    };
                }
            }
        }
    }

    Ok(())
}

fn ui<B: Backend>(
    app: &App,
    frame: &mut Frame<B>,
) {
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
