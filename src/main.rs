use bollard::Docker;
use bollard::config::EventMessage;
use bollard::query_parameters::EventsOptionsBuilder;
use crossterm::event::EventStream;
use futures::{Stream, StreamExt, stream_select};
use ratatui::DefaultTerminal;
use std::collections::HashMap;
use std::io;

pub mod config;
pub mod model;
pub mod tea;
pub mod view;

enum Events {
    Docker(Result<EventMessage, bollard::errors::Error>),
    Crossterm(io::Result<crossterm::event::Event>),
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let term = ratatui::init();
    let res = run(term).await;
    ratatui::restore();

    return res;
}

async fn run(mut terminal: DefaultTerminal) -> anyhow::Result<()> {
    let mut cfg = config::Config::load()?;
    let mut model = model::Model::from_config(&cfg.projects);

    let docker = Docker::connect_with_local_defaults()?;
    let filters = HashMap::from([
        ("type", vec!["container"]),
        ("event", vec!["start", "stop"]),
    ]);
    let evts_params = EventsOptionsBuilder::new().filters(&filters).build();

    let docker_events = docker.events(Some(evts_params)).map(|e| Events::Docker(e));
    let key_events = EventStream::new().map(|e| Events::Crossterm(e));

    let mut all_events = stream_select!(docker_events, key_events);

    loop {
        terminal.draw(|frame| view::render(frame, &mut model))?;

        if let Some(e) = all_events.next().await {
            let msg = match e {
                Events::Docker(e) => tea::handle_docker_event(&model, e)?,
                Events::Crossterm(e) => tea::handle_input(&model, e)?,
            };

            model.update(msg);
        }

        if model.quit {
            break;
        }
    }

    if model.need_save_config {
        cfg.projects = model
            .projects
            .iter()
            .map(|p| {
                config::Project::new(p.name.clone(), p.compose_path.clone(), p.shell_cmd.clone())
            })
            .collect();
        cfg.save()?;
    }

    Ok(())
}
