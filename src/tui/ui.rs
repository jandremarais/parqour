use ratatui::{widgets::Paragraph, Frame};

use super::state::State;

pub fn render(state: &mut State, frame: &mut Frame) {
    let tmp = Paragraph::new(format!("PARQOUR!!: {}", state.viewer.filename()));
    frame.render_widget(tmp, frame.area());
}
