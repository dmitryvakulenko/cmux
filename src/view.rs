use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Block;
use crate::model::Model;

pub fn render(frame: &mut Frame, model: &Model) {
    // let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    // frame.render_widget(greeting, frame.area());
    let layout = Layout::horizontal(vec![
        Constraint::Percentage(10),
        Constraint::Percentage(10),
        Constraint::Percentage(80),
    ])
    .split(frame.area());

    let files = Block::bordered().title("Compose files");
    frame.render_widget(files, layout[0]);

    let containers = Block::bordered().title("Containers");
    frame.render_widget(containers, layout[1]);

    let terminal = Block::bordered().title("Terminal");
    frame.render_widget(terminal, layout[2]);
}
