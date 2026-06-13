#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Key {
    J,
    L,
    H,
}

impl Key {
    pub fn label(self) -> &'static str {
        match self {
            Self::J => "J",
            Self::L => "L",
            Self::H => "H",
        }
    }
}
