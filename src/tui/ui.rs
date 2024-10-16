use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
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
        .select(state.tab as usize)
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

    match state.tab {
        Tab::Data => {
            frame.render_widget(Paragraph::new("Data"), screen[1]);
        }
        Tab::Metadata => render_metadata(state, frame, screen[1]),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tab {
    Data = 0,
    Metadata = 1,
}

impl Default for Tab {
    fn default() -> Self {
        Self::Data
    }
}

impl Tab {
    const fn get_headers() -> &'static [&'static str] {
        &["data", "metadata"]
    }
}

impl From<usize> for Tab {
    fn from(v: usize) -> Self {
        match v {
            0 => Self::Data,
            1 => Self::Metadata,
            _ => Self::default(),
        }
    }
}

pub const N_TABS: usize = Tab::get_headers().len() - 1;

pub fn render_metadata(state: &mut State, frame: &mut Frame, rect: Rect) {
    let p = Paragraph::new(Text::from(
        state
            .viewer
            .arrow_schema()
            .fields()
            .iter()
            .map(|f| Line::from(f.name().to_string().fg(Color::Red)))
            .collect::<Vec<Line>>(),
    ));
    frame.render_widget(p, rect);
}
