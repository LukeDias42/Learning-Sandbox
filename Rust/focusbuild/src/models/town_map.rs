use ratatui::style::Style;

use crate::application::theme::Theme;

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

impl Object {
    pub fn to_chars(self) -> String {
        match self {
            Object::River => "██",
            Object::House => "🏠",
            Object::HouseWithGarden => "🏡",
            Object::Office => "🏢",
            Object::Tree => "🌲",
            Object::Tree2 => "🌳",
            Object::Palm => "🌴",
            Object::Grass => "🌱",
            Object::Cactus => "🌵",
            Object::Road => "██",
            Object::None => "  ",
        }
        .into()
    }
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

impl Floor {
    pub fn to_style(self, theme: Theme) -> Style {
        match self {
            Floor::River => theme.town.river,
            Floor::Grass => theme.town.grass,
            Floor::City => theme.town.city,
            Floor::Earth => theme.town.earth,
            Floor::Desert => theme.town.desert,
        }
    }
}
