use std::time::Duration;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use anyhow::Context;
use crate::model::Model;

pub enum Message {
    None,
    NextView,
    PrevView,
    NextProject,
    PrevProject,
    NextContainer,
    PrevContainer,
    Quit,
}

pub fn handle_input(m: &Model) -> anyhow::Result<Message> {
    if !event::poll(Duration::from_millis(250)).context("event poll failed")? {
        return Ok(Message::None);
    }

    let q_pressed = event::read()
        .context("event read failed")?
        .as_key_press_event();

    if q_pressed.is_none() {
        return Ok(Message::None);
    }

    let key = q_pressed.unwrap().code;
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

        _ => Ok(Message::None),
    }
}