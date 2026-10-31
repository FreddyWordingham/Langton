#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum SpawnTab {
    Terrain,
    Feature,
    Construct,
    Item,
}

impl SpawnTab {
    pub(crate) const ALL: [SpawnTab; 4] = [Self::Terrain, Self::Feature, Self::Construct, Self::Item];

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Terrain => "Terrain",
            Self::Feature => "Feature",
            Self::Construct => "Construct",
            Self::Item => "Item",
        }
    }
}
