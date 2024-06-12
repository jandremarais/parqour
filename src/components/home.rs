use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
    action::Action,
    config::{Config, KeyBindings},
    trace_dbg,
};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<()> {
        let r = match action {
            Action::Tick => Ok(()),
            Action::Move => Err("Failed"),
            _ => Ok(()),
        };
        if let Err(v) = r {
            self.send(Action::Quit)?;
        }
        Ok(())
    }

    // fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) {
        f.render_widget(Paragraph::new("Parqour"), area);
        // Ok(())
    }

    fn send(&self, action: Action) -> Result<()> {
        if let Some(tx) = self.command_tx.clone() {
            tx.send(action)?
        }
        Ok(())
    }
}
