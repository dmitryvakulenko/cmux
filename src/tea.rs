use std::time::Duration;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode};
use anyhow::Context;
use crate::model::Model;

pub enum Message {
    None,
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
        _ => Ok(Message::None),
    }
}