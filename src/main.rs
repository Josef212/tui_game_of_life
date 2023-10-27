use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::stdout;

const MAX_LIFE_CYCLES: usize = 10;

#[derive(Clone, Debug)]
pub enum CellState {
    Alive(usize),
    Dead,
}

#[derive(Debug)]
pub enum PlayerState {
    Play,
    Pause,
}

impl PlayerState {
    pub fn switch(&mut self) {
        *self = match self {
            PlayerState::Pause => PlayerState::Play,
            PlayerState::Play => PlayerState::Pause,
        }
    }
}

pub struct AppLayout {
    main_layout: Rect,
    grid_panel: Rect,
    config_panel: Rect,
    console_panel: Rect,
    bottom_panel: Rect,
    grid_cell_width: usize,
    grid_cell_height: usize,
    grid_constraints: Vec<Constraint>,
}

impl AppLayout {
    pub fn generate(terminal_rect: Rect) -> Self {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(3)])
            .split(terminal_rect.inner(&Margin::new(1, 1)));

        let bottom_panel = main_layout[1];

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(main_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
            .split(main_layout[1]);

        let grid_width = Self::get_grid_width(&main_layout[0]);
        let grid_height = Self::get_grid_height(&main_layout[0]);
        let constrains = std::iter::repeat(Constraint::Length(2))
            .take(grid_width as usize)
            .collect::<Vec<Constraint>>();

        Self {
            main_layout: terminal_rect,
            grid_panel: main_layout[0],
            config_panel: right_layout[0],
            console_panel: right_layout[1],
            bottom_panel,
            grid_cell_width: grid_width,
            grid_cell_height: grid_height,
            grid_constraints: constrains,
        }
    }

    fn get_grid_width(grid_panel: &Rect) -> usize {
        ((grid_panel.width - 2) / 2) as usize
    }

    fn get_grid_height(grid_panel: &Rect) -> usize {
        (grid_panel.height - 2) as usize
    }
}

pub struct DoubleBufferGrid {
    grids: [Vec<CellState>; 2],
    cycle: usize,
    width: usize,
    height: usize,
}

impl DoubleBufferGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let grids = [
            std::iter::repeat(CellState::Dead).take(size).collect(),
            std::iter::repeat(CellState::Dead).take(size).collect(),
        ];

        Self {
            grids,
            cycle: 0,
            width,
            height,
        }
    }

    pub fn randomize(&mut self) {
        let size = self.width * self.height;
        for i in 0..size {
            let state = if rand::random() {CellState::Alive(0)} else {CellState::Dead};
            self.grids[0][i] = state.clone();
            self.grids[1][i] = state.clone();
        }
    }

    pub fn add_cycle(&mut self) {
        self.cycle += 1;
    }

    pub fn get_read_grid(&self) -> &Vec<CellState> {
        let read_grid_index = self.cycle % 2;
        &self.grids[read_grid_index]
    }

    pub fn get_write_grid(&mut self) -> &mut Vec<CellState> {
        let write_grid_index = (self.cycle + 1) % 2;
        &mut self.grids[write_grid_index]
    }

    pub fn get_render_grid(&self) -> &Vec<CellState> {
        let write_grid_index = (self.cycle + 1) % 2;
        &self.grids[write_grid_index]
    }

    pub fn get_alive_neighbours_at_point(&self, x: usize, y: usize) -> usize {
        let mut indices = Vec::with_capacity(8);
        for yy in (-1 as i32)..2 {
            for xx in (-1 as i32)..2 {
                if xx == 0 && yy == 0 {
                    continue;
                }

                let new_x = x as i32 + xx;
                let new_y = y as i32 + yy;

                if new_x < 0
                    || new_x >= self.width as i32
                    || new_y < 0
                    || new_y >= self.height as i32
                {
                    continue;
                }

                let index = (new_y as usize) * self.width + (new_x as usize);
                indices.push(index);
            }
        }

        indices
            .iter()
            .map(|i| &self.get_read_grid()[*i])
            .filter(|cs| match *cs {
                CellState::Alive(_) => true,
                CellState::Dead => false,
            })
            .count()
    }
}

pub struct App {
    grids: DoubleBufferGrid,
    grid_width: usize,
    grid_height: usize,
    cycle_count: usize,
    skip_ratio: u16,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        let grids = DoubleBufferGrid::new(width, height);

        App {
            grids,
            grid_width: width,
            grid_height: height,
            cycle_count: 0,
            skip_ratio: 1,
        }
    }

    pub fn randomize_cells(&mut self) -> &mut Self {
        self.grids.randomize();
        self
    }

    pub fn get_alive_neighbours_at_point(&self, x: usize, y: usize) -> usize {
        self.grids.get_alive_neighbours_at_point(x, y)
    }
}

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let size = terminal.size()?;
    let mut player_state = PlayerState::Pause;
    let app_layout = AppLayout::generate(size);
    let mut app = App::new(app_layout.grid_cell_width, app_layout.grid_cell_height);
    app.randomize_cells();


    let mut should_quit = false;
    while !should_quit {
        should_quit = handle_events(&mut app, &mut player_state)?;
        logic_update(&mut app, &player_state)?;
        terminal.draw(|frame| ui(&app, &app_layout, &player_state, frame))?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn handle_events(app: &mut App, player_state: &mut PlayerState) -> anyhow::Result<bool> {
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
                    KeyCode::Char('k') => {
                        if app.skip_ratio > 1 {
                            app.skip_ratio -= 1;
                        }
                        false
                    }
                    KeyCode::Char('j') => {
                        app.skip_ratio += 1;
                        false
                    }
                    KeyCode::Char('p') => {
                        player_state.switch();
                        false
                    }
                    _ => false,
                });
            }
        }
    }

    Ok(false)
}

fn logic_update(app: &mut App, player_state: &PlayerState) -> anyhow::Result<()> {
    match player_state {
        PlayerState::Play => {}
        PlayerState::Pause => return Ok(()),
    };

    app.cycle_count += 1;

    if app.cycle_count % app.skip_ratio as usize != 0 {
        return Ok(());
    }

    app.grids.add_cycle();

    for y in 0..app.grid_height {
        for x in 0..app.grid_width {
            let index = (y * app.grid_width + x) as usize;
            let alive_neighbours = app.grids.get_alive_neighbours_at_point(x, y);
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
    app_layout: &AppLayout,
    player_state: &PlayerState,
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
                    let dc = 1.0 - dc;
                    let col = (dc * 255.0) as u8;
                    let col = Color::Rgb(255-col, col, col);

                    // Cell::from("  ").bg(Color::White).fg(Color::Black)
                    Cell::from("  ").bg(col).fg(Color::Black)
                },
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

    frame.render_widget(block, app_layout.main_layout);

    let table = Table::new(cells)
        .block(Block::default().borders(Borders::ALL))
        .widths(&app_layout.grid_constraints)
        .column_spacing(0);

    frame.render_widget(table, app_layout.grid_panel);

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
            Span::raw("Player state: "),
            Span::raw(format!("{:?}", player_state)),
        ]),
    ]);
    let text = Paragraph::new(text).block(block);

    frame.render_widget(text, app_layout.config_panel);

    let block = Block::new()
        .title("Console")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    frame.render_widget(block, app_layout.console_panel);

    let block = Block::new()
        .title("Cheatsheat")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let shortcuts = Text::from("Q: quit  R: reset  P: play/pause");
    let shortcuts = Paragraph::new(shortcuts).block(block);
    frame.render_widget(shortcuts, app_layout.bottom_panel);
}
