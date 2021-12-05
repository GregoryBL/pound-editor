use crossterm::terminal;

pub struct TerminalGroomer;

impl TerminalGroomer {
    pub fn new() -> Self {
        terminal::enable_raw_mode().expect("Could not enter raw mode");
        Self
    }
}

impl Drop for TerminalGroomer {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}

