use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::engine::TuskEngine;

pub struct App {
    pub engine: Option<TuskEngine>,
    pub input: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            engine: None,
            input: String::new(),
        }
    }
}

pub const COMMANDS: &[&str] = &["int(", "sin(", "cos(", "exp(", "ln("];

pub fn get_suggestion(input: &str) -> Option<&'static str> {
    if input.is_empty() {
        return None;
    }
    
    // Find the last alphabetic word being typed
    let rev_word: String = input.chars().rev().take_while(|c| c.is_alphabetic()).collect();
    let word: String = rev_word.chars().rev().collect();
    
    if word.is_empty() {
        return None;
    }

    for &cmd in COMMANDS {
        if cmd.starts_with(&word) && cmd != word {
            let suffix = &cmd[word.len()..];
            return Some(suffix);
        }
    }
    None
}

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.area());

    let suggestion = get_suggestion(&app.input).unwrap_or("");
    let input_line = Line::from(vec![
        Span::styled(app.input.as_str(), Style::default().fg(Color::Yellow)),
        Span::styled(suggestion, Style::default().fg(Color::DarkGray)),
    ]);

    let input_display = Paragraph::new(input_line)
        .block(Block::default().borders(Borders::ALL).title(" Tusk Input (.tk) [Press Tab to Autocomplete] "));

    f.render_widget(input_display, chunks[0]);

    let steps_content = if let Some(engine) = &app.engine {
        if engine.steps.is_empty() {
            format!("Parsed: {:?}", engine.current_expr)
        } else {
            let mut s = String::new();
            for (i, step) in engine.steps.iter().enumerate() {
                s.push_str(&format!("Step {}: {}\n", i + 1, step.transformation.description));
            }
            s
        }
    } else {
        "Type an expression to parse...".to_string()
    };

    let steps_display = Paragraph::new(steps_content)
        .block(Block::default().borders(Borders::ALL).title(" Integration Steps "));

    f.render_widget(steps_display, chunks[1]);
}
