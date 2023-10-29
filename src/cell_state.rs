#[derive(Clone, Debug)]
pub enum CellState {
    Alive(usize),
    Dead,
}
