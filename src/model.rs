use std::collections::HashMap;
use ratatui::widgets::ListState;
use crate::tea::Message;

#[derive(Debug, Default)]
pub struct Model {
    pub projects: Vec<Project>,
    pub selected_project: ListState,
    pub active_view: i8,
    pub need_save_config: bool,
    pub quit: bool
}

#[derive(Debug, Default)]
pub struct Project {
    pub name: String,
    pub compose_path: String
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_config(cfg: HashMap<String, String>) -> Self {
        let mut res = Self::default();
        res.projects = cfg.into_iter().map(|(name, path)| Project::new(name, path)).collect();

        res
    }

    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::None => (),
            Message::NextView => {
                self.active_view += 1;
                if self.active_view > 2 {
                    self.active_view = 2;
                }
            }
            Message::PrevView => {
                self.active_view -= 1;
                if self.active_view < 0 {
                    self.active_view = 0;
                }
            },
            Message::NextProject => {
                self.selected_project.select_next();
            }
            Message::PrevProject => {
                self.selected_project.select_previous();
            }
            Message::NextContainer => {}
            Message::PrevContainer => {}
            Message::Quit => self.quit = true,
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