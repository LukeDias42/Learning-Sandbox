#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Floor {
    #[default]
    Grass,
    City,
    Earth,
    Desert,
    River,
}

