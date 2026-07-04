use anyhow::Context;
use ratatui::{DefaultTerminal, Frame};

pub mod model;
pub mod tea;
pub mod view;
pub mod config;

fn main() -> anyhow::Result<()> {
    ratatui::run(run).context("failed to run app")
}

fn run(terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
    let cfg = config::Config::load()?;
    let mut model = model::Model::from_config(cfg.projects);
    loop {
        terminal.draw(|frame| view::render(frame, &mut model))?;
        let msg = tea::handle_input(&model)?;
        model.update(msg);
        if model.quit {
            break;
        }
    }

    if model.need_save_config {
        let cfg = config::Config {
            projects: model.projects.iter().map(|p| (p.name.clone(), p.compose_path.clone())).collect()
        };
        cfg.save()?;
    }

    Ok(())
}
