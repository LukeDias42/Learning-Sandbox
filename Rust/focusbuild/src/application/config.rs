use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Widget},
};

use crate::models::settings::Settings;

use super::{
    app::{KeyPressResult, Mode, RemoveFromStack, Screen},
    config_font::ConfigFont,
    theme::Theme,
};

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub struct Config {
    pub settings: Settings,
    pub saved: bool,
    settings_row: u8,
}

impl Config {
    pub fn new(settings: Settings) -> Self {
        Config {
            settings,
            saved: false,
            settings_row: 0,
        }
    }

    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                if self.settings_row < 4 {
                    self.settings_row += 1;
                }
                KeyPressResult(Screen::Config, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                if self.settings_row > 0 {
                    self.settings_row -= 1;
                }
                KeyPressResult(Screen::Config, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                if self.settings_row == 0 {
                    self.settings.theme = self.settings.theme.next();
                } else if self.settings_row == 0 {
                    self.settings.language = self.settings.language.next();
                } else if self.settings_row == 0 {
                    self.settings.font_size = self.settings.font_size.next();
                } else {
                    self.settings.focus_break_proportion += 1;
                }
                KeyPressResult(Screen::Config, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                if self.settings_row == 0 {
                    self.settings.theme = self.settings.theme.prev();
                } else if self.settings_row == 0 {
                    self.settings.language = self.settings.language.prev();
                } else if self.settings_row == 0 {
                    self.settings.font_size = self.settings.font_size.prev();
                } else {
                    self.settings.focus_break_proportion += 1;
                }
                KeyPressResult(Screen::Config, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(true))
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.saved = true;
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(true))
            }
            _ => KeyPressResult(Screen::Config, Mode::Running, RemoveFromStack(false)),
        })
    }
    pub fn draw_settings(&self, area: Rect, buf: &mut Buffer, theme: Theme) {
        let config_font = ConfigFont::new(area.width, area.height, self.settings);
        let x = config_font.logo.offset
            + (area.width as i16 / 4 + (area.width as i16 / 2 - config_font.logo.width as i16) / 2)
                as u16;
        let logo_area = Rect::new(
            x,
            area.height / 4 - config_font.logo.height,
            config_font.logo.width,
            config_font.logo.height,
        );
        Text::raw(config_font.logo.text)
            .style(theme.logo)
            .render(logo_area, buf);
        let largest_line =
            config_font.keybinding.longest_description + config_font.keybinding.width + 2;
        let x = (area.width as i16 / 4 + (area.width as i16 / 2 - largest_line as i16) / 2) as u16;
        let keybind_block_area = Rect::new(
            x,
            area.height / 4,
            largest_line + 4,
            config_font.keybinding.total_height + 2,
        );
        Block::new()
            .borders(Borders::all())
            .style(theme.key_binding.block)
            .render(keybind_block_area, buf);
    }
}

impl Widget for Config {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = Theme::new(self.settings.theme);

        self.draw_settings(area, buf, theme);
    }
}
