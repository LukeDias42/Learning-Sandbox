use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Styled,
    text::Text,
    widgets::{Block, Borders, Widget},
};

use crate::application::theme::THEME;

use super::app::{KeyPressResult, Mode, RemoveFromStack, Screen};

const LOGO_WIDTH: u16 = 73;
const LOGO_HEIGHT: u16 = 6;
const POMO_BUILD_LOGO: &str =
    "███████╗ ██████╗  ██████╗██╗   ██╗███████╗██████╗ ██╗   ██╗██╗██╗     ██████╗ 
██╔════╝██╔═══██╗██╔════╝██║   ██║██╔════╝██╔══██╗██║   ██║██║██║     ██╔══██╗
█████╗  ██║   ██║██║     ██║   ██║███████╗██████╔╝██║   ██║██║██║     ██║  ██║
██╔══╝  ██║   ██║██║     ██║   ██║╚════██║██╔══██╗██║   ██║██║██║     ██║  ██║
██║     ╚██████╔╝╚██████╗╚██████╔╝███████║██████╔╝╚██████╔╝██║███████╗██████╔╝
╚═╝      ╚═════╝  ╚═════╝ ╚═════╝ ╚══════╝╚═════╝  ╚═════╝ ╚═╝╚══════╝╚═════╝ 
                                                                              ";

const SMALL_LOGO_WIDTH: u16 = 38;
const SMALL_LOGO_HEIGHT: u16 = 4;
const SMALL_POMO_BUILD_LOGO: &str = "   _____              __          _    
    /  '             /  )        //   /
 ,-/-,__ _. . . _   /--<  . . o // __/ 
(_/  (_)(__(_/_/_)_/___/_(_/_<_</_(_/_ 
                                       ";

const KEYBIND_STRINGS: [(&str, &str); 5] = [
    (
        " ▗▄▄▖
▐▌   
 ▝▀▚▖
▗▄▄▞▘",
        " ▗▄▄▖▗▄▄▄▖▗▄▖ ▗▄▄▖▗▄▄▄▖
▐▌     █ ▐▌ ▐▌▐▌ ▐▌ █  
 ▝▀▚▖  █ ▐▛▀▜▌▐▛▀▚▖ █  
▗▄▄▞▘  █ ▐▌ ▐▌▐▌ ▐▌ █  ",
    ),
    (
        "▗▖ ▗▖
▐▌ ▐▌
▐▛▀▜▌
▐▌ ▐▌",
        "▗▖ ▗▖▗▄▄▄▖ ▗▄▄▖▗▄▄▄▖▗▄▖ ▗▄▄▖▗▖  ▗▖
▐▌ ▐▌  █  ▐▌     █ ▐▌ ▐▌▐▌ ▐▌▝▚▞▘ 
▐▛▀▜▌  █   ▝▀▚▖  █ ▐▌ ▐▌▐▛▀▚▖ ▐▌  
▐▌ ▐▌▗▄█▄▖▗▄▄▞▘  █ ▝▚▄▞▘▐▌ ▐▌ ▐▌  ",
    ),
    (
        "▗▄▄▄ 
▐▌  █
▐▌  █
▐▙▄▄▀",
        "▗▄▄▄  ▗▄▖▗▄▄▄▖▗▄▖ 
▐▌  █▐▌ ▐▌ █ ▐▌ ▐▌
▐▌  █▐▛▀▜▌ █ ▐▛▀▜▌
▐▙▄▄▀▐▌ ▐▌ █ ▐▌ ▐▌",
    ),
    (
        "▗▄▄▄▖
  █  
  █  
  █  ",
        "▗▄▄▄▖▗▄▖ ▗▖ ▗▖▗▖  ▗▖
  █ ▐▌ ▐▌▐▌ ▐▌▐▛▚▖▐▌
  █ ▐▌ ▐▌▐▌ ▐▌▐▌ ▝▜▌
  █ ▝▚▄▞▘▐▙█▟▌▐▌  ▐▌",
    ),
    (
        "▗▄▄▄▖ 
▐▌ ▐▌ 
▐▌ ▐▌ 
▐▙▄▟▙▖",
        "▗▄▄▄▖ ▗▖ ▗▖▗▄▄▄▖▗▄▄▄▖
▐▌ ▐▌ ▐▌ ▐▌  █    █  
▐▌ ▐▌ ▐▌ ▐▌  █    █  
▐▙▄▟▙▖▝▚▄▞▘▗▄█▄▖  █  ",
    ),
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MainMenu {}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {}
    }
}

impl MainMenu {
    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                KeyPressResult(Screen::Timer, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                KeyPressResult(Screen::History, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                KeyPressResult(Screen::None, Mode::Quit, RemoveFromStack(true))
            }
            _ => KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(false)),
        })
    }
    pub fn draw_keybinds(&self, area: Rect, buf: &mut Buffer) {
        for (i, (key, desc)) in KEYBIND_STRINGS.iter().enumerate() {
            let mut key_area = area.clone();
            key_area.y += 5 * i as u16;

            Text::raw("")
                .style(THEME.key_binding.key)
                .render(key_area, buf);
            key_area.x += 1;
            Text::raw(key.to_string())
                .style(THEME.key_binding.key)
                .render(key_area, buf);
            key_area.x += 1;
            Text::raw("")
                .style(THEME.key_binding.key)
                .render(key_area, buf);
            key_area.x += 5;

            Text::raw("")
                .style(THEME.key_binding.description)
                .render(key_area, buf);
            key_area.x += 2;
            Text::raw(desc.to_string())
                .style(THEME.key_binding.description)
                .render(key_area, buf);
        }
    }
}

impl Widget for MainMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width / 2 > LOGO_WIDTH && area.height / 2 > LOGO_HEIGHT {
            let title_area = Rect::new(
                area.width / 4 + (area.width / 4 - LOGO_WIDTH / 2),
                area.height / 4 - LOGO_HEIGHT,
                area.width / 2,
                area.height / 2,
            );
            Text::raw(POMO_BUILD_LOGO)
                .style(THEME.logo)
                .render(title_area, buf);
        } else if area.width / 2 > SMALL_LOGO_HEIGHT && area.height / 2 > SMALL_LOGO_HEIGHT {
            let title_area = Rect::new(
                area.width / 4 + (area.width / 4 - SMALL_LOGO_WIDTH / 2),
                area.height / 4 - SMALL_LOGO_HEIGHT,
                area.width / 2,
                area.height / 2,
            );
            Text::raw(SMALL_POMO_BUILD_LOGO)
                .style(THEME.logo)
                .render(title_area, buf);
        }
        let keybind_block_area = Rect::new((area.width - 45) / 2, area.height / 4, 45, 27);
        Block::new()
            .borders(Borders::all())
            .style(THEME.key_binding.block)
            .render(keybind_block_area, buf);
        let keybind_area = Rect::new(keybind_block_area.x + 1, keybind_block_area.y + 1, 35, 5);
        self.draw_keybinds(keybind_area, buf);
    }
}
