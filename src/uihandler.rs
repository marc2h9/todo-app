use ratatui::{
   Frame,
   widgets::{Block, Paragraph}
};

pub fn draw_test(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Hello, World!").block(Block::bordered().title("Press Q to quit")),
        frame.area()
    );
}
