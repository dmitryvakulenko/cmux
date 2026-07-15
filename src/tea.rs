use std::io;
use std::time::Duration;
use anyhow::Context;
use bollard::config::EventMessage;
use crossterm::event;
use crossterm::event::KeyCode;
use crate::model::Model;

pub enum Message {
    None,
    NextView,
    PrevView,
    NextProject,
    PrevProject,
    NextContainer,
    PrevContainer,
    ShowAddProjectDialog,
    HideAddProjectDialog,
    RemoveProjectDialog,
    Input(char),
    Backspace,
    Tab,
    Submit,
    Quit,
}

pub fn handle_input(m: &Model, evt: io::Result<crossterm::event::Event>) -> anyhow::Result<Message> {
    let q_pressed = evt
        .context("event read failed")?
        .as_key_press_event();

    if q_pressed.is_none() {
        return Ok(Message::None);
    }

    let key = q_pressed.unwrap().code;

    if m.show_add_project_dialog {
        return match key {
            KeyCode::Esc => Ok(Message::HideAddProjectDialog),
            KeyCode::Char(c) => Ok(Message::Input(c)),
            KeyCode::Backspace => Ok(Message::Backspace),
            KeyCode::Tab => Ok(Message::Tab),
            KeyCode::Enter => Ok(Message::Submit),
            _ => Ok(Message::None),
        };
    }

    match key {
        KeyCode::Char('q') => Ok(Message::Quit),

        KeyCode::Left => Ok(Message::PrevView),
        KeyCode::Right => Ok(Message::NextView),

        KeyCode::Up => {
            if m.active_view == 0 {
                Ok(Message::PrevProject)
            } else if m.active_view == 1 {
                Ok(Message::PrevContainer)
            } else {
                Ok(Message::None)
            }
        },
        
        KeyCode::Down => {
            if m.active_view == 0 {
                Ok(Message::NextProject)
            } else if m.active_view == 1 {
                Ok(Message::NextContainer)
            } else {
                Ok(Message::None)
            }
        },

        KeyCode::Char('a') => {
            if m.active_view == 0 {
                Ok(Message::ShowAddProjectDialog)
            } else {
                Ok(Message::None)
            }
        }
        KeyCode::Char('d') => {
            if m.active_view == 0 {
                Ok(Message::RemoveProjectDialog)
            } else {
                Ok(Message::None)
            }
        }

        KeyCode::Esc => {
            if m.show_add_project_dialog {
                Ok(Message::HideAddProjectDialog)
            } else {
                Ok(Message::None)
            }
        }

        _ => Ok(Message::None),
    }
}

pub fn handle_docker_event(
    m: &Model,
    evt_result: Result<EventMessage, bollard::errors::Error>,
) -> anyhow::Result<Message> {
    let evt = evt_result.context("event read failed")?;

    return Ok(Message::None);
}