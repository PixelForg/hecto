use crossterm::{
    event::{read, Event::Key, KeyCode::Char},
    terminal::enable_raw_mode,
};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(err) => println!("Error:{err}"),
                _ => (),
            }
        }
    }
}
