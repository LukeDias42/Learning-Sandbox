use ratatui::style::{Color, Style};
pub struct Theme {
    pub key_binding: KeyBinding,
    pub logo: Style,
    pub timer: Timer,
    pub history: History,
    pub data: Data,
    pub town: Town,
}

pub struct KeyBinding {
    pub key: Style,
    pub description: Style,
    pub surrounding: Style,
    pub block: Style,
}

pub struct Timer {
    pub active: Style,
    pub inactive: Style,
    pub title: Style,
}

pub struct Data {
    pub break_bar: Style,
    pub focus_bar: Style,
    pub block: Style,
}
pub struct History {
    pub style: Style,
    pub border: Style,
}

pub struct Town {
    pub river: Style,
    pub grass: Style,
    pub city: Style,
    pub earth: Style,
    pub desert: Style,
}

pub const THEME: Theme = Theme {
    logo: Style::new().fg(EARTH_RED),
    key_binding: KeyBinding {
        key: Style::new().fg(EARTH_RED).bg(CHARCOAL_BROWN_DARK),
        description: Style::new().fg(EARTH_RED).bg(Color::Reset),
        surrounding: Style::new().fg(EARTH_RED),
        block: Style::new().fg(EARTH_RED),
    },
    timer: Timer {
        active: Style::new().bg(DARK_MOSS_GREEN).fg(EARTH_RED),
        inactive: Style::new().fg(EARTH_RED),
        title: Style::new().fg(EARTH_RED),
    },
    history: History {
        style: Style::new().fg(EARTH_RED),
        border: Style::new().fg(EARTH_RED),
    },
    data: Data {
        break_bar: Style::new().fg(LIGHT_RED),
        focus_bar: Style::new().fg(FOCUS_GREEN),
        block: Style::new().fg(Color::White),
    },
    town: Town {
        river: Style::new().fg(DEEP_BLUE).bg(Color::Cyan),
        grass: Style::new().bg(Color::LightGreen),
        city: Style::new().bg(Color::DarkGray),
        earth: Style::new().bg(CHARCOAL_BROWN),
        desert: Style::new().bg(LIGHT_SAND_COLOR),
    },
};

const CHARCOAL_BROWN: Color = Color::Rgb(60, 48, 42);
const CHARCOAL_BROWN_DARK: Color = Color::Rgb(40, 28, 22);
const EARTH_RED: Color = Color::Rgb(0xAD, 0x4B, 0x44);
const LIGHT_RED: Color = Color::Rgb(0xFF, 0x77, 0x77);
const FOCUS_GREEN: Color = Color::Rgb(0x17, 0xad, 0x52);
const DARK_MOSS_GREEN: Color = Color::Rgb(0x2D, 0x33, 0x30);
const LIGHT_SAND_COLOR: Color = Color::Rgb(255, 223, 186);
const DEEP_BLUE: Color = Color::Rgb(127, 130, 180);
