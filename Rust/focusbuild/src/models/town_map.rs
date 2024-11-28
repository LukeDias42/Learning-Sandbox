#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    #[default]
    None,
    River,
    House,
    HouseWithGarden,
    Office,
    Tree,
    Tree2,
    Palm,
    Grass,
    Cactus,
    Road,
}

pub enum Floor {
    #[default]
    Grass,
    City,
    Earth,
    Desert,
    River,
}

