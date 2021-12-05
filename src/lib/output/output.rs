use crossterm::{cursor, execute, queue};
use crossterm::event::KeyCode;
use crossterm::terminal::{self, ClearType};
use std::cmp;
use std::io::{Write, stdout};

use super::cursor_controller::CursorController;
use super::document_content::DocumentContent;
use super::editor_contents::EditorContents;
use super::terminal_groomer::TerminalGroomer;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct Output {
    pub win_size: (usize, usize),
    document_content: DocumentContent,
    editor_contents: EditorContents,
    cursor_controller: CursorController,
    // Sets raw mode and drops it
    _terminal_groomer: TerminalGroomer,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            document_content: DocumentContent::new(),
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
            _terminal_groomer: TerminalGroomer::new(),
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction)
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(
            self.editor_contents, 
            cursor::Hide,
            cursor::MoveTo(0, 0),
        )?;
        self.draw_rows();
        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        queue!(
            self.editor_contents, 
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show,
        )?;
        self.editor_contents.flush()
    }

    fn draw_welcome_message(&mut self) {
        let mut welcome = format!("Pound Editor --- Version {}", VERSION);
        let width = self.win_size.0;
        let len = welcome.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome = format!("~{}{}", spaces, welcome);
        welcome.truncate(width);
        self.editor_contents.push_str(&welcome)
    }

    fn draw_rows(&mut self) {
        if self.document_content.empty() {

        } else {
            let screen_columns = self.win_size.0;
            let num_rows = cmp::min(self.document_content.number_of_rows(), self.win_size.1);
            let rows = &self.document_content[0..num_rows];
            for row in rows {
                let len = cmp::min(row.len(), screen_columns);
                self.editor_contents.push_str(&row[..len]);
                queue!(self.editor_contents, terminal::Clear(ClearType::UntilNewLine)).unwrap();
                if row != rows.last().unwrap() {
                    self.editor_contents.push_str("\r\n")
                }

            }
        }

    }
/*     fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        for i in 0..screen_rows {
            if i >= self.document_content.number_of_rows() {
                if self.document_content.empty() && i == screen_rows / 3 {
                    self.draw_welcome_message()
                } else {
                    self.editor_contents.push('~');
                }
            } else {
                let len = cmp::min(self.document_content.get_row(i).len(), screen_columns);
                self.editor_contents.push_str(&self.document_content.get_row(i)[..len])
            }
            queue!(
                self.editor_contents,
                terminal::Clear(ClearType::UntilNewLine)
            ).unwrap();
            if i < screen_rows - 1 {
                self.editor_contents.push_str("\r\n");
            }
        }
    } */

    fn clear_screen(&mut self) -> crossterm::Result<()> {
        execute!(
            stdout(),
            terminal::Clear(ClearType::All)
        )?;
        execute!(
            stdout(),
            cursor::MoveTo(0, 0)
        )
    }
}

impl Drop for Output {
    fn drop(&mut self) {
        self.clear_screen().expect("Error");
    }
}