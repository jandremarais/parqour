use std::sync::mpsc;

use crate::app::Viewer;

use super::{command::Command, event::Event, Result};

pub struct State {
    pub running: bool,
    pub viewer: Viewer,
}

impl State {
    pub fn new(viewer: Viewer) -> Self {
        Self {
            running: true,
            viewer,
        }
    }

    pub fn run_command(
        &mut self,
        command: Command,
        event_sender: mpsc::Sender<Event>,
    ) -> Result<()> {
        match command {
            Command::Exit => {
                self.running = false;
            }
            Command::Nothing => {}
        }
        Ok(())
    }
}
