use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal,
    prelude::*,
    widgets::{Block, List, ListItem, ListState, Paragraph, Wrap},
};

mod days;

struct App {
    days: DaysList,
    status: Option<String>,
}

impl App {
    fn new() -> Self {
        Self {
            days: DaysList::new(12),
            status: None,
        }
    }

    fn select_next(&mut self) {
        self.days.select_next();
    }

    fn select_previous(&mut self) {
        self.days.select_previous();
    }

    fn run_selected_day(&mut self) {
        let Some(day) = self.days.selected_day() else {
            self.status = Some("No day selected".into());
            return;
        };

        let result = match day.number {
            1 => days::day01::run(),
            _ => Ok(format!("Day {:02} not implemented yet.", day.number)),
        };

        match result {
            Ok(message) => self.status = Some(message),
            Err(report) => {
                self.status = Some(format!("Day {:02} failed: {}", day.number, report));
            }
        }
    }
}

struct DaysList {
    items: Vec<Day>,
    state: ListState,
}

impl DaysList {
    fn new(count: u8) -> Self {
        let items = (1..=count).map(Day::new).collect();
        let mut state = ListState::default();
        state.select(Some(0));
        Self { items, state }
    }

    fn selected_day(&self) -> Option<Day> {
        self.state.selected().map(|idx| self.items[idx])
    }

    fn select_next(&mut self) {
        if self.items.is_empty() {
            self.state.select(None);
            return;
        }
        let next = match self.state.selected() {
            Some(idx) if idx + 1 < self.items.len() => idx + 1,
            _ => 0,
        };
        self.state.select(Some(next));
    }

    fn select_previous(&mut self) {
        if self.items.is_empty() {
            self.state.select(None);
            return;
        }
        let prev = match self.state.selected() {
            Some(0) | None => self.items.len() - 1,
            Some(idx) => idx - 1,
        };
        self.state.select(Some(prev));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Day {
    number: u8,
}

impl Day {
    fn new(number: u8) -> Self {
        Self { number }
    }

    fn title(&self) -> String {
        format!("Day {:02}", self.number)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let app = App::new();
    let terminal = ratatui::init();
    run(terminal, app)
}

fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &mut app))?;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                KeyCode::Char('j') | KeyCode::Down => app.select_next(),
                KeyCode::Char('k') | KeyCode::Up => app.select_previous(),
                KeyCode::Enter => app.run_selected_day(),
                _ => {}
            },
            _ => {}
        }
    }
}

fn render(frame: &mut Frame, app: &mut App) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let art = Paragraph::new(christmas_tree())
        .block(Block::default().title("Advent of Code"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    frame.render_widget(art, layout[0]);

    let side_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(5)])
        .split(layout[1]);

    let items: Vec<ListItem> = app
        .days
        .items
        .iter()
        .map(|day| ListItem::new(day.title()))
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Days"))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("-> ");
    frame.render_stateful_widget(list, side_layout[0], &mut app.days.state);

    let status_text = app
        .status
        .as_deref()
        .unwrap_or("Press Enter to run the selected day.");
    let status = Paragraph::new(status_text)
        .block(Block::default().title("Status"))
        .wrap(Wrap { trim: true });
    frame.render_widget(status, side_layout[1]);
}

fn christmas_tree() -> Text<'static> {
    let rows = [
        "                               *",
        "                              /_\\",
        "                             /o#\\",
        "                            /###\\",
        "                           /#o#o\\",
        "                          /o###o\\",
        "                         /###o##\\",
        "                        /#o###o#\\",
        "                       /o###o###\\",
        "                      /###o###o#\\",
        "                     /#o###o###o\\",
        "                    /o###o###o##\\",
        "                   /###o###o###o\\",
        "                  /#o###o###o###\\",
        "                 /o###o###o###o#\\",
        "                /###o###o###o###\\",
        "               /#o###o###o###o##\\",
        "              /o###o###o###o###o\\",
        "             /###o###o###o###o##\\",
        "            /#o###o###o###o###o#\\",
        "           /o###o###o###o###o###\\",
        "          /###o###o###o###o###o#\\",
        "         /#o###o###o###o###o###o\\",
        "        /########################\\",
        "       /##########################\\",
        "      /o##########################o\\",
        "     /##############################\\",
        "               |||||    |||||",
        "               |||||    |||||",
        "              _/___\\____/___\\_",
        "             /_____/  \\_____ \\",
    ];

    rows.iter().map(|row| colorize_row(row)).collect()
}

fn colorize_row(row: &str) -> Line<'static> {
    let spans: Vec<_> = row.chars().map(colorize_char).collect();
    Line::from(spans)
}

fn colorize_char(ch: char) -> Span<'static> {
    let style = match ch {
        '*' => Style::default().fg(Color::Yellow),
        'o' => Style::default().fg(Color::Red),
        '#' | '/' | '\\' => Style::default().fg(Color::Green),
        '_' => Style::default().fg(Color::Rgb(34, 139, 34)),
        '|' => Style::default().fg(Color::Rgb(139, 69, 19)),
        _ => Style::default().fg(Color::White),
    };
    Span::styled(ch.to_string(), style)
}
