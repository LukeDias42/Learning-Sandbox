#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub object: Object,
    pub floor: Floor,
}

impl Tile {
    pub fn new(object: Object, floor: Floor) -> Self {
        Tile { object, floor }
    }
}

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Floor {
    #[default]
    Grass,
    City,
    Earth,
    Desert,
    River,
}

