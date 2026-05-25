use crate::engine::TuskEngine;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

const CLR_BG: Color = Color::Rgb(18, 18, 24);
const CLR_BORDER: Color = Color::Rgb(58, 58, 80);
const CLR_TITLE: Color = Color::Rgb(180, 140, 255);
const CLR_INPUT: Color = Color::Rgb(255, 220, 100);
const CLR_GHOST: Color = Color::Rgb(80, 80, 100);
const CLR_AST: Color = Color::Rgb(120, 220, 180);
const CLR_STEP: Color = Color::Rgb(160, 160, 180);
const CLR_ACTIVE: Color = Color::Rgb(100, 180, 255);
const CLR_LABEL: Color = Color::Rgb(255, 130, 100);
const CLR_DIM: Color = Color::Rgb(100, 100, 120);

const GHOST_EXAMPLES: &[&str] = &[
    "integral(x*cos(x); y^2 of d x from 1 to 10)",
    "integral(x^2 + 2*x of d x)",
    "sin(x) * exp(x)",
    "integral(1/x of d x)",
];

fn themed_block(title: &str) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(CLR_BORDER))
        .title(Span::styled(
            format!(" {title} "),
            Style::default().fg(CLR_TITLE).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(CLR_BG))
}

pub struct App {
    pub engine: Option<TuskEngine>,
    pub input: String,
    pub selected_step: usize,
    pub active_ghost: usize,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            engine: None,
            input: String::new(),
            selected_step: 0,
            active_ghost: 0,
        }
    }

    pub fn reparse(&mut self) {
        if self.input.is_empty() {
            self.active_ghost = (self.active_ghost + 1) % GHOST_EXAMPLES.len();
        }

        match crate::ast::Expr::parse(&self.input) {
            Ok(expr) => {
                let mut engine = crate::engine::TuskEngine::new(expr);
                engine.run();
                self.selected_step = engine.steps.len();
                self.engine = Some(engine);
            }
            Err(_) => {
                self.engine = None;
                self.selected_step = 0;
            }
        }
    }
}

const COMMANDS: &[&str] = &["integral(", "sin(", "cos(", "exp(", "ln("];

pub fn get_suggestion(input: &str) -> Option<&'static str> {
    let word: String = input
        .chars()
        .rev()
        .take_while(|c| c.is_alphabetic())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    if word.is_empty() {
        return None;
    }
    COMMANDS
        .iter()
        .find(|cmd| cmd.starts_with(&*word) && **cmd != word)
        .map(|cmd| &cmd[word.len()..])
}

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(f.area());

    let ghost_text = if app.input.is_empty() {
        GHOST_EXAMPLES[app.active_ghost]
    } else {
        get_suggestion(&app.input).unwrap_or("")
    };

    let input_line = Line::from(vec![
        Span::styled(&app.input, Style::default().fg(CLR_INPUT)),
        Span::styled(ghost_text, Style::default().fg(CLR_GHOST)),
    ]);
    f.render_widget(
        Paragraph::new(input_line).block(themed_block("Tusk Input ─ Tab to autocomplete")),
        chunks[0],
    );

    let state_text = match &app.engine {
        Some(engine) if app.selected_step < engine.steps.len() => {
            format!("{}", engine.steps[app.selected_step].initial_state)
        }
        Some(engine) => format!("{}", engine.current_expr),
        None => "".into(),
    };
    f.render_widget(
        Paragraph::new(Span::styled(state_text, Style::default().fg(CLR_AST)))
            .block(themed_block("Expression State")),
        chunks[1],
    );

    if let Some(engine) = &app.engine {
        let total = engine.steps.len();
        let mut items: Vec<ListItem> = engine
            .steps
            .iter()
            .enumerate()
            .map(|(i, step)| {
                let selected = i == app.selected_step;
                let marker = if selected { "▸ " } else { "  " };
                let idx = format!("[{}/{}] ", i + 1, total);
                let line = Line::from(vec![
                    Span::styled(
                        marker,
                        Style::default().fg(if selected { CLR_ACTIVE } else { CLR_DIM }),
                    ),
                    Span::styled(idx, Style::default().fg(CLR_LABEL)),
                    Span::styled(
                        &step.transformation.description,
                        Style::default()
                            .fg(if selected { CLR_ACTIVE } else { CLR_STEP })
                            .add_modifier(if selected {
                                Modifier::BOLD
                            } else {
                                Modifier::empty()
                            }),
                    ),
                ]);
                ListItem::new(line)
            })
            .collect();

        let sel_final = app.selected_step == total;
        items.push(ListItem::new(Line::from(vec![
            Span::styled(
                if sel_final { "▸ " } else { "  " },
                Style::default().fg(if sel_final { CLR_ACTIVE } else { CLR_DIM }),
            ),
            Span::styled(
                "✓ Final Result",
                Style::default()
                    .fg(if sel_final {
                        Color::Rgb(100, 255, 160)
                    } else {
                        CLR_STEP
                    })
                    .add_modifier(if sel_final {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            ),
        ])));

        f.render_widget(
            List::new(items).block(themed_block("Steps ─ ↑↓ to time-travel")),
            chunks[2],
        );
    } else {
        f.render_widget(
            Paragraph::new(Span::styled("No steps yet.", Style::default().fg(CLR_DIM)))
                .block(themed_block("Steps")),
            chunks[2],
        );
    }

    let step_info = match &app.engine {
        Some(e) => format!(
            " Step {}/{} │ Esc to quit",
            app.selected_step + 1,
            e.steps.len() + 1
        ),
        None => " Esc to quit".into(),
    };
    f.render_widget(
        Paragraph::new(Span::styled(step_info, Style::default().fg(CLR_DIM))),
        chunks[3],
    );
}