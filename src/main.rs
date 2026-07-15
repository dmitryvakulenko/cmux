use std::collections::HashMap;
use bollard::Docker;
use bollard::query_parameters::EventsOptionsBuilder;
use crossterm::event::EventStream;
use ratatui::DefaultTerminal;
use futures::{stream, StreamExt, stream_select};

pub mod config;
pub mod model;
pub mod tea;
pub mod view;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let term = ratatui::init();
    let res = run(term).await;
    ratatui::restore();

    return res
}

async fn run(mut terminal: DefaultTerminal) -> anyhow::Result<()> {
    let cfg = config::Config::load()?;
    let mut model = model::Model::from_config(cfg.projects);

    let docker = Docker::connect_with_local_defaults()?;
    let filters = HashMap::from([
        ("type".to_string(), vec!["container".to_string()]),
        ("event".to_string(), vec!["start".to_string()]),
    ]);
    let evts_params = EventsOptionsBuilder::new()
        .filters(&filters)
        .build();

    let docker_events = docker.events(Some(evts_params));
    let key_events = EventStream::new();

    let mut all_events = stream_select!(&docker_events, &key_events);

    loop {
        terminal.draw(|frame| view::render(frame, &mut model))?;

        if let Some(evt) = all_events.next().await {

        }

        let msg = tea::handle_input(&model)?;

        model.update(msg);
        if model.quit {
            break;
        }
    }

    if model.need_save_config {
        let cfg = config::Config {
            projects: model
                .projects
                .iter()
                .map(|p| (p.name.clone(), p.compose_path.clone()))
                .collect(),
        };
        cfg.save()?;
    }

    Ok(())
}
