#[derive(Debug, Clone, PartialEq)]
pub enum BorderPolicy {
    Clamp,
    Wrap,
}

impl BorderPolicy {
    pub fn switch(&mut self) {
        *self = match self {
            Self::Clamp => Self::Wrap,
            Self::Wrap => Self::Clamp,
        };
    }
}
