use std::time::Instant;

use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};
use serde::{Deserialize, Serialize};

use super::Component;

use crate::action::Action;
use crate::components::utils::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SoftwareList {
    pub items: Vec<String>, // todo - replace with software struct
    pub has_focus: bool,
}

impl Focusable for SoftwareList {
    fn focus(&mut self) {
        self.has_focus = true;
    }

    fn unfocus(&mut self) {
        self.has_focus = false;
    }
}

impl Default for SoftwareList {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareList {
    pub fn new() -> Self {
        Self {
            items: vec![
                "Dorado v0.8.1".to_string(),
                "Bonito v0.1.0".to_string(),
                "Guppy v0.2.0".to_string(),
            ],
            has_focus: false,
        }
    }
}

impl Component for SoftwareList {
    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // Convert self.items to "ListItems"
        let list_items = self
            .items
            .iter()
            .map(|item| Span::styled(item, Style::default()));

        let list = List::new(list_items)
            .block(
                Block::default()
                    .border_style(focus_border_style(self.has_focus))
                    .borders(Borders::ALL)
                    .title("Software"),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        frame.render_widget(list, area);
        Ok(())
    }
}
