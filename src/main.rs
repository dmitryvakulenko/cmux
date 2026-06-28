use ratatui::{DefaultTerminal, Frame};
use ratatui::widgets::Paragraph;
use std::time::Duration;
use ratatui::crossterm::event::{self, KeyCode};
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    ratatui::run(run).context("failed to run app")
}

fn run(terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    loop {
        terminal.draw(render)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

fn should_quit() -> anyhow::Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        let q_pressed = event::read()
            .context("event read failed")?
            .as_key_press_event()
            .is_some_and(|key| key.code == KeyCode::Char('q'));
        return Ok(q_pressed);
    }
    Ok(false)
}
