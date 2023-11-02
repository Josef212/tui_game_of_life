use ratatui::prelude::Rect;

use crate::{
    app_layout::AppLayout, border_policy::BorderPolicy, cell_state::CellState,
    double_buffer_grid::DoubleBufferGrid, player_state::PlayerState,
};

pub struct App {
    pub grids: DoubleBufferGrid,
    pub grid_width: usize,
    pub grid_height: usize,
    pub cycle_count: usize,
    pub border_policy: BorderPolicy,
    pub layout: AppLayout,
    pub player_state: PlayerState,
    pub should_quit: bool,
}

impl App {
    pub fn new(terminal_rect: Rect) -> Self {
        let width = terminal_rect.width as usize;
        let height = terminal_rect.height as usize;

        let layout = AppLayout::generate(terminal_rect);
        let grids = DoubleBufferGrid::new(width, height);

        App {
            grids,
            grid_width: width,
            grid_height: height,
            cycle_count: 0,
            border_policy: BorderPolicy::Clamp,
            layout,
            player_state: PlayerState::Pause,
            should_quit: false,
        }
    }

    pub fn randomize_cells(&mut self) -> &mut Self {
        self.grids.randomize();
        self
    }

    pub fn get_alive_neighbours_at_point(&self, x: usize, y: usize) -> usize {
        self.grids
            .get_alive_neighbours_at_point(x, y, self.border_policy.clone())
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn tick(&mut self) {
        self.logic_update().expect("Failed to do logic update on tick");
    }

    pub fn logic_update(&mut self) -> anyhow::Result<()> {
        match self.player_state {
            PlayerState::Play => {}
            PlayerState::Pause => return Ok(()),
        };

        self.cycle_count += 1;
        self.grids.add_cycle();

        for y in 0..self.grid_height {
            for x in 0..self.grid_width {
                let index = (y * self.grid_width + x) as usize;
                let alive_neighbours = self.get_alive_neighbours_at_point(x, y);
                let cell = &self.grids.get_read_grid()[index].clone();

                let write = self.grids.get_write_grid();
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
}
