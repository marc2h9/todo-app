use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    widgets::{Block, Paragraph}
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init()?;
}
