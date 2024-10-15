use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    text::Line,
    widgets::{Block, Paragraph}
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| {
            frame.render_widget(
                Paragraph::new("Hello, World!").block(Block::bordered().title("Press Q to quit")),
                frame.area()
            );
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
