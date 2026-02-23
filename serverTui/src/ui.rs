use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.size());

    let panes = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(root[0]);

    let docker_text = app.docker_lines.join("\n");
    let pm2_text = app.pm2_lines.join("\n");

    let docker =
        Paragraph::new(docker_text).block(Block::default().borders(Borders::ALL).title("Docker"));
    let pm2 = Paragraph::new(pm2_text).block(Block::default().borders(Borders::ALL).title("PM2"));

    f.render_widget(docker, panes[0]);
    f.render_widget(pm2, panes[1]);

    let status = Paragraph::new(app.status.clone()).block(Block::default().borders(Borders::TOP));
    f.render_widget(status, root[1]);
}
