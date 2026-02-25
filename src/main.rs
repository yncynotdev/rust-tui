use color_eyre::Result;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    widgets::Widget,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
struct App {
    layout_name: String,
    state: AppState,
}

#[derive(Default, PartialEq)]
enum AppState {
    #[default]
    Running,
    // Quit,
}

impl App {
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!("Later")
    }
}
