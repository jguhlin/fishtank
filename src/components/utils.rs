use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

pub trait Focusable {
    fn focus(&mut self);
    fn unfocus(&mut self);
}

pub fn focus_border_style(has_focus: bool) -> Style {
    if has_focus {
        Style::default().fg(Color::White)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}
