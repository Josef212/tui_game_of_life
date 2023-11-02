use ratatui::prelude::Rect;

use crate::{double_buffer_grid::DoubleBufferGrid, border_policy::BorderPolicy, app_layout::AppLayout, player_state::PlayerState};

pub struct App {
    pub grids: DoubleBufferGrid,
    pub grid_width: usize,
    pub grid_height: usize,
    pub cycle_count: usize,
    pub border_policy: BorderPolicy,
    pub layout: AppLayout,
    pub player_state: PlayerState,
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
}
