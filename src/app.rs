use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::Rect, text::Text, widgets::Widget, Frame};

use crate::tui::{Action, Event, Tui};

#[derive(Debug, Default)]
pub struct App {
    should_quit: bool,
}

impl App {
    pub async fn run(&mut self) -> color_eyre::Result<()> {
        let mut tui = Tui::new()?.tick_rate(4.0).frame_rate(60.0);

        tui.enter()?;

        loop {
            tui.draw(|f| {
                self.ui(f);
            })?;

            if let Some(evt) = tui.next().await {
                let mut maybe_action = self.handle_event(evt);
                while let Some(action) = maybe_action {
                    maybe_action = self.update(action);
                }
            };

            if self.should_quit {
                break;
            }
        }
        tui.exit()?;

        Ok(())
    }

    fn ui(&self, frame: &mut Frame) {
        let area = frame.size();
        frame.render_widget(self, area);
    }

    fn handle_event(&self, event: Event) -> Option<Action> {
        match event {
            Event::Quit => Some(Action::Exit),
            Event::Key(key_event) => self.handle_key_event(key_event),
            _ => None,
        }
    }

    fn handle_key_event(&self, key_event: KeyEvent) -> Option<Action> {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => Some(Action::Exit),
            _ => None,
        }
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Exit => {
                self.should_quit = true;
                None
            }
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Text::from("Parqour").render(area, buf);
    }
}
