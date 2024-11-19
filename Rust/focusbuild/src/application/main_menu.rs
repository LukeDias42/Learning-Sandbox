use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};

use crate::application::theme::THEME;

use super::app::{KeyPressResult, Mode, RemoveFromStack, Screen};

const LOGO_WIDTH: u16 = 73;
const LOGO_HEIGHT: u16 = 6;
const POMO_BUILD_LOGO: &str =
    "██████╗  ██████╗ ███╗   ███╗ ██████╗ ██████╗ ██╗   ██╗██╗██╗     ██████╗ 
██╔══██╗██╔═══██╗████╗ ████║██╔═══██╗██╔══██╗██║   ██║██║██║     ██╔══██╗
██████╔╝██║   ██║██╔████╔██║██║   ██║██████╔╝██║   ██║██║██║     ██║  ██║
██╔═══╝ ██║   ██║██║╚██╔╝██║██║   ██║██╔══██╗██║   ██║██║██║     ██║  ██║
██║     ╚██████╔╝██║ ╚═╝ ██║╚██████╔╝██████╔╝╚██████╔╝██║███████╗██████╔╝
╚═╝      ╚═════╝ ╚═╝     ╚═╝ ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝╚══════╝╚═════╝ 
";

const SMALL_LOGO_WIDTH: u16 = 38;
const SMALL_LOGO_HEIGHT: u16 = 4;
const SMALL_POMO_BUILD_LOGO: &str = " _ __                __          _    
' )  )              /  )        //   /
 /--'__ ______  __ /--<  . . o // __/ 
/   (_)/ / / <_(_)/___/_(_/_<_</_(_/_ ";

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
            KeyCode::Char('q') | KeyCode::Esc => {
                KeyPressResult(Screen::None, Mode::Quit, RemoveFromStack(true))
            }
            KeyCode::Char('b') => {
                KeyPressResult(Screen::Timer, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('s') => {
                KeyPressResult(Screen::Statistics, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('t') => {
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            _ => KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(false)),
        })
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
        let area = Rect::new(
            3 * area.width / 8,
            area.height / 4,
            area.width / 4,
            area.height / 2,
        );
        let options = Paragraph::new(
            "[B]egin
[T]own
[S]tatistics
[Q]uit",
        )
        .block(
            Block::new()
                .borders(Borders::ALL)
                .padding(Padding::symmetric(area.width / 12, area.height / 6)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .style(THEME.key_binding.key);
        options.render(area, buf);
    }
}
