use ratatui::{
   prelude::*,
   widgets::*,
   Frame,
};

pub fn main(frame: &mut Frame) {
    
    // Top bar icon such as name and how to quit
    let outer_layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());

    let top_bar_layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(90),
        ])
        .split(outer_layout_vertical[0]);
    
    let top_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(10),
            Constraint::Percentage(85),
        ])
        .split(top_bar_layout_vertical[0]);

    frame.render_widget(
        Paragraph::new("Todo App")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
            ),
        top_bar_layout[0]

    );

    frame.render_widget(
        Paragraph::new("Press \"Esc\" to quit")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
            ),
        top_bar_layout[1]
    );

    // Ticket Selection Menu 
    let ticket_select_bar_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(95),
        ])
        .split(frame.area());

    let ticket_select_bar_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ])
        .split(ticket_select_bar_vertical[1]);

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL),
        ticket_select_bar_horizontal[0]
    );
}
