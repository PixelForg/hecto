use std::io::{stdout, Error};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};

pub(crate) struct Terminal {}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)
    }
    pub fn clear_screen() -> Result<(), Error> {
        execute!(stdout(), Clear(ClearType::All))
    }
    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }
    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), Error> {
        // MoveTo must be executed otherwise it does nothing, it is just a struct
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }
    pub fn size() -> Result<(u16, u16), Error> {
        size()
    }
}
