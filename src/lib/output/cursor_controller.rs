use crossterm::event::{KeyCode};

/* #[derive(Copy, Clone)]
enum CursorDirection {
    Left,
    Right,
    Up,
    Down,
}
 */

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    screen_columns: usize,
    screen_rows: usize,
}

impl CursorController {
    pub fn new(win_size: (usize, usize)) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Left => {
                self.cursor_x = self.cursor_x.saturating_sub(1);
            },
            KeyCode::Right => {
                if self.cursor_x < self.screen_columns {
                    self.cursor_x += 1
                }
            },
            KeyCode::Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            },
            KeyCode::Down => {
                if self.cursor_y < self.screen_rows {
                    self.cursor_y += 1
                }
            },
            KeyCode::End => self.cursor_x = self.screen_columns - 1,
            KeyCode::Home => self.cursor_x = 0,
            _ => unimplemented!()
        }
    }
}

