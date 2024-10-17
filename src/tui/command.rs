use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Exit,
    Nothing,
    Next(ScrollType),
    Previous(ScrollType),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScrollType {
    Tab,
    Table,
    Chunk,
}

impl From<KeyEvent> for Command {
    fn from(key_event: KeyEvent) -> Self {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => Self::Exit,
            KeyCode::BackTab => Self::Previous(ScrollType::Tab),
            KeyCode::Tab => Self::Next(ScrollType::Tab),
            KeyCode::Char('j') | KeyCode::Down => Self::Next(ScrollType::Table),
            KeyCode::Char('k') | KeyCode::Up => Self::Previous(ScrollType::Table),
            KeyCode::Char('l') | KeyCode::Right => Self::Next(ScrollType::Chunk),
            KeyCode::Char('h') | KeyCode::Left => Self::Previous(ScrollType::Chunk),
            _ => Self::Nothing,
        }
    }
}
