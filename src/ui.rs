use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Row, Table},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Min(0)])
        .split(f.size());
    draw_first_tab(f, app, chunks[0]);
}

fn draw_first_tab<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(area);
    draw_files(f, app, chunks[0]);
}

fn draw_files<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let files: Vec<Row> = app
        .files
        .files
        .iter()
        .map(|file| Row::new(vec![file.name.as_str(), file.size.as_str()]))
        .collect();
    let table = Table::new(files)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(app.default_path.as_ref()),
        )
        .header(Row::new(vec!["Name", "Size"]).bottom_margin(1))
        .widths(&[Constraint::Length(15), Constraint::Length(15)])
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    f.render_stateful_widget(table, area, &mut app.files.state);
}