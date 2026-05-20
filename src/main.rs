mod ast;
mod calculus;
mod engine;
mod heuristics;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};
use ui::App;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    app.input.push(c);
                    // Try parsing
                    if let Ok(expr) = ast::Expr::parse(&app.input) {
                        let mut engine = engine::TuskEngine::new(expr);
                        engine.run();
                        app.engine = Some(engine);
                    } else {
                        app.engine = None;
                    }
                }
                KeyCode::Backspace => {
                    app.input.pop();
                    // Try parsing
                    if let Ok(expr) = ast::Expr::parse(&app.input) {
                        let mut engine = engine::TuskEngine::new(expr);
                        engine.run();
                        app.engine = Some(engine);
                    } else {
                        app.engine = None;
                    }
                }
                KeyCode::Tab | KeyCode::Right => {
                    if let Some(suggestion) = ui::get_suggestion(&app.input) {
                        app.input.push_str(suggestion);
                        if let Ok(expr) = ast::Expr::parse(&app.input) {
                            let mut engine = engine::TuskEngine::new(expr);
                            engine.run();
                            app.engine = Some(engine);
                        } else {
                            app.engine = None;
                        }
                    }
                }
                KeyCode::Esc => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}
