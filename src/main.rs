mod app;
use crate::app::App;
mod ui;
use crate::ui::draw;
use crossterm::{
    event::{read, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
//use crossterm_input::{InputEvent, KeyEvent, MouseButton, MouseEvent};
use std::{error::Error, io::stdout};
use tui::{backend::CrosstermBackend, Terminal};

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

    let mut app = App::new("Filemng", cli.enhanced_graphics);

    terminal.clear()?;
    loop {
        terminal.draw(|f| draw(f, &mut app))?;
        match read()? {
            Event::Key(event) => match event.code {        
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Left => app.on_left(),
                KeyCode::Up => app.on_up(),
                KeyCode::Right => app.on_right(),
                KeyCode::Down => app.on_down(),
                KeyCode::Enter => app.on_enter(),
                KeyCode::Esc => app.on_esc(),
                _ => {}
            }
            _ => {}
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    Ok(())
}
