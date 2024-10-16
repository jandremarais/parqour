use std::sync::mpsc;

use crate::app::Viewer;

use super::{
    command::Command,
    event::Event,
    ui::{Tab, N_TABS},
    Result,
};

pub struct State {
    pub running: bool,
    pub viewer: Viewer,
    pub tab: Tab,
}

impl State {
    pub fn new(viewer: Viewer) -> Self {
        Self {
            running: true,
            viewer,
            tab: Default::default(),
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
                let new_tab = self.tab as usize + 1;
                if new_tab > N_TABS {
                    self.tab = 0.into();
                } else {
                    self.tab = new_tab.into();
                }
            }
            Command::Previous => {
                self.tab = (self.tab as usize).checked_sub(1).unwrap_or(N_TABS).into();
            }
            Command::Nothing => {}
        }
        Ok(())
    }
}
