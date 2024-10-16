use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::state::State;

pub fn render(state: &mut State, frame: &mut Frame) {
    let screen = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Min(1)])
        .split(frame.area());

    let title_line = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(screen[0]);

    frame.render_widget(
        Paragraph::new(Text::styled(
            " parqour ",
            Style::default().bg(Color::LightCyan).fg(Color::DarkGray),
        )),
        title_line[0],
    );
    frame.render_widget(
        Paragraph::new(Text::styled(
            format!(" {} ", state.viewer.filename()),
            Style::default().bg(Color::LightCyan).fg(Color::DarkGray),
        ))
        .right_aligned(),
        title_line[1],
    );
}
