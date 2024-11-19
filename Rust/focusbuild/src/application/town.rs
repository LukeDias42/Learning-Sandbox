use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, text::Text, widgets::Widget};

use crate::application::theme::THEME;

use super::app::{KeyPressResult, Mode, RemoveFromStack, Screen};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Town {}

impl Town {
    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(true))
            }
            _ => KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false)),
        })
    }
}

impl Widget for Town {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Text::raw("Town")
            .style(THEME.timer.active)
            .render(area, buf);
    }
}
