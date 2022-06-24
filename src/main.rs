mod app;
use crate::app::App;
mod ui;
use crate::ui::draw;
use console_engine::{events::Event, KeyCode};
use crossterm::terminal;

fn main() {
    let mut engine = console_engine::ConsoleEngine::init_fill(30).unwrap();
    let mut app = App::new("vecile");
    let mut height: u32;
    let mut width: u32;

    while !app.should_quit {
        (width, height) = convert_size_type(terminal::size().unwrap());
        // I noticed ~2x decrease in CPU usage with this condition
        if width != engine.get_width() || height != engine.get_height() {
            engine.resize(width, height);
        }
        match engine.poll() {
            Event::Frame => {
                engine.clear_screen();
                draw(&app, &mut engine, (height - 3) as i32, (width - 3) as i32);
                engine.draw();
            }
            Event::Key(keyevent) => match keyevent.code {
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Down => app.on_down(),
                KeyCode::Up => app.on_up(),
                KeyCode::Left => app.on_left(),
                KeyCode::Right => app.on_right(),
                KeyCode::Enter => app.on_enter(),
                KeyCode::Esc => app.on_esc(),
                _ => {}
            },
            _ => {}
        }
    }
}

fn convert_size_type(tuple: (u16, u16)) -> (u32, u32) {
    (tuple.0 as u32, tuple.1 as u32)
}
