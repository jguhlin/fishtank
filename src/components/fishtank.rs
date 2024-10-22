use std::{borrow::Cow, time::Instant};

use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use color_eyre::Result;
use ratatui::{
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};
use serde::{Deserialize, Serialize};

use super::Component;

use crate::action::Action;
use crate::components::utils::*;

pub const FISH1: &str = 
r#"
|\   \\\\__     o
| \_/    o \    o 
> _   (( <_  oo  
| / \__+___/      
|/     |/
"#;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fishtank {
    pub has_focus: bool,
    pub margin: Margin,
}

impl Focusable for Fishtank {
    fn focus(&mut self) {
        self.has_focus = true;
    }

    fn unfocus(&mut self) {
        self.has_focus = false;
    }
}

impl Default for Fishtank {
    fn default() -> Self {
        Self::new()
    }
}

impl Fishtank {
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

impl Component for Fishtank {
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
            .title("Fish Tank")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));

        let fish = FISH1;

        // Create it as a Text widget, with Lines for each \n in the string
        let text = Text::styled(fish, Style::default().fg(Color::White));

        // Each tick we want to move the fish around a bit
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(0);

        // Only move by 1 in any direction (down to 0) 20% of the time
        let move_chance = 0.2;
        let move_amount = 1 as i16;

        // Get the current fish position
        let mut fish_x = self.margin.horizontal as i16;
        let mut fish_y = self.margin.vertical as i16;

        // Move the fish
        if rng.gen::<f64>() < move_chance {
            fish_x += rng.gen_range(-move_amount..=move_amount);
            fish_y += rng.gen_range(-move_amount..=move_amount);
        }

        // Keep the fish in the tank
        fish_x = fish_x.max(self.margin.horizontal as i16).min(area.width as i16 - self.margin.horizontal as i16);
        fish_y = fish_y.max(self.margin.vertical as i16).min(area.height as i16 - self.margin.vertical as i16);

        // Update margin
        self.margin = Margin {
            horizontal: fish_x as u16,
            vertical: fish_y as u16,
        };

        let area = area.inner(self.margin);

        frame.render_widget(text, area);

        Ok(())
    }
}
