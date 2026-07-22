use crate::config;
use crate::tea::Message;
use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub compose_path: String,
    pub shell_cmd: Option<String>,
}

#[derive(Debug)]
pub struct Model {
    pub projects: Vec<Project>,
    pub selected_project: ListState,
    pub active_view: i8,
    pub need_save_config: bool,
    pub show_add_project_dialog: bool,
    pub new_project_name: String,
    pub new_project_path: String,
    pub add_project_focus: usize,
    pub show_remove_project_dialog: bool,
    pub remove_project_button_focus: usize,
    pub project_to_remove: Option<usize>,
    pub quit: bool,
}

impl Model {
    pub fn from_config(projects: &Vec<config::Project>) -> Self {
        let mut res = Self {
            projects: projects
                .iter()
                .map(|p| Project {
                    name: p.name.clone(),
                    compose_path: p.compose_path.clone(),
                    shell_cmd: p.shell_cmd.clone(),
                })
                .collect(),
            selected_project: ListState::default(),
            active_view: 0,
            need_save_config: false,
            show_add_project_dialog: false,
            new_project_name: "".to_string(),
            new_project_path: "".to_string(),
            add_project_focus: 0,
            show_remove_project_dialog: false,
            remove_project_button_focus: 1,
            project_to_remove: None,
            quit: false,
        };

        if res.projects.len() == 1 {
            res.selected_project = res.selected_project.with_selected(Some(0));
        }

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
            }
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
            }
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
                if self.show_add_project_dialog {
                    self.add_project_focus = (self.add_project_focus + 1) % 2;
                } else if self.show_remove_project_dialog {
                    self.remove_project_button_focus = (self.remove_project_button_focus + 1) % 2;
                }
            }
            Message::AddNewProject => {
                if !self.new_project_name.is_empty() && !self.new_project_path.is_empty() {
                    self.projects.push(Project {
                        name: self.new_project_name.clone(),
                        compose_path: self.new_project_path.clone(),
                        shell_cmd: None,
                    });
                    self.show_add_project_dialog = false;
                    self.need_save_config = true;
                }
                if self.projects.len() == 1 {
                    self.selected_project = self.selected_project.with_selected(Some(0));
                }
            }
            Message::RemoveProjectDialog(idx) => {
                self.show_remove_project_dialog = true;
                self.project_to_remove = Some(idx);
                self.remove_project_button_focus = 1;
            }
            Message::ConfirmRemoveProject => {
                if let Some(idx) = self.project_to_remove {
                    if idx < self.projects.len() {
                        self.projects.remove(idx);
                        self.need_save_config = true;
                        if self.projects.is_empty() {
                            self.selected_project.select(None);
                        } else {
                            let new_idx = if idx >= self.projects.len() {
                                self.projects.len() - 1
                            } else {
                                idx
                            };
                            self.selected_project.select(Some(new_idx));
                        }
                    }
                }
                self.show_remove_project_dialog = false;
                self.project_to_remove = None;
            }
            Message::CancelRemoveProject => {
                self.show_remove_project_dialog = false;
                self.project_to_remove = None;
            }
        }
    }
}
