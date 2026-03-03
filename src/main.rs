use std::io;
// use std::time::SystemTime;

use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Row, StatefulWidget, Table, TableState, Widget},
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
    app_state: AppState,
    table_state: TableState,
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

    fn draw(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(vec![Constraint::Min(1), Constraint::Length(5)])
            .split(frame.area());

        self.render_table(frame, layout[0]);
        self.render_app(frame, layout[1]);
    }

    fn is_running(&self) -> bool {
        self.app_state == AppState::Running
    }

    fn quit(&mut self) {
        self.app_state = AppState::Quit
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

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let rows = [Row::new(vec![
            "Foo",
            "Modality",
            "Time in",
            "Time out",
            "Total of Hours",
            "Task Accomplished",
        ])];
        let widths = [
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
            Constraint::Min(1),
        ];

        let table = Table::new(rows, widths)
            .column_spacing(1)
            .style(Style::new().yellow())
            .header(
                Row::new(vec![
                    "Date",
                    "Modality",
                    "Time in",
                    "Time out",
                    "Total of Hours",
                    "Task Accomplished",
                ])
                .style(Style::new().bold().magenta())
                .bottom_margin(1),
            )
            .block(
                Block::bordered()
                    .title("Table")
                    .border_set(border::THICK)
                    .border_style(Style::new().magenta()),
            );

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    fn render_app(&mut self, frame: &mut Frame, area: Rect) {
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

        let p = Paragraph::new("").centered().block(block);
        frame.render_widget(p, area);
    }
}
