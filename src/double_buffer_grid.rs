use crate::{cell_state::CellState, border_policy::BorderPolicy};

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
            let state = if rand::random() {
                CellState::Alive(0)
            } else {
                CellState::Dead
            };
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

    pub fn get_alive_neighbours_at_point(&self, x: usize, y: usize, policy: BorderPolicy) -> usize {
        let mut indices = Vec::with_capacity(8);
        for yy in (-1 as i32)..2 {
            for xx in (-1 as i32)..2 {
                if xx == 0 && yy == 0 {
                    continue;
                }

                let mut new_x = x as i32 + xx;
                let mut new_y = y as i32 + yy;

                if policy == BorderPolicy::Clamp {
                    if new_x < 0
                        || new_x >= self.width as i32
                        || new_y < 0
                        || new_y >= self.height as i32
                    {
                        continue;
                    }
                } else if policy == BorderPolicy::Wrap {
                    // Would be great to use modulus but it's not working with negative numbers 🙃
                    // new_x = new_x % (self.width as i32);
                    // new_y = new_y % (self.height as i32);
                    if new_x < 0 {
                        new_x = (self.width - 1) as i32;
                    }
                    if new_x >= self.width as i32 {
                        new_x = 0;
                    }
                    if new_y < 0 {
                        new_y = (self.height - 1) as i32;
                    }
                    if new_y >= self.height as i32 {
                        new_y = 0;
                    }
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
