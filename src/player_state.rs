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
        };
    }
}
