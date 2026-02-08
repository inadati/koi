use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::utils::error::{KoiError, Result};

const MAX_VISIBLE: usize = 15;

struct FuzzySelectState {
    query: String,
    selected: usize,
    items: Vec<String>,
}

impl FuzzySelectState {
    fn new(items: Vec<String>) -> Self {
        Self {
            query: String::new(),
            selected: 0,
            items,
        }
    }

    fn filtered(&self) -> Vec<&String> {
        if self.query.is_empty() {
            return self.items.iter().collect();
        }
        let matcher = SkimMatcherV2::default();
        let mut scored: Vec<(&String, i64)> = self
            .items
            .iter()
            .filter_map(|item| {
                matcher
                    .fuzzy_match(item, &self.query)
                    .map(|score| (item, score))
            })
            .collect();
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored.into_iter().map(|(item, _)| item).collect()
    }

    fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn move_down(&mut self, max: usize) {
        if max > 0 && self.selected < max - 1 {
            self.selected += 1;
        }
    }
}

pub fn select_from_list(items: &[String], prompt_msg: &str) -> Result<String> {
    if items.is_empty() {
        return Err(KoiError::SkillNotFound("no items available".to_string()));
    }

    let mut state = FuzzySelectState::new(items.to_vec());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, &mut state, prompt_msg);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut FuzzySelectState,
    prompt_msg: &str,
) -> Result<String> {
    loop {
        let filtered = state.filtered();
        let filtered_len = filtered.len();

        terminal.draw(|f| {
            render(f, state, &filtered, prompt_msg);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Esc => return Err(KoiError::Cancelled),
                KeyCode::Enter => {
                    let filtered = state.filtered();
                    if let Some(item) = filtered.get(state.selected) {
                        return Ok((*item).clone());
                    }
                    return Err(KoiError::Cancelled);
                }
                KeyCode::Up => state.move_up(),
                KeyCode::Down => state.move_down(filtered_len),
                KeyCode::Backspace => {
                    state.query.pop();
                    state.selected = 0;
                }
                KeyCode::Char(c) => {
                    state.query.push(c);
                    state.selected = 0;
                }
                _ => {}
            }
        }
    }
}

fn render(
    f: &mut Frame,
    state: &FuzzySelectState,
    filtered: &[&String],
    prompt_msg: &str,
) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // prompt
            Constraint::Length(1), // input
            Constraint::Length(1), // separator
            Constraint::Min(1),   // list
            Constraint::Length(1), // help
        ])
        .split(area);

    // Prompt
    let prompt = Paragraph::new(prompt_msg)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
    f.render_widget(prompt, chunks[0]);

    // Input
    let input_text = format!("> {}_", state.query);
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(input, chunks[1]);

    // Separator
    let sep = Paragraph::new("─".repeat(area.width as usize))
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(sep, chunks[2]);

    // List
    let visible_count = filtered.len().min(MAX_VISIBLE);
    let list_items: Vec<ListItem> = filtered
        .iter()
        .take(MAX_VISIBLE)
        .enumerate()
        .map(|(i, item)| {
            let prefix = if i == state.selected { "● " } else { "  " };
            let style = if i == state.selected {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Rgb(40, 40, 80))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(format!("{}{}", prefix, item)).style(style)
        })
        .collect();

    let list = List::new(list_items);
    f.render_widget(list, chunks[3]);

    // Help
    let count_info = if filtered.len() > visible_count {
        format!(" ({}/{})", visible_count, filtered.len())
    } else {
        format!(" ({})", filtered.len())
    };
    let help = Paragraph::new(format!(
        "↑↓: 移動  Enter: 決定  Esc: 取消{}",
        count_info
    ))
    .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, chunks[4]);
}
