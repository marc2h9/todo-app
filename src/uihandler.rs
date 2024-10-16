use ratatui::{
   prelude::*,
   widgets::*,
   Frame,
};

pub fn main(frame: &mut Frame) {
    
    // Top bar Layout
    let top_bar_layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(95),
        ])
        .split(frame.area());
    
    let top_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ])
        .split(top_bar_layout_vertical[0]);

    // Ticket Selection Menu Layout
    let ticket_select_bar_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ])
        .split(top_bar_layout_vertical[1]);

    // Widget rendering
    // Todo app top bar
    frame.render_widget(
        Paragraph::new("Todo App | Press \"Esc\" to quit")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
            ),
        top_bar_layout[0]
    );

    // Todo app side bar
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL),
        ticket_select_bar_horizontal[0]
    )
}
