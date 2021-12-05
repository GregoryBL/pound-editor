use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use super::output::Output;
use super::reader::Reader;

pub struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            reader: Reader, 
            output: Output::new(),
        }
    }

    pub fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }

    fn process_keypress(&mut self) -> crossterm::Result<bool> {
        let event = self.reader.read_key()?;
        match event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            KeyEvent {
                code: 
                    val 
                    @ 
                    (KeyCode::Up 
                    | KeyCode::Down 
                    | KeyCode::Left 
                    | KeyCode::Right
                    | KeyCode::Home
                    | KeyCode::End
                ),
                modifiers: KeyModifiers::NONE,
            } => self.output.move_cursor(val),
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
            } => (0..self.output.win_size.1).for_each(|_| {
                self.output.move_cursor(if matches!(val, KeyCode::PageUp) {
                    KeyCode::Up
                } else {
                    KeyCode::Down
                });
            }),
            _ => { 
                // todo 
            }
        }
        Ok(true)
    }
}
