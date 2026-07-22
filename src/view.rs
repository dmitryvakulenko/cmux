use crate::model::Model;
use ratatui::Frame;
use ratatui::layout::Spacing::Space;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Clear, List, ListItem, Paragraph};

pub fn render(frame: &mut Frame, model: &mut Model) {
    let layout = Layout::horizontal(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(80),
    ])
    .split(frame.area());

    let projects_block =
        Block::bordered()
            .title("Projects")
            .border_type(if model.active_view == 0 {
                BorderType::Double
            } else {
                BorderType::Plain
            });

    let mut projects = Vec::with_capacity(model.projects.len());
    model.projects.iter().for_each(|p| {
        projects.push(ListItem::new(p.name.as_str()));
    });

    let list = List::new(projects)
        .block(projects_block)
        .highlight_style(Style::new().bg(Color::Gray).fg(Color::Black));
    frame.render_stateful_widget(list, layout[0], &mut model.selected_project);

    let containers_block =
        Block::bordered()
            .title("Containers")
            .border_type(if model.active_view == 1 {
                BorderType::Double
            } else {
                BorderType::Plain
            });
    frame.render_widget(containers_block, layout[1]);

    let terminal = Block::bordered()
        .title("Terminal")
        .border_type(if model.active_view == 2 {
            BorderType::Double
        } else {
            BorderType::Plain
        });
    frame.render_widget(terminal, layout[2]);

    if model.show_add_project_dialog {
        render_add_project_dialog(frame, model);
    }

    if model.show_remove_project_dialog {
        render_remove_project_dialog(frame, model);
    }
}

fn render_add_project_dialog(frame: &mut Frame, model: &mut Model) {
    let area = frame.area();
    let popup_area = Layout::vertical(vec![Constraint::Length(10)])
        .flex(ratatui::layout::Flex::Center)
        .split(
            Layout::horizontal(vec![Constraint::Length(60)])
                .flex(ratatui::layout::Flex::Center)
                .split(area)[0],
        )[0];

    frame.render_widget(Clear, popup_area);

    let block = Block::bordered()
        .title("Add Project")
        .border_type(BorderType::Double);
    frame.render_widget(block, popup_area);

    let inner_layout = Layout::vertical(vec![
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .margin(1)
    .split(popup_area);

    let name_block =
        Block::bordered()
            .title("Project Name")
            .border_style(if model.add_project_focus == 0 {
                Style::new().fg(Color::Yellow)
            } else {
                Style::new()
            });
    let name_para = Paragraph::new(model.new_project_name.as_str()).block(name_block);
    frame.render_widget(name_para, inner_layout[0]);

    let path_block = Block::bordered().title("Docker-compose path").border_style(
        if model.add_project_focus == 1 {
            Style::new().fg(Color::Yellow)
        } else {
            Style::new()
        },
    );
    let path_para = Paragraph::new(model.new_project_path.as_str()).block(path_block);
    frame.render_widget(path_para, inner_layout[1]);

    let hint = Paragraph::new("Tab: Switch | Enter: Add | Esc: Cancel");
    frame.render_widget(hint, inner_layout[2]);
}

fn render_remove_project_dialog(frame: &mut Frame, model: &mut Model) {
    let window = Block::bordered()
        .title("Remove Project")
        .border_type(BorderType::Double);

    let project_name = &model.projects[model.project_to_remove.unwrap()].name;
    let text = format!(
        "Are you sure you want to remove project '{}'?",
        project_name
    );
    let text_len = text.len();

    let window_area = frame.area().centered(
        Constraint::Length(text_len as u16 + 4),
        Constraint::Length(7),
    );

    let inner_area = window.inner(window_area);
    frame.render_widget(window, window_area);

    let layout =
        Layout::vertical(vec![Constraint::Length(2), Constraint::Length(3)]).split(inner_area);

    frame.render_widget(Text::raw(text).centered(), layout[0]);

    let (btn_layout, _) =
        Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .spacing(Space(1))
            .split_with_spacers(layout[1]);

    render_button(
        frame,
        btn_layout[0],
        "Ok",
        model.remove_project_button_focus == 0,
    );
    render_button(
        frame,
        btn_layout[1],
        "Cancel",
        model.remove_project_button_focus == 1,
    );
}

fn render_button(frame: &mut Frame, area: Rect, text: &str, active: bool) {
    let block = Block::bordered().border_type(if active {
        BorderType::Double
    } else {
        BorderType::Plain
    });
    let para = Paragraph::new(text).centered().block(block);

    frame.render_widget(para, area);
}
