pub mod app;
pub mod args;
pub mod error;
pub mod prelude;
pub mod tui;

use std::{fs::File, io};

use app::Viewer;
use args::Args;
use prelude::*;
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::{
    command::Command,
    event::{Event, EventHandler},
    state::State,
    Tui,
};

pub fn run(args: Args) -> Result<()> {
    let name = args
        .filename
        .file_stem()
        .map(|s| s.to_string_lossy().to_string());
    let file = File::open(args.filename)?;

    let viewer = Viewer::new(file, name)?;
    // dbg!(viewer.parquet_metadata());
    // dbg!(viewer.arrow_schema());
    // dbg!(viewer.file_metadata());
    // dbg!(viewer.parquet_schema());

    start_tui(viewer)
}

pub fn start_tui(viewer: Viewer) -> Result<()> {
    let mut state = State::new(viewer);

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;

    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while state.running {
        tui.draw(&mut state)?;

        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => {
                let command = Command::from(key_event);
                state.run_command(command, tui.events.sender.clone())?;
            }
            Event::Mouse(mouse_event) => {
                //
            }
            Event::Resize(_, _) => {}
        }
    }
    tui.exit()?;
    Ok(())
}
