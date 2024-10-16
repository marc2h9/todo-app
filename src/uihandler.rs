use ratatui::{
   prelude::*,
   widgets::*,
   Frame,
};

pub fn main(frame: &mut Frame) {
    
    // Bar Layout
    let bar_layout_vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ])
        .split(frame.area());
   
    // Top bar
    let top_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ])
        .split(bar_layout_vertical[0]);

    // Bottom bar
    let bottom_bar_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ])
        .split(bar_layout_vertical[2]);

    // Ticket Selection Bar Layout
    let ticket_select_bar_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ])
        .split(bar_layout_vertical[1]);

    // Ticket viewer layout
    let ticket_view_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ])
        .split(frame.area());




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

    // Todo app bottom bar
    frame.render_widget(
        Paragraph::new("Press \"C\" for new ticket")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
            ),
        bottom_bar_layout[0]
    );

    // Todo app side bar
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL),
        ticket_select_bar_horizontal[0]
    );

    // Todo viewer
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL),
        ticket_view_layout[1]
    );
}

pub fn exit_confirmation(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Are you sure you want to quit \"Y\" | \"N\"")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
            ),
        frame.area()
    )
}
