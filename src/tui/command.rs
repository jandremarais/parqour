use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Exit,
    Nothing,
}

impl From<KeyEvent> for Command {
    fn from(key_event: KeyEvent) -> Self {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Self::Exit,
            _ => Self::Nothing,
        }
    }
}
