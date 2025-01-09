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
        Self::clear_screen()
    }
    pub fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }
    pub fn draw_rows() -> Result<(), Error> {
        let number_of_rows = size()?.1;
        let mut stdout = stdout();
        for i in 0..number_of_rows {
            // MoveTo must be executed otherwise it does nothing, it is just a struct
            execute!(stdout, MoveTo(0, i))?;
            print!("~");
        }
        execute!(stdout, MoveTo(0, 0))?;
        Ok(())
    }
}
