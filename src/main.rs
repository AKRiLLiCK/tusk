mod ast;
mod calculus;
mod engine;
mod heuristics;
mod risch;

#[cfg(not(target_arch = "wasm32"))]
mod ui;

#[cfg(not(target_arch = "wasm32"))]
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[cfg(not(target_arch = "wasm32"))]
use ratatui::{backend::CrosstermBackend, Terminal};
#[cfg(not(target_arch = "wasm32"))]
use std::io;
#[cfg(not(target_arch = "wasm32"))]
use ui::App;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let res = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = res { eprintln!("{e:?}"); }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press { continue; }

            match key.code {
                KeyCode::Char(c) => {
                    app.input.push(c);
                    app.reparse();
                }
                KeyCode::Backspace => {
                    app.input.pop();
                    app.reparse();
                }
                KeyCode::Tab => {
                    if let Some(suffix) = ui::get_suggestion(&app.input) {
                        app.input.push_str(suffix);
                        app.reparse();
                    }
                }
                KeyCode::Up => {
                    if app.selected_step > 0 {
                        app.selected_step -= 1;
                    }
                }
                KeyCode::Down => {
                    let max = app.engine.as_ref().map_or(0, |e| e.steps.len());
                    if app.selected_step < max {
                        app.selected_step += 1;
                    }
                }
                KeyCode::Esc => return Ok(()),
                _ => {}
            }
        }
    }
}