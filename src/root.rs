use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    widgets::{Widget, WidgetRef},
};

use crate::components::Logo;

pub struct Root {
    children: Vec<Box<dyn WidgetRef>>,
}

impl Root {
    pub fn new() -> Self {
        Self {
            children: vec![Box::new(Logo::default())],
        }
    }

    pub fn add_child(&mut self, child: Box<dyn WidgetRef>) {
        self.children.push(child);
    }
}

impl WidgetRef for Root {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6)])
            .flex(Flex::Center)
            .split(area);

        let parts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(3)])
            .split(chunks[0]);

        let mut i = 0;
        for child in &self.children {
            child.render_ref(parts[i], buf);
            i = i + 1;
        }
    }
}
