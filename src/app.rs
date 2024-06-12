use std::{collections::vec_deque::Iter, path::Path};

use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{self, Constraint, Direction, Layout},
    prelude::Rect,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{
    action::Action,
    components::{fps::FpsCounter, home::Home, Component},
    config::Config,
    mode::Mode,
    trace_dbg, tui,
};

struct AppComponents {
    header: Box<dyn Component>,
    footer: Box<dyn Component>,
}

impl Component for AppComponents {
    fn register_action_handler(&mut self, tx: mpsc::UnboundedSender<Action>) -> Result<()> {
        self.header.register_action_handler(tx.clone())?;
        self.footer.register_action_handler(tx)?;
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.header.register_config_handler(config.clone())?;
        self.footer.register_config_handler(config)?;
        Ok(())
    }

    fn init(&mut self, area: Rect) -> Result<()> {
        self.header.init(area)?;
        self.footer.init(area)?;
        Ok(())
    }

    fn handle_events(&mut self, event: Option<tui::Event>) -> Result<()> {
        self.header.handle_events(event.clone())?;
        self.footer.handle_events(event.clone())?;
        Ok(())
    }

    fn draw(&mut self, f: &mut tui::Frame<'_>, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(area);
        self.header.draw(f, layout[0]);
        self.footer.draw(f, layout[2]);
    }

    fn update(&mut self, action: Action) -> Result<()> {
        self.header.update(action.clone())?;
        self.footer.update(action.clone())?;
        Ok(())
    }

    // fn send(&self, action: Action) -> Result<()> {
    //     Ok(())
    // }
}

pub struct App {
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    // pub components: Vec<Box<dyn Component>>,
    pub components: AppComponents,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub last_tick_key_events: Vec<KeyEvent>,
    pub filepath: String,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64, path: &Path) -> Result<Self> {
        let home = Home::new();
        let fps = FpsCounter::default();
        let config = Config::new()?;
        let mode = Mode::Home;
        let filepath = path.display().to_string();
        let components = AppComponents {
            header: Box::new(home),
            footer: Box::new(fps),
        };
        Ok(Self {
            tick_rate,
            frame_rate,
            // components: vec![Box::new(home), Box::new(fps)],
            components,
            should_quit: false,
            should_suspend: false,
            config,
            mode,
            last_tick_key_events: Vec::new(),
            filepath,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut tui = tui::Tui::new()?
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        // tui.mouse(true);
        tui.enter()?;

        // let components = [self.components.header, self.components.footer];

        self.components.register_action_handler(action_tx.clone())?;
        // self.header.register_action_handler(action_tx.clone())?;
        // self.footer.register_action_handler(action_tx.clone())?;
        // for component in self.components.iter_mut() {
        //     component.register_action_handler(action_tx.clone())?;
        // }

        self.components
            .register_config_handler(self.config.clone())?;
        // for component in self.components.iter_mut() {
        // for component in self.components.iter_mut() {
        //     component.register_config_handler(self.config.clone())?;
        // }

        self.components.init(tui.size()?)?;
        // for component in self.components.iter_mut() {
        // for component in self.components.iter_mut() {
        //     component.init(tui.size()?)?;
        // }

        loop {
            if let Some(e) = tui.next().await {
                match e {
                    tui::Event::Quit => action_tx.send(Action::Quit)?,
                    tui::Event::Tick => action_tx.send(Action::Tick)?,
                    tui::Event::Render => action_tx.send(Action::Render)?,
                    tui::Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
                    tui::Event::Key(key) => {
                        if let Some(keymap) = self.config.keybindings.get(&self.mode) {
                            if let Some(action) = keymap.get(&vec![key]) {
                                log::info!("Got action: {action:?}");
                                action_tx.send(action.clone())?;
                            } else {
                                // If the key was not handled as a single key action,
                                // then consider it for multi-key combinations.
                                self.last_tick_key_events.push(key);

                                // Check for multi-key combinations
                                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                                    log::info!("Got action: {action:?}");
                                    action_tx.send(action.clone())?;
                                }
                            }
                        };
                    }
                    _ => {}
                }
                // for component in self.components.iter_mut() {
                self.components.handle_events(Some(e.clone()));
                // for component in self.components.iter_mut() {
                //     if let Some(action) = component.handle_events(Some(e.clone()))? {
                //         action_tx.send(action)?;
                //     }
                // }
            }

            while let Ok(action) = action_rx.try_recv() {
                if action != Action::Tick && action != Action::Render {
                    log::debug!("{action:?}");
                }
                match action {
                    Action::Tick => {
                        self.last_tick_key_events.drain(..);
                    }
                    Action::Quit => self.should_quit = true,
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    Action::Resize(w, h) => {
                        tui.resize(Rect::new(0, 0, w, h))?;
                        tui.draw(|f| {
                            self.components.draw(f, f.size());
                            // for component in self.components.iter_mut() {
                            // let r = component.draw(f, f.size());
                            // if let Err(e) = r {
                            //     component
                            //         .action_tx
                            //         .send(Action::Error(format!("Failed to draw: {:?}", e)))
                            //         .unwrap();
                            // }
                            // component.draw(f, f.size());
                            // }
                        })?;
                    }
                    Action::Render => {
                        tui.draw(|f| {
                            self.components.draw(f, f.size());
                            // for component in self.components.iter_mut() {
                            // let r = component.draw(f, f.size());
                            // if let Err(e) = r {
                            //     action_tx
                            //         .send(Action::Error(format!("Failed to draw: {:?}", e)))
                            //         .unwrap();
                            // }
                            // component.draw(f, f.size());
                            // }
                        })?;
                    }
                    _ => {}
                }
                self.components.update(action.clone())?
                // for component in self.components.iter_mut() {
                //     if let Some(action) = component.update(action.clone())? {
                //         action_tx.send(action)?
                //     };
                // }
            }
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                tui = tui::Tui::new()?
                    .tick_rate(self.tick_rate)
                    .frame_rate(self.frame_rate);
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }
}
