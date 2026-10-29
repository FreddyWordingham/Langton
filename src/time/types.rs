#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TimeSpeed {
    Stop,
    Slow,
    Normal,
    Fast,
    Fastest,
}

impl TimeSpeed {
    pub(super) const ALL: [Self; 5] = [Self::Stop, Self::Slow, Self::Normal, Self::Fast, Self::Fastest];

    pub(super) fn as_str(&self) -> &'static str {
        match self {
            Self::Stop => "x0",
            Self::Slow => "x2",
            Self::Normal => "x4",
            Self::Fast => "x8",
            Self::Fastest => "x16",
        }
    }

    pub(super) fn tps(&self) -> Option<f64> {
        match self {
            Self::Stop => None,
            Self::Slow => Some(10.0),
            Self::Normal => Some(20.0),
            Self::Fast => Some(40.0),
            Self::Fastest => Some(80.0),
        }
    }
}
