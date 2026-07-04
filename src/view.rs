use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, List, ListItem, ListState};
use crate::model::Model;

pub fn render(frame: &mut Frame, model: &mut Model) {
    let layout = Layout::horizontal(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(80),
    ])
    .split(frame.area());

    let projects_block = Block::bordered().title("Projects");
    let mut projects = Vec::with_capacity(model.projects.len());
    model.projects.iter().for_each(|p| {
        projects.push(ListItem::new(p.name.as_str()));
    });

    let list = List::new(projects)
        .block(projects_block)
        .highlight_style(Style::new().bg(Color::Gray).fg(Color::Black));
    frame.render_stateful_widget(list, layout[0], &mut model.selected_project);

    let containers = Block::bordered().title("Containers");
    frame.render_widget(containers, layout[1]);

    let terminal = Block::bordered().title("Terminal");
    frame.render_widget(terminal, layout[2]);
}
