use crate::{double_buffer_grid::DoubleBufferGrid, border_policy::BorderPolicy};

pub struct App {
    pub grids: DoubleBufferGrid,
    pub grid_width: usize,
    pub grid_height: usize,
    pub cycle_count: usize,
    pub skip_ratio: u16,
    pub border_policy: BorderPolicy,
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
            border_policy: BorderPolicy::Clamp,
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
