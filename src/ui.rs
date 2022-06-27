use crate::app::{App, PopupType};
use console_engine::{pixel::pxl_fg, rect_style::BorderStyle, Color, ConsoleEngine};
use std::collections::HashMap;

pub fn draw(app: &App, engine: &mut ConsoleEngine, height: i32, width: i32) {
    draw_files(app, engine, height as usize);
    engine.rect_border(0, 0, width + 2, height + 2, BorderStyle::new_light());
    engine.print(2, 0, app.title);
    engine.print(15, 0, &app.path.to_string());
    engine.print(
        width - 50,
        0,
        &format!(
            "height: {}, width: {}",
            engine.get_height(),
            engine.get_width()
        ),
    );
    engine.print(width - 12, 0, &format!("Frame: {}", engine.frame_count));

    if app.popup.visible {
        draw_popup(app, engine, height, width)
    }
}

fn draw_files(app: &App, engine: &mut ConsoleEngine, height: usize) {
    let diff: usize;
    // If user curser goes beyond the border
    if app.files.state.selected > height {
        diff = app.files.state.selected - height - 1;
        for a in diff..app.files.files.len() {
            if app.files.state.selected == a {
                engine.set_pxl(1, (height + 1) as i32, pxl_fg('>', Color::Cyan));
            }
            engine.print(3, (a - diff) as i32, &app.files.files[a].name)
        }
    } else {
        for a in 0..app.files.files.len() {
            if app.files.state.selected == a {
                engine.set_pxl(1, (a + 1) as i32, pxl_fg('>', Color::Cyan));
            }
            engine.print(3, (a + 1) as i32, &app.files.files[a].name)
        }
    }
}

fn draw_popup(app: &App, engine: &mut ConsoleEngine, height: i32, width: i32) {
    // TODO: padding instead of borders for draw_popup_input?
    let borders = calculate_popup_borders(height, width, 6, 34);
    if let PopupType::RenameFile = app.popup.popup_type {
        draw_popup_input(app, engine, &borders);
    }
    engine.rect_border(
        borders["start_x"],
        borders["start_y"],
        borders["end_x"],
        borders["end_y"],
        BorderStyle::new_light(),
    );
    engine.print(borders["start_x"] + 3, borders["start_y"], app.popup.title);
    draw_popup_choices(app, engine, borders);
}

fn draw_popup_input(app: &App, engine: &mut ConsoleEngine, borders: &HashMap<&'static str, i32>) {
    engine.line(borders["start_x"]+3, borders["start_y"]+2, borders["end_x"]-3, borders["start_y"]+2, pxl_fg('_', Color::White));
    engine.print(borders["start_x"]+3, borders["start_y"]+2, app.popup.input.as_ref().unwrap());
}

fn draw_popup_choices(app: &App, engine: &mut ConsoleEngine, borders: HashMap<&'static str, i32>) {
    for a in 0..=1 {
        if app.popup.state.selected == a {
            engine.set_pxl(
                borders["end_x"] - 11,
                borders["end_y"] - 2 + (a as i32),
                pxl_fg('>', Color::Cyan),
            );
        }
        engine.print(
            borders["end_x"] - 9,
            borders["end_y"] - 2 + (a as i32),
            app.popup.choices[a],
        );
    }
}

fn calculate_popup_borders(
    height: i32,
    width: i32,
    popup_height: i32,
    popup_width: i32,
) -> HashMap<&'static str, i32> {
    let x_center = width / 2;
    let y_center = height / 2;
    HashMap::from([
        ("start_x", x_center - popup_width / 2),
        ("start_y", y_center - popup_height / 2),
        ("end_x", x_center + popup_width / 2),
        ("end_y", y_center + popup_height / 2),
    ])
}
