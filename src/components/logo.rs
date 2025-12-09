use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, WidgetRef},
};

use crate::types::EventHandler;

pub struct Logo {
    logo_left: [&'static str; 3],
    logo_right: [&'static str; 3],
}

impl Logo {
    pub fn new() -> Self {
        Self {
            logo_left: ["█▀▀█ █▀▀█ █▀▀▀", "█▄▄█ █░░█ █░░░", "▀  ▀ ▀▀▀▀ ▀▀▀▀"],
            logo_right: ["▀▀▀█ █▀▀▀", "█▀▀▀ ▀▀▀█", "▀▀▀▀ ▀▀▀▀"],
        }
    }
}

impl WidgetRef for Logo {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut lines: Vec<Line> = Vec::new();

        for i in 0..self.logo_left.len() {
            lines.push(Line::from(vec![
                Span::styled(self.logo_left[i], Style::default().fg(Color::DarkGray)),
                Span::raw(" "),
                Span::styled(self.logo_right[i], Style::default().fg(Color::Gray)),
            ]));
        }

        let version = Line::from(vec![Span::styled(
            "v.0.0.1",
            Style::default().fg(ratatui::style::Color::DarkGray),
        )]);
        lines.push(version);

        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}

impl EventHandler for Logo {
    fn handle_event(&mut self, _key: ratatui::crossterm::event::KeyCode) {
        //no-op
    }
}
