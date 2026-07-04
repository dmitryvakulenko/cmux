use std::collections::HashMap;
use ratatui::widgets::ListState;
use crate::tea::Message;

#[derive(Debug, Default)]
pub struct Model {
    pub projects: Vec<Project>,
    pub selected_project: ListState,
    pub active_view: i8,
    pub need_save_config: bool,
    pub show_add_project_dialog: bool,
    pub new_project_name: String,
    pub new_project_path: String,
    pub add_project_focus: usize,
    pub hide_add_project_dialog: bool,
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
            Message::ShowAddProjectDialog => {
                self.show_add_project_dialog = true;
                self.new_project_name.clear();
                self.new_project_path.clear();
                self.add_project_focus = 0;
            }
            Message::HideAddProjectDialog => {
                self.show_add_project_dialog = false;
            },
            Message::Input(c) => {
                if self.add_project_focus == 0 {
                    self.new_project_name.push(c);
                } else {
                    self.new_project_path.push(c);
                }
            }
            Message::Backspace => {
                if self.add_project_focus == 0 {
                    self.new_project_name.pop();
                } else {
                    self.new_project_path.pop();
                }
            }
            Message::Tab => {
                self.add_project_focus = (self.add_project_focus + 1) % 2;
            }
            Message::Submit => {
                if !self.new_project_name.is_empty() && !self.new_project_path.is_empty() {
                    self.projects.push(Project::new(self.new_project_name.clone(), self.new_project_path.clone()));
                    self.show_add_project_dialog = false;
                    self.need_save_config = true;
                }
            }
            Message::RemoveProjectDialog => {

            }
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