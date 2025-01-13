use std::io::{stdout, Error, Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};

pub struct Terminal {}

pub struct Size {
    pub height: u16,
    pub width: u16,
}

pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(&Position { x: 0, y: 0 })?;
        Self::execute()
    }
    pub fn clear_screen() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::All))
    }
    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))
    }
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()
    }
    pub fn move_cursor_to(position: &Position) -> Result<(), Error> {
        // MoveTo must be executed otherwise it does nothing, it is just a struct
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }
    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), Error> {
        queue!(stdout(), Print(string))?;
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { height, width })
    }
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}
