mod app;
use crate::app::App;
mod ui;
use crate::ui::draw;
use crossterm::{
    event::{read, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use std::{error::Error, io::{stdout, Stdout}};
use tui::{backend::CrosstermBackend, Terminal};
use rustbox::{RustBox, Key};

#[derive(Debug)]
struct Cli {
    /// whether unicode symbols are used to improve the overall look of the app
    enhanced_graphics: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli {
        enhanced_graphics: true,
    };

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new("Vecile", cli.enhanced_graphics);

    // Rustbox is used here only for keyboard input, as crossterm didn't work for me when using kitty terminal emulator on manjaro

    terminal.clear()?;
    if cfg!(windows) {
        run_on_windows(terminal, &mut app).ok();
    } else if cfg!(unix) {
        run_on_unix(terminal, &mut app).ok();
    }

    disable_raw_mode()?;
    Ok(())
}

fn run_on_windows(mut terminal: Terminal<CrosstermBackend<Stdout>>, mut app: &mut App) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| draw(f, &mut app))?;
        if let Event::Key(event ) = read()? { 
            match event.code {        
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Left => app.on_left(),
                KeyCode::Up => app.on_up(),
                KeyCode::Right => app.on_right(),
                KeyCode::Down => app.on_down(),
                KeyCode::Enter => app.on_enter(),
                KeyCode::Esc => app.on_esc(),
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn run_on_unix(mut terminal: Terminal<CrosstermBackend<Stdout>>, mut app: &mut App) -> Result<(), std::io::Error> { 
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };
    loop {
        terminal.draw(|f| draw(f, &mut app))?;
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char(c) => app.on_key(c),
                    Key::Left => app.on_left(),
                    Key::Up => app.on_up(),
                    Key::Right => app.on_right(),
                    Key::Down => app.on_down(),
                    Key::Enter => app.on_enter(),
                    Key::Esc => app.on_esc(),
                    _ => { }
                }
            },
            _ => { }
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}