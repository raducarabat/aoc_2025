use color_eyre::eyre::Result;
use components::Logo;
use crossterm::event::read;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, RatatuiLogo, Widget, WidgetRef},
};
use root::Root;

mod components;
mod root;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut root = Root::new();
    let version = Line::from(vec![Span::styled(
        "v.0.0.1",
        Style::default().fg(Color::DarkGray),
    )]);
    root.add_child(Box::new(
        Paragraph::new(version).alignment(Alignment::Center),
    ));
    loop {
        terminal.draw(|frame| {
            render(frame, &root);
        })?;
        if matches!(read()?, crossterm::event::Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, root: &Root) {
    root.render_ref(frame.area(), frame.buffer_mut());
}
