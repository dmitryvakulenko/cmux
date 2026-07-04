use anyhow::Context;
use ratatui::{DefaultTerminal, Frame};

mod model;
mod tea;
mod view;

fn main() -> anyhow::Result<()> {
    ratatui::run(run).context("failed to run app")
}

fn run(terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    let mut model = model::Model::new();
    loop {
        terminal.draw(|frame| view::render(frame, &model))?;
        let msg = tea::handle_input(&model::Model::new())?;
        model.update(msg);
        if model.quit {
            break;
        }
    }

    Ok(())
}
