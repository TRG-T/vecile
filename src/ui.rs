use crate::app::{App, PopupType};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Row, Table},
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
    if app.popup.visible {
        draw_popup(f, app, area, 20, 20)
    }
}

fn draw_popup<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    area: Rect,
    percent_x: u16,
    percent_y: u16,
) {
    let block = Block::default()
        .title(app.popup.title)
        .borders(Borders::ALL);
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(vertical[1])[1];
    match app.popup.p_type {
        PopupType::DeleteFile => draw_delete_popup(f, app, horizontal, block),
        _ => {}
    }
}

fn draw_delete_popup<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect, block: Block) {
    f.render_widget(Clear, area);
    f.render_widget(block, area);
    draw_popup_choices(f, app, area)
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

fn draw_popup_choices<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let position = relative_position(60, 60, area);
    let list = Table::new(vec![Row::new(vec!["Cancel"]), Row::new(vec!["Confirm"])])
        .widths(&[Constraint::Length(15), Constraint::Length(15)])
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, position, &mut app.popup.state);
}



fn relative_position(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(percent_y),
                Constraint::Percentage(percent_y),
            ]
            .as_ref(),
        )
        .split(area);
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(percent_x),
                Constraint::Percentage(percent_x),
            ]
            .as_ref(),
        )
        .split(vertical[1])[1];
    horizontal
}
 