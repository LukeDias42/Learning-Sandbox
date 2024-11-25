use chrono::{Date, Duration, Local, NaiveDate};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{
    infra::repositories::focus_session_repository::FocusSessionRepository,
    models::focus_session::FocusSession,
};

use super::{
    app::{KeyPressResult, Mode, RemoveFromStack, Screen},
    theme::THEME,
};

pub struct Data {
    focus_sessions: Vec<FocusSession>,
    scroll_offset: usize,
    pub max_visible: usize,
}

struct DayTotalFocus {
    day: NaiveDate,
    break_seconds: u64,
    focus_seconds: u64,
}

impl Data {
    pub fn new() -> Result<Self> {
        let focus_sessions = FocusSessionRepository::new()?.select_many()?;
        let timer = Self {
            focus_sessions,
            scroll_offset: 0,
            max_visible: 1,
        };
        Ok(timer)
    }
    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(true))
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.scroll_offset -= if self.scroll_offset == 0 { 0 } else { 1 };
                KeyPressResult(Screen::History, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.scroll_offset +=
                    if self.scroll_offset == self.focus_sessions.len() - self.max_visible {
                        0
                    } else {
                        1
                    };
                KeyPressResult(Screen::History, Mode::Running, RemoveFromStack(false))
            }
            _ => KeyPressResult(Screen::History, Mode::Running, RemoveFromStack(false)),
        })
    }
    pub fn draw_entries(&self, area: Rect, buf: &mut Buffer) {
        Block::new()
            .title("statistics")
            .borders(Borders::ALL)
            .style(THEME.history.border)
            .render(area, buf);

        let visible_sessions = self
            .focus_sessions
            .iter()
            .skip(self.scroll_offset)
            .take(self.max_visible);
    }

    pub fn draw_keybinds(&self, area: Rect, buf: &mut Buffer) {
        let keys = [("Quit", "Q"), ("Up", "K/↑"), ("Down", "J/↓")];

        let spans: Vec<Span> = keys
            .iter()
            .flat_map(|(desc, key)| {
                let key = Span::styled(format!(" {key} "), THEME.key_binding.key);
                let desc = Span::styled(format!(" {desc} "), THEME.key_binding.description);
                [key, desc]
            })
            .collect();
        Line::from(spans)
            .centered()
            .style(THEME.key_binding.surrounding)
            .render(area, buf);
    }

    pub fn update_max_visible(&mut self, max_visible: usize) {
        self.max_visible = max_visible;
        if self.scroll_offset > self.focus_sessions.len() - self.max_visible {
            self.scroll_offset = self.focus_sessions.len() - self.max_visible;
        };
    }
}
impl Widget for &Data {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let graph_area = Rect::new(area.x, area.y + 1, area.width, area.height - 6);
        let keybinds_area = Rect::new(
            graph_area.x,
            graph_area.y + graph_area.height,
            graph_area.width,
            1,
        );
        self.draw_entries(graph_area, buf);
        self.draw_keybinds(keybinds_area, buf);
    }
}