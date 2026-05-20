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
    pub selected_step: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            engine: None,
            input: String::new(),
            selected_step: 0,
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
        .constraints([
            Constraint::Length(3), // Input box
            Constraint::Length(6), // AST time-travel state
            Constraint::Min(1),    // Steps
        ])
        .split(f.area());

    let suggestion = get_suggestion(&app.input).unwrap_or("");
    let input_line = Line::from(vec![
        Span::styled(app.input.as_str(), Style::default().fg(Color::Yellow)),
        Span::styled(suggestion, Style::default().fg(Color::DarkGray)),
    ]);

    let input_display = Paragraph::new(input_line)
        .block(Block::default().borders(Borders::ALL).title(" Tusk Input (.tk) [Press Tab to Autocomplete] "));

    f.render_widget(input_display, chunks[0]);

    let current_expr_str = if let Some(engine) = &app.engine {
        if app.selected_step < engine.steps.len() {
            format!("{:#?}", engine.steps[app.selected_step].initial_state)
        } else {
            format!("{:#?}", engine.current_expr)
        }
    } else {
        "No valid expression parsed.".to_string()
    };

    let state_display = Paragraph::new(current_expr_str)
        .block(Block::default().borders(Borders::ALL).title(" Time-Travel State (AST) "));
    f.render_widget(state_display, chunks[1]);

    if let Some(engine) = &app.engine {
        use ratatui::widgets::{List, ListItem};
        
        let items: Vec<ListItem> = engine.steps.iter().enumerate().map(|(i, step)| {
            let prefix = if i == app.selected_step { ">> " } else { "   " };
            let content = format!("{}{}: {:?}", prefix, step.transformation.description, step.transformation.rule);
            let style = if i == app.selected_step { Style::default().fg(Color::Cyan) } else { Style::default() };
            ListItem::new(content).style(style)
        }).collect();

        let final_prefix = if app.selected_step == engine.steps.len() { ">> " } else { "   " };
        let final_style = if app.selected_step == engine.steps.len() { Style::default().fg(Color::Cyan) } else { Style::default() };
        let mut all_items = items;
        all_items.push(ListItem::new(format!("{}Final Result", final_prefix)).style(final_style));

        let list = List::new(all_items)
            .block(Block::default().borders(Borders::ALL).title(" Transformation Steps [Up/Down to Time-Travel] "));
        
        f.render_widget(list, chunks[2]);
    }
}
