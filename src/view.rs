use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, List, ListItem};
use crate::model::Model;

pub fn render(frame: &mut Frame, model: &mut Model) {
    let layout = Layout::horizontal(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(80),
    ])
    .split(frame.area());

    let projects_block = Block::bordered()
        .title("Projects")
        .border_type(if model.active_view == 0 { BorderType::Double } else { BorderType::Plain });

    let mut projects = Vec::with_capacity(model.projects.len());
    model.projects.iter().for_each(|p| {
        projects.push(ListItem::new(p.name.as_str()));
    });

    let list = List::new(projects)
        .block(projects_block)
        .highlight_style(Style::new().bg(Color::Gray).fg(Color::Black));
    frame.render_stateful_widget(list, layout[0], &mut model.selected_project);

    let containers_block = Block::bordered()
        .title("Containers")
        .border_type(if model.active_view == 1 { BorderType::Double } else { BorderType::Plain });;
    frame.render_widget(containers_block, layout[1]);

    let terminal = Block::bordered()
        .title("Terminal")
        .border_type(if model.active_view == 2 { BorderType::Double } else { BorderType::Plain });
    frame.render_widget(terminal, layout[2]);
}
