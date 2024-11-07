use std::{borrow::Cow, time::Instant};

use color_eyre::Result;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};
use serde::{Deserialize, Serialize};

use super::Component;

use crate::action::Action;
use crate::components::utils::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub has_focus: bool,
    pub margin: Margin,
}

impl Focusable for Settings {
    fn focus(&mut self) {
        self.has_focus = true;
    }

    fn unfocus(&mut self) {
        self.has_focus = false;
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Self {
        Self {
            has_focus: false,
            margin: Margin {
                horizontal: 0,
                vertical: 0,
            },
        }
    }
}

impl Component for Settings {
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
        // Put a border around the fish tank
        let block = Block::default()
            .title("Global Settings")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title_style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );

        // Text Input fields

        // pod5/fast5 Project Base Dir
        let pod5_project_base_dir = Paragraph::new("pod5/fast5 Project Base Dir")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White))
                    .title_style(
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
            );
       
        // Add the text input fields to the list
        let settings_list = List::new(vec![pod5_project_base_dir])
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::LightBlue))
            .highlight_symbol(">>");

        // Layout the components
        let settings_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(0),
                ]
                .as_ref(),
            )
            .split(area);

        // Render the list
        frame.render_widget(settings_list, settings_layout[1]);

        

        
        // Render the block
        frame.render_widget(block, area);

        

        Ok(())
    }
}
