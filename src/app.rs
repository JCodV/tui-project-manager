use std::default;

use color_eyre::{owo_colors::OwoColorize, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Cell, Gauge, Paragraph, Row, Table},
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

// can maybe have task modules which separates
// related tasks into grouped categories for
// better organization
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

#[derive(Debug)]
enum AppState {
    ProjectSelectionPage,
    ProjectCreationPage,
    SelectedProjectTodoList,
    TaskCreationPage,
}

impl Default for AppState {
    fn default() -> Self {
        AppState::ProjectSelectionPage
    }
}

#[derive(Debug, Default)]
pub struct App {
    is_running: bool,
    state: AppState,
    projects: Vec<Project>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.is_running = true;
        while self.is_running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn load_saved_projects(file_name: &str) {}

    fn save_projects(file_name: &str) {}

    fn draw(&mut self, frame: &mut Frame) {
        //self.draw_project_selection_page(frame);
        self.draw_table(frame);
    }

    fn draw_project_selection_page(&self, frame: &mut Frame) {
        let title = Line::from("Project Manager").blue().bold().centered();

        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";

        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        );
    }

    fn draw_table(&self, frame: &mut Frame) {
        let progress_bar = Gauge::default()
            .block(Block::bordered().title("Progress"))
            .percent(50);

        let header = Row::new(vec!["this", "is", "DATA!"]);

        let rows = [Row::new(vec![
            Cell::from("this"),
            Cell::from("is"),
            Cell::from("SPARTA!!!"),
        ])];

        let table = Table::default().header(header).rows(rows);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
            ])
            .split(frame.area());

        frame.render_widget(table, layout[0]);
        frame.render_widget(progress_bar, layout[1]);
        // frame.render_widget(progress_bar, frame.area());
    }

    fn draw_project_creation_popup(&self, frame: &mut Frame) {}

    fn draw_selected_project_todo_list(&self, frame: &mut Frame) {}

    fn draw_task_creation_popup(&self, frame: &mut Frame) {}

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
        self.is_running = false;
    }
}
