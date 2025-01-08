use std::collections::HashMap;

use color_eyre::{owo_colors::OwoColorize, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
struct Task {
    description: String,
    is_complete: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            description,
            is_complete: false,
        }
    }
}

#[derive(Debug, Default)]
struct TodoList {
    tasks: Vec<Task>,
    progress: f32,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            progress: 0.0,
        }
    }

    fn calculate_progress(&self) -> f32 {
        let total_tasks = self.tasks.len();
        let mut completed_tasks = 0;

        for task in self.tasks.iter() {
            if task.is_complete {
                completed_tasks += 1;
            }
        }

        let percentage: f32 = (completed_tasks as f32 / total_tasks as f32) * 100.0;
        return percentage;
    }
}

#[derive(Debug)]
struct Project {
    name: String,
    description: String,
    file_location: String,
    todo_list: TodoList,
    // languages: Vec<String>, // can probably just pull from github
}

impl Project {
    pub fn new(name: String, description: String, file_location: String) -> Self {
        Self {
            name,
            description,
            file_location,
            todo_list: TodoList::new(),
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
