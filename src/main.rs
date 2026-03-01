use std::io;
use std::time::SystemTime;

use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Row, Table, Widget},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Default, Debug)]
struct App {
    state: AppState,
    items: Vec<Data>,
}

#[derive(Default, PartialEq, Debug)]
enum AppState {
    #[default]
    Running,
    Quit,
}

#[derive(Default, Debug)]
struct Data {
    date: String,
    modality: Modality,
    time_in: String,
    time_out: String,
    total_of_hours: u32,
    task_accomplished: String,
}

#[derive(Default, PartialEq, Debug)]
enum Modality {
    #[default]
    Onsite,
    Online,
}

impl App {
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(vec![Constraint::Min(1), Constraint::Length(5)])
            .split(frame.area());

        let rows = [Row::new(vec![
            "Foo",
            "Modality",
            "Time in",
            "Time out",
            "Total of Hours",
            "Task Accomplished",
        ])];
        let widths = [
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ];

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(Style::new().green())
            .header(
                Row::new(vec![
                    "Date",
                    "Modality",
                    "Time in",
                    "Time out",
                    "Total of Hours",
                    "Task Accomplished",
                ])
                .style(Style::new().bold())
                .bottom_margin(1),
            )
            .block(
                Block::bordered()
                    .title("Table")
                    .border_set(border::THICK)
                    .border_style(Style::new().magenta()),
            );

        frame.render_widget(table, layout[0]);
        frame.render_widget(self, layout[1]);
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => self.quit(),
                _ => (),
            },
            _ => {}
        };
        Ok(())
    }

    fn quit(&mut self) {
        self.state = AppState::Quit
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(vec!["OJT Tracker".magenta().bold()]);
        let options = Line::from(vec![
            " <I -".magenta().bold(),
            " Time-In>".magenta().bold(),
            " <O -".magenta().bold(),
            " Time-Out>".magenta().bold(),
            " <Q -".magenta().bold(),
            " Quit> ".magenta().bold(),
        ]);
        let block = Block::bordered()
            .title_top(title.centered())
            .title_bottom(options.centered())
            .border_set(border::THICK)
            .border_style(Style::new().magenta());

        Paragraph::new("").centered().block(block).render(area, buf);
    }
}
