use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::stdout;

#[derive(Clone, Debug)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Clone)]
pub struct App {
    cells: Vec<CellState>,
    grid_width: u32,
    grid_height: u32,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        let size = width * height;
        let grid = std::iter::repeat(CellState::Dead)
            .take(size as usize)
            .collect::<Vec<CellState>>();

        App {
            cells: grid,
            grid_width: width,
            grid_height: height,
        }
    }

    pub fn print_cells(&self) {
        for y in 0..self.grid_height {
            let mut line = String::new();
            for x in 0..self.grid_width {
                let index = y * self.grid_width + x;
                let cell = &self.cells[index as usize];
                line += &format!("{:?} ", cell);
            }

            println!("{}", line);
        }
    }

    pub fn randomize_cells(&mut self) -> &mut Self {
        self.cells.iter_mut().for_each(|c| {
            *c = if rand::random() {
                CellState::Alive
            } else {
                CellState::Dead
            }
        });
        self
    }

    pub fn get_alive_neighbours_at_point(&self, x: u32, y: u32) -> usize {
        let mut indices = Vec::with_capacity(8);
        for yy in (-1 as i32)..2 {
            for xx in (-1 as i32)..2 {
                if xx == 0 && yy == 0 {
                    continue;
                }

                let new_x = x as i32 + xx;
                let new_y = y as i32 + yy;

                if new_x < 0
                    || new_x >= self.grid_width as i32
                    || new_y < 0
                    || new_y >= self.grid_height as i32
                {
                    continue;
                }

                let index = (new_y as u32) * self.grid_width + (new_x as u32);
                indices.push(index as usize);
            }
        }

        indices
            .iter()
            .map(|i| &self.cells[*i])
            .filter(|cs| match *cs {
                CellState::Alive => true,
                CellState::Dead => false,
            })
            .count()
    }
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;

    let size = terminal.size()?;
    let grid_width = ((size.width - 2) / 2) as u32;
    let grid_height = (size.height - 2) as u32;

    let mut app = App::new(grid_width, grid_height);
    app.randomize_cells();

    while !should_quit {
        should_quit = handle_events()?;
        logic_update(&mut app)?;
        terminal.draw(|frame| ui(&app, frame))?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn handle_events() -> anyhow::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn logic_update(app: &mut App) -> anyhow::Result<()> {
    // TODO: Worst way to do this. Add a second vector to the App and have a double buffer swap
    let copy = app.clone();

    for y in 0..app.grid_height {
        for x in 0..app.grid_width {
            let index = (y * app.grid_width + x) as usize;
            let alive_neighbours = copy.get_alive_neighbours_at_point(x, y);
            let cell = &copy.cells[index];

            match cell {
                CellState::Dead => {
                    app.cells[index] = if alive_neighbours == 3 {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    };
                }
                CellState::Alive => {
                    app.cells[index] = if alive_neighbours == 2 || alive_neighbours == 3 {
                        CellState::Alive
                    } else {
                        CellState::Dead
                    };
                }
            }
        }
    }

    Ok(())
}

fn ui<B: Backend>(app: &App, frame: &mut Frame<B>) {
    let mut cells = Vec::with_capacity((app.grid_width * app.grid_height) as usize);
    for y in 0..app.grid_height {
        let mut row = Vec::new();

        for x in 0..app.grid_width {
            let index = y * app.grid_width + x;
            let cell = match &app.cells[index as usize] {
                // CellState::Alive => Cell::from("██").bg(Color::Black).fg(Color::White),
                CellState::Alive => Cell::from("  ").bg(Color::White).fg(Color::Black),
                CellState::Dead => Cell::from(" ").bg(Color::Reset).fg(Color::White),
            };

            row.push(cell);
        }

        let row = Row::new(row);
        cells.push(row);
    }

    let mut constrains = Vec::new();
    for _ in 0..app.grid_width {
        constrains.push(Constraint::Length(2));
    }

    let table = Table::new(cells)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .widths(&constrains)
        .column_spacing(0);

    frame.render_widget(table, frame.size());
}
