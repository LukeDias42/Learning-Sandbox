use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Position, Rect},
    text::Text,
    widgets::{Block, Borders},
    Frame,
};

use crate::{
    application::theme::THEME,
    models::town_map::{Object, Tile, TownMap},
};

use super::app::{KeyPressResult, Mode, RemoveFromStack, Screen};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Town {
    pub town_map: TownMap,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub vertical_offset: u16,
    pub horizontal_offset: u16,
}

impl Town {
    pub fn new() -> Result<Self> {
        let map = vec![vec![Tile::default(); 80]; 40];
        let city = TownMap::new(map, "Inland Town".to_string());
        Ok(Town {
            town_map: city,
            cursor_x: 0,
            cursor_y: 0,
            vertical_offset: 0,
            horizontal_offset: 0,
        })
    }

    pub fn handle_key_press(&mut self, key: KeyEvent) -> Result<KeyPressResult> {
        Ok(match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                KeyPressResult(Screen::MainMenu, Mode::Running, RemoveFromStack(true))
            }
            KeyCode::Up => {
                self.vertical_offset -= 1;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Down => {
                self.vertical_offset += 1;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Left => {
                self.horizontal_offset -= 1;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Right => {
                self.horizontal_offset += 1;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('k') => {
                self.cursor_y -= 1;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('j') => {
                self.cursor_y += 1;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('h') => {
                self.cursor_x -= 1;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('l') => {
                self.cursor_x += 1;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('a') => {
                self.town_map.map[self.cursor_y as usize + self.vertical_offset as usize]
                    [self.cursor_x as usize + self.horizontal_offset as usize]
                    .object = Object::House;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('o') => {
                self.town_map.map[self.cursor_y as usize + self.vertical_offset as usize]
                    [self.cursor_x as usize + self.horizontal_offset as usize]
                    .object = Object::Office;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('x') => {
                self.town_map.map[self.cursor_y as usize + self.vertical_offset as usize]
                    [self.cursor_x as usize + self.horizontal_offset as usize]
                    .object = Object::None;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            KeyCode::Char('r') => {
                self.town_map.map[self.cursor_y as usize + self.vertical_offset as usize]
                    [self.cursor_x as usize + self.horizontal_offset as usize]
                    .object = Object::River;
                KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false))
            }
            _ => KeyPressResult(Screen::Town, Mode::Running, RemoveFromStack(false)),
        })
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        let map_rows = self.town_map.map.len() as u16;
        let map_columns = self.town_map.map[0].len() as u16;
        let block_area = Rect::new(area.x, area.y, map_columns * 2 + 2, map_rows + 2);
        frame.render_widget(
            Block::new()
                .title(self.town_map.name.clone())
                .borders(Borders::ALL)
                .style(THEME.logo),
            block_area,
        );
        let max_horz_tiles = ((area.width - 2) / 2) as usize;
        let max_vert_tiles = (area.height - 2) as usize;
        self.town_map
            .map
            .iter()
            .skip(self.vertical_offset as usize)
            .take(max_vert_tiles)
            .enumerate()
            .for_each(|(y, tile_row)| {
                tile_row
                    .iter()
                    .skip(self.horizontal_offset as usize)
                    .take(max_horz_tiles)
                    .enumerate()
                    .for_each(|(x, tile)| {
                        let tile_area =
                            Rect::new(area.x + 1 + 2 * x as u16, area.y + 1 + y as u16, 2, 1);
                        frame.render_widget(
                            Text::raw(tile.object.to_chars()).style(tile.floor.to_style()),
                            tile_area,
                        );
                    });
            });
        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            block_area.x + self.cursor_x * 2 + 1,
            // Move one line down, from the border to the input line
            block_area.y + self.cursor_y + 1,
        ))
    }
}
