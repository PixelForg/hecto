mod terminal;
use std::io::Error;
use terminal::Terminal;

use crossterm::event::{
    read,
    Event::{self, Key},
    KeyCode::Char,
    KeyEvent, KeyModifiers,
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    fn draw_rows() -> Result<(), Error> {
        let number_of_rows = Terminal::size()?.1;
        for current_row in 0..number_of_rows {
            print!("~");
            if current_row + 1 < number_of_rows {
                print!("\r\n");
            }
        }
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye. \r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }
}
