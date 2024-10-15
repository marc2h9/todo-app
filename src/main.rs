pub mod uihandler;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {
            uihandler::main(frame);
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind ==  KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    ratatui::restore();
    Ok(())
}
