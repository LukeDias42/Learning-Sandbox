use ratatui::style::{Color, Style};
pub struct Theme {
    pub root: Style,
    pub key_binding: KeyBinding,
    pub logo: Style,
    pub timer: Timer,
    pub history: History,
}

pub struct KeyBinding {
    pub key: Style,
    pub description: Style,
    pub surrounding: Style,
}

pub struct Timer {
    pub active: Style,
    pub inactive: Style,
    pub title: Style,
}

pub struct History {
    pub style: Style,
    pub border: Style,
}

pub const THEME: Theme = Theme {
    root: Style::new().bg(CHARCOAL_BROWN),
    logo: Style::new().fg(MOSS_GREEN),
    key_binding: KeyBinding {
        key: Style::new().fg(MOSS_GREEN).bg(CHARCOAL_BROWN_DARK),
        description: Style::new().fg(MOSS_GREEN).bg(CHARCOAL_BROWN_LIGHT),
        surrounding: Style::new().fg(MOSS_GREEN).bg(CHARCOAL_BROWN_LIGHT),
    },
    timer: Timer {
        active: Style::new().bg(LIGHT_GREEN).fg(MOSS_GREEN),
        inactive: Style::new().bg(CHARCOAL_BROWN).fg(MOSS_GREEN),
        title: Style::new().fg(MOSS_GREEN),
    },
    history: History {
        style: Style::new().bg(CHARCOAL_BROWN).fg(MOSS_GREEN),
        border: Style::new().fg(MOSS_GREEN),
    },
};

pub const CHARCOAL_BROWN: Color = Color::Rgb(60, 48, 42);
pub const CHARCOAL_BROWN_DARK: Color = Color::Rgb(40, 28, 22);
pub const CHARCOAL_BROWN_LIGHT: Color = Color::Rgb(80, 68, 62);
pub const MOSS_GREEN: Color = Color::Rgb(138, 154, 91);
// pub const LIGHT_GRAY: Color = Color::Rgb(0xC2, 0xC2, 0xC2);
// pub const LIGHT_RED: Color = Color::Rgb(0xFF, 0xC2, 0xC2);
pub const LIGHT_GREEN: Color = Color::Rgb(0xC2, 0xFF, 0xC2);
// pub const LIGHT_BLUE: Color = Color::Rgb(0xC2, 0xC2, 0xFF);
// pub const DARK_GRAY: Color = Color::Rgb(0x22, 0x22, 0x22);
// pub const BLACK: Color = Color::Rgb(0x0, 0x0, 0x0);
// pub const WHITE: Color = Color::Rgb(0xFF, 0xFF, 0xFF);
// pub const DARK_BLUE: Color = Color::Rgb(0x42, 0x42, 0xFF);
