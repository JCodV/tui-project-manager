use std::collections::HashMap;

use color_eyre::{owo_colors::OwoColorize, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug)]
struct Project {
    name: String,
    description: String,
    file_location: String,
    // languages: Vec<String>,
}

impl Project {
    pub fn new(name: String, description: String, file_location: String) -> Self {
        Self {
            name,
            description,
            file_location,
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,

    // project name mapped to location
    projects: Vec<Project>,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_saved_projects(file_name: &str) {}

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/master/examples>
    fn draw(&mut self, frame: &mut Frame) {
        self.draw_startup_page(frame);
    }

    fn draw_startup_page(&self, frame: &mut Frame) {
        let title = Line::from("Project Manager").blue().bold().centered();

        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }

    fn draw_project_selection_page(&self, frame: &mut Frame) {}

    fn draw_project_creation_page(&self, frame: &mut Frame) {}

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
