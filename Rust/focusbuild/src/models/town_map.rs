#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TownMap {
    pub map: Vec<Vec<Tile>>,
    pub name: String,
}

impl TownMap {
    pub fn new(map: Vec<Vec<Tile>>, name: String) -> Self {
        Self { map, name }
    }
}

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

