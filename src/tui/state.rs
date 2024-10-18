use std::sync::mpsc;

use ratatui::widgets::TableState;

use crate::app::Viewer;

use super::{
    command::{Command, ScrollType},
    event::Event,
    ui::{Tab, N_TABS},
    Result,
};

pub struct State {
    pub running: bool,
    pub viewer: Viewer,
    pub tab: Tab,
    pub table_state: TableState,
    pub data_table_state: TableState,
    pub chunk_ind: usize,
}

impl State {
    pub fn new(viewer: Viewer) -> Self {
        Self {
            running: true,
            viewer,
            tab: Default::default(),
            table_state: TableState::default().with_selected(Some(0)),
            data_table_state: TableState::default().with_selected(Some(0)),
            chunk_ind: 0,
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
            Command::Next(scroll_type) => match scroll_type {
                ScrollType::Tab => {
                    let new_tab = self.tab as usize + 1;
                    if new_tab > N_TABS {
                        self.tab = 0.into();
                    } else {
                        self.tab = new_tab.into();
                    }
                }
                ScrollType::Vertical => match self.tab {
                    Tab::Metadata => {
                        if let Some(selection) = self.table_state.selected_mut() {
                            *selection = (*selection + 1) % self.viewer.num_cols;
                        }
                    }
                    Tab::Data => {
                        if self.viewer.selected_row == (self.viewer.batch_size - 1) {
                            // TODO: what to do when reached end of batch?
                            // read in next batch but might need to be in separate thread?
                            self.viewer.batch = self.viewer.reader.next().unwrap()?;
                            // self.viewer.reader.
                        } else {
                            self.viewer.selected_row += 1;
                            if self.viewer.selected_row
                                > (self.viewer.row_offset + self.viewer.visible_rows - 1)
                            {
                                self.viewer.row_offset += 1;
                            }
                        }
                    }
                },
                ScrollType::Horizontal => match self.tab {
                    Tab::Metadata => {
                        self.chunk_ind = (self.chunk_ind + 1) % self.viewer.num_row_groups;
                    }
                    Tab::Data => {
                        if self.viewer.selected_col == (self.viewer.num_cols - 1) {
                            self.viewer.selected_col = 0;
                            self.viewer.col_offset = 0;
                        } else {
                            self.viewer.selected_col += 1;
                            if self.viewer.selected_col
                                > (self.viewer.col_offset + self.viewer.visible_cols - 1)
                            {
                                self.viewer.col_offset += 1;
                            }
                        }
                    }
                },
            },
            Command::Previous(scroll_type) => match scroll_type {
                ScrollType::Tab => {
                    self.tab = (self.tab as usize).checked_sub(1).unwrap_or(N_TABS).into();
                }
                ScrollType::Vertical => match self.tab {
                    Tab::Metadata => {
                        if let Some(selection) = self.table_state.selected_mut() {
                            *selection = if *selection == 0 {
                                self.viewer.num_cols - 1
                            } else {
                                (*selection - 1) % self.viewer.num_cols
                            }
                        }
                    }

                    Tab::Data => {
                        if self.viewer.selected_row != 0 {
                            self.viewer.selected_row -= 1;
                            if self.viewer.selected_row < self.viewer.row_offset {
                                self.viewer.row_offset -= 1;
                            }
                        }
                    }
                },
                ScrollType::Horizontal => match self.tab {
                    Tab::Metadata => {
                        self.chunk_ind = if self.chunk_ind == 0 {
                            self.viewer.num_row_groups - 1
                        } else {
                            (self.chunk_ind - 1) % self.viewer.num_row_groups
                        }
                    }
                    Tab::Data => {
                        if self.viewer.selected_col != 0 {
                            self.viewer.selected_col -= 1;
                            if self.viewer.selected_col < self.viewer.col_offset {
                                self.viewer.col_offset -= 1;
                            }
                        } else {
                            self.viewer.selected_col = self.viewer.num_cols - 1;
                            self.viewer.col_offset =
                                self.viewer.num_cols - self.viewer.visible_cols;
                        }
                    }
                },
            },
            Command::Nothing => {}
        }
        Ok(())
    }
}
