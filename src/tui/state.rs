use std::sync::mpsc;

use crate::app::Viewer;

use super::{command::Command, event::Event, Result};

pub struct State {
    pub running: bool,
    pub viewer: Viewer,
    pub tab: usize,
}

impl State {
    pub fn new(viewer: Viewer) -> Self {
        Self {
            running: true,
            viewer,
            tab: 0,
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
            Command::Next => {
                let new_tab = self.tab + 1;
                if new_tab > 1 {
                    self.tab = 0;
                } else {
                    self.tab = new_tab;
                }
            }
            Command::Previous => {
                self.tab = self.tab.checked_sub(1).unwrap_or(1);
            }
            Command::Nothing => {}
        }
        Ok(())
    }
}
