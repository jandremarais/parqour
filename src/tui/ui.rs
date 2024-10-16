use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Tabs},
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

    let tabs = Tabs::new(vec!["data", "metadata"])
        .highlight_style(Style::default().yellow())
        .select(state.tab)
        .divider("|")
        .padding(" ", " ");

    frame.render_widget(tabs, title_line[0]);
    let label = Paragraph::new(Line::from(vec![
        Span::from(" parqour ")
            .bg(Color::LightMagenta)
            .fg(Color::DarkGray),
        Span::from(format!(" {} ", state.viewer.filename()))
            .bg(Color::LightCyan)
            .fg(Color::DarkGray),
    ]))
    .right_aligned();
    frame.render_widget(label, title_line[1]);
}
