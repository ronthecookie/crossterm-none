extern crate crossterm;

use crossterm::{
    event,
    terminal,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    Result,
};

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    match event::read()? {
        Event::Key(k) => match k {
            KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE,
            } => println!("char: {}", ch),
            _ => {}
        },
        _ => {}
    }
    terminal::disable_raw_mode()
}
