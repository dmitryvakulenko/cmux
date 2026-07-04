use ratatui::widgets::ListState;
use crate::tea::Message;

pub struct Model {
    pub projects: Vec<Project>,
    pub selected_project: ListState,
    pub quit: bool
}

pub struct Project {
    pub name: String,
    pub compose_path: String
}

impl Model {
    pub fn new() -> Self {
        Self {
            projects: vec![
                Project::new("project1".to_string(), "/path/to/project1".to_string()),
                Project::new("project2".to_string(), "/path/to/project2".to_string()),
            ],
            selected_project: ListState::default().with_selected(Some(0)),
            quit: false,
        }
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Quit => self.quit = true,
            Message::None => (),
        }
    }
}

impl Project {
    pub fn new(name: String, compose_path: String) -> Self {
        Self {
            name,
            compose_path
        }
    }
}