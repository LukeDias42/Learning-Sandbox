use ratatui::style::{Color, Style};

pub struct Theme {
    pub root: Style,
    pub borders: Style,
    pub key_binding: KeyBinding,
    pub logo: Style,
    pub timer: Timer,
    pub statistics: Statistics,
}

pub struct KeyBinding {
    pub key: Style,
    pub description: Style,
}

pub struct Timer {
    pub active: Style,
    pub inactive: Style,
    pub title: Style,
}

pub struct Statistics {
    pub style: Style,
    pub title: Style,
}

pub const THEME: Theme = Theme {
    root: Style::new().bg(CHARCOAL_BROWN),
    borders: Style::new().fg(LIGHT_GRAY),
    logo: Style::new().fg(MOSS_GREEN),
    key_binding: KeyBinding {
        key: Style::new().fg(BLACK).bg(DARK_GRAY),
        description: Style::new().fg(DARK_GRAY).bg(BLACK),
    },
    timer: Timer {
        active: Style::new().bg(DARK_BLUE).fg(MOSS_GREEN),
        inactive: Style::new().bg(LIGHT_RED).fg(MOSS_GREEN),
        title: Style::new().fg(MOSS_GREEN),
    },
    statistics: Statistics {
        style: Style::new().bg(DARK_BLUE).fg(LIGHT_RED),
        title: Style::new().fg(MOSS_GREEN),
    },
};

pub const CHARCOAL_BROWN: Color = Color::Rgb(60, 48, 42);
pub const MOSS_GREEN: Color = Color::Rgb(138, 154, 91);
pub const LIGHT_GRAY: Color = Color::Rgb(0xC2, 0xC2, 0xC2);
pub const LIGHT_RED: Color = Color::Rgb(0xFF, 0xC2, 0xC2);
// pub const LIGHT_GREEN: Color = Color::Rgb(0xC2, 0xFF, 0xC2);
// pub const LIGHT_BLUE: Color = Color::Rgb(0xC2, 0xC2, 0xFF);
pub const DARK_GRAY: Color = Color::Rgb(0x22, 0x22, 0x22);
pub const BLACK: Color = Color::Rgb(0x0, 0x0, 0x0);
// pub const WHITE: Color = Color::Rgb(0xFF, 0xFF, 0xFF);
pub const DARK_BLUE: Color = Color::Rgb(0x42, 0x42, 0xFF);
