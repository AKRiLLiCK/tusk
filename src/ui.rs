use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
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

    let input_display = Paragraph::new(app.input.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title(" Tusk Input (.tk) "));

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
