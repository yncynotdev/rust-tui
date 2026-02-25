# rust-tui

A self TUI project to learn Rust

## Patterns

Just a documentation on how "I" implement (it's all on the docs, I'm new to Rust, need to help me remember things)

```
use color_eyre::Result;
// this is like how I view common imports for ratatui
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, // the backend for ratatui
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    DefaultTerminal,
};

// return color_eyre::Result;
fn main() -> Result<()> {
   color_eyre::install()?;
   let terminal = ratatui::init();
   let app_result = App.run(terminal);
   app_result
}

// They alway call this struct to determine the parts or features of your TUI
// AFAIK: You should implement some sort of State Machine to determine states of your TUI
[#derive(Default, Debug)]
struct App { 
    state: AppState,
}

// The state machine itself using "enum"
// As you can see, we have basic states such as Running and Quit
// That's like the bread and butter of your state
// You can add more
[#derive(Default, PartialEq, Debug)]
enum AppState {
    [#default] // Means "Running" will be the default
    Running,
    Quit
}

// Functions should be accessed or a member of App struct
impl App {
    // They always have the run() and render() functions as your bread and butter combo
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        // Always call the loop here
        while self.is_running() {
            // calls draw() callback
            terminal.draw(|frame| self.draw(frame))?;
            todo!("Call some key even functions here");
        }
        Ok(());
    }

    fn draw(&self, frame: &mut Frame) {
        // call the render function
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&self) -> Result<()> {
        // Check event reads
        if let Event::Key(key) = event::read()? {
            // Check if you are pressing a key 
            // key is KeyEventKind::Press
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }
            todo!("match logic");
        }
    }

    // Handle the state here
    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }
}

// You call the ratatui::widgets::Widget here and point it to the struct App
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!("Render logic here");
    }
}
```
