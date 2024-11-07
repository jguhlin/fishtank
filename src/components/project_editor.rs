use std::{borrow::Cow, time::Instant};

use color_eyre::Result;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};
use serde::{Deserialize, Serialize};

use super::Component;

use crate::action::Action;
use crate::components::utils::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectList {
    pub items: Vec<Project>, // todo - replace with software struct
    pub has_focus: bool,
}

impl Focusable for ProjectList {
    fn focus(&mut self) {
        self.has_focus = true;
    }

    fn unfocus(&mut self) {
        self.has_focus = false;
    }
}

impl Default for ProjectList {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            has_focus: true,
        }
    }
}

impl Component for ProjectList {
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

        // If we have focus, add key bindings to the bottom border
        let title_bottom = if self.has_focus {
            Line::from(vec![
                // a add
                Span::styled(" [", Style::default()),
                Span::styled("a", Style::default().fg(Color::Yellow)),
                Span::styled("] Add", Style::default()),
                // r archive
                Span::styled(" [", Style::default()),
                Span::styled("r", Style::default().fg(Color::Yellow)),
                Span::styled("] Archive", Style::default()),
                // t show archived
                Span::styled(" [", Style::default()),
                Span::styled("t", Style::default().fg(Color::Yellow)),
                Span::styled("] Show Archived", Style::default()),
            ])
            .centered()
        } else {
            Line::from(vec![])
        };

        let list = List::new(list_items)
            .block(
                Block::default()
                    .border_style(focus_border_style(self.has_focus))
                    .borders(Borders::ALL)
                    .title("Projects")
                    .title_bottom(title_bottom),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        frame.render_widget(list, area);
        Ok(())
    }
}

/// Defines the Nanopore sequencing project
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Project {
    name: String,
    pub path: String,
    pub read_count: u64,
    pub basecall_runs: Vec<BasecallRun>,
    pub format: RawBasecallsFormat,
}

impl Project {
    pub fn to_list_item(&self) -> String {
        format!("{} ({})", self.name, self.read_count)
    }
}

impl<'a> Into<Cow<'a, str>> for &Project {
    fn into(self) -> Cow<'a, str> {
        self.to_list_item().into()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RawBasecallsFormat {
    Fast5,
    Slow5,
    #[default]
    Pod5,
}

/// Defines a basecall run within a project
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BasecallRun {
    pub name: String,
    pub path: String,
    pub read_count: u64,
    pub output_path: String,
    pub basecaller: Basecaller,
}

/// Defines the basecaller used to process raw basecalls
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Basecaller {
    pub name: String,
    pub version: String,
    pub path: String,
    pub config: String,
    pub model: String,
}
