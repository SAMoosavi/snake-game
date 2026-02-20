#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameSpeed {
    Slow,
    #[default]
    Normal,
    Fast,
}

impl GameSpeed {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Slow => "Slow",
            Self::Normal => "Normal",
            Self::Fast => "Fast",
        }
    }

    pub fn tick_millis(&self) -> u64 {
        match self {
            Self::Slow => 120,
            Self::Normal => 80,
            Self::Fast => 50,
        }
    }

    pub fn score_multiplier(&self) -> u16 {
        match self {
            Self::Slow => 1,
            Self::Normal => 2,
            Self::Fast => 3,
        }
    }

    pub fn faster(&self) -> Self {
        match self {
            Self::Slow => Self::Normal,
            Self::Normal => Self::Fast,
            Self::Fast => Self::Fast,
        }
    }

    pub fn slower(&self) -> Self {
        match self {
            Self::Slow => Self::Slow,
            Self::Normal => Self::Slow,
            Self::Fast => Self::Normal,
        }
    }
}
