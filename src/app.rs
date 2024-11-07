use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Layout},
    prelude::Rect,
    widgets::Paragraph,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{debug, info};

use crate::{
    action::Action,
    components::*,
    config::Config,
    tui::{Event, Tui},
};

pub struct App {
    config: Config,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    last_tick_key_events: Vec<KeyEvent>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
    software: SoftwareList,
    projects: ProjectList,
    has_focus: usize,
    focusable_max: usize,
    main_area: Box<dyn Component>,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Home,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64) -> Result<Self> {
        let (action_tx, action_rx) = mpsc::unbounded_channel();
        Ok(Self {
            should_quit: false,
            should_suspend: false,
            config: Config::new()?,
            mode: Mode::Home,
            last_tick_key_events: Vec::new(),
            action_tx,
            action_rx,
            software: SoftwareList::default(),
            projects: ProjectList::default(),
            has_focus: 0,
            focusable_max: 2, // Software and Projects
            main_area: Box::new(Fishtank::default()),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = Tui::new()?;
        // .mouse(true) // uncomment this line to enable mouse support
        // .tick_rate(self.tick_rate)
        // .frame_rate(self.frame_rate);
        tui.enter()?;

        /*
        for component in self.components.iter_mut() {
            component.register_action_handler(self.action_tx.clone())?;
        }
        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }
        for component in self.components.iter_mut() {
            component.init(tui.size()?)?;
        } */

        self.software
            .register_action_handler(self.action_tx.clone())?;
        self.software.register_config_handler(self.config.clone())?;
        self.software.init(tui.size()?)?;

        let action_tx = self.action_tx.clone();
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;
            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
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

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }

        /*
        for component in self.components.iter_mut() {
            if let Some(action) = component.handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        } */
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        let action_tx = self.action_tx.clone();
        let Some(keymap) = self.config.keybindings.get(&self.mode) else {
            return Ok(());
        };
        match keymap.get(&vec![key]) {
            Some(action) => {
                info!("Got action: {action:?}");

                action_tx.send(action.clone())?;
            }
            _ => {
                // If the key was not handled as a single key action,
                // then consider it for multi-key combinations.
                self.last_tick_key_events.push(key);

                // Check for multi-key combinations
                if let Some(action) = keymap.get(&self.last_tick_key_events) {
                    info!("Got action: {action:?}");
                    action_tx.send(action.clone())?;
                }
            }
        }
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            if action != Action::Tick && action != Action::Render {
                debug!("{action:?}");
            }
            match action {
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                Action::Quit => self.should_quit = true,
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                Action::NextPane => {
                    self.has_focus = (self.has_focus + 1) % self.focusable_max;
                    self.software.has_focus = self.has_focus == 0;
                    self.projects.has_focus = self.has_focus == 1;

                    // Unfocus all
                    self.software.unfocus();
                    self.projects.unfocus();

                    match self.has_focus {
                        0 => self.software.focus(),
                        1 => self.projects.focus(),
                        _ => {}
                    }
                }
                _ => {}
            }

            /*
            for component in self.components.iter_mut() {
                if let Some(action) = component.update(action.clone())? {
                    self.action_tx.send(action)?
                };
            } */
        }
        Ok(())
    }

    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|frame| {
            let [header_area, main_area, footer_area] = Layout::vertical([
                Constraint::Length(2),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .areas(frame.area());

            let [list_area, item_area] =
                Layout::horizontal([Constraint::Fill(1), Constraint::Fill(2)]).areas(main_area);

            // Render
            frame.render_widget(
                Paragraph::new("Fishtank: Manage your Nanopore Sequencing and Basecalling              <*((((><        <*((((><          <*((((><"),
                header_area,
            );

            // Add key combos to the footer
            // Up/Down - Navigate, Enter - Select, q - Quit, Tab - Next Pane

            let bottom_footer = Paragraph::new("Author: Joseph Guhlin -- [q] Quit | [Tab] Next Pane | [Up/Down] Navigate | [Enter] Select | [`] Configuration");
            frame.render_widget(bottom_footer, footer_area);


            // Left hand column (Projects + Software)
            let [projects_area, software_area] =
                Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                    .areas(list_area);

            self.projects
                .draw(frame, projects_area)
                .expect("Failed to draw projects list");

            self.software
                .draw(frame, software_area)
                .expect("Failed to draw software list");

            // frame.render_widget(Paragraph::new("item"), item_area);
            self.main_area
                .draw(frame, item_area)
                .expect("Failed to draw main area");

            /*
            for component in self.components.iter_mut() {
                if let Err(err) = component.draw(frame, frame.area()) {
                    let _ = self
                        .action_tx
                        .send(Action::Error(format!("Failed to draw: {:?}", err)));
                }
            } */
        })?;
        Ok(())
    }
}
