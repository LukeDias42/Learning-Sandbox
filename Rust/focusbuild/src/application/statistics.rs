use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
};

use crate::{
    application::theme::THEME,
    infra::repositories::focus_session_repository::{self, FocusSessionRepository},
    models::focus_session,
};

use super::app::{KeyPressResult, Mode, RemoveFromStack, Screen};

pub struct Statistics {
    focus_session_repository: FocusSessionRepository,
}

impl Statistics {
    pub fn new() -> Result<Self> {
        let timer = Self {
            focus_session_repository: FocusSessionRepository::new()?,
        };
        Ok(timer)
    }
    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(true))
            }
            _ => KeyPressResult(Screen::Statistics, Mode::Running, RemoveFromStack(false)),
        })
    }
}

impl Widget for &Statistics {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let messages_area = Rect::new(area.x + 10, area.y + 5, area.width - 20, area.height - 10);
        let focus_sessions = match self.focus_session_repository.select_many() {
            Ok(focus_sessions) => focus_sessions,
            Err(e) => {
                format!("{:?}", e).render(area, buf);
                return;
            }
        };
        let mut y_offset = 0;
        for focus_session in focus_sessions.iter() {
            let date = focus_session.start.to_string();
            let focus_seconds = focus_session.focus_seconds;
            let break_seconds = focus_session.break_seconds;
            let id = focus_session.id;
            let content = Paragraph::new(format!("{date}, {focus_seconds}, {break_seconds}"))
                .block(Block::new().title("Session {id}").borders(Borders::ALL));

            let message_area = Rect::new(area.x + 10, area.y + 5 + y_offset, area.width - 20, 3);

            content.render(message_area, buf);

            y_offset += 4;
        }
    }
}
