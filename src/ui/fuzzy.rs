use std::collections::HashSet;
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
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
    multi_select: bool,
    checked: HashSet<usize>,
}

impl FuzzySelectState {
    fn new(items: Vec<String>, multi_select: bool) -> Self {
        Self {
            query: String::new(),
            selected: 0,
            items,
            multi_select,
            checked: HashSet::new(),
        }
    }

    fn filtered(&self) -> Vec<(usize, &String)> {
        if self.query.is_empty() {
            return self.items.iter().enumerate().collect();
        }
        let matcher = SkimMatcherV2::default();
        let mut scored: Vec<(usize, &String, i64)> = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                matcher
                    .fuzzy_match(item, &self.query)
                    .map(|score| (idx, item, score))
            })
            .collect();
        scored.sort_by(|a, b| b.2.cmp(&a.2));
        scored.into_iter().map(|(idx, item, _)| (idx, item)).collect()
    }

    fn toggle_checked(&mut self, original_idx: usize) {
        if self.checked.contains(&original_idx) {
            self.checked.remove(&original_idx);
        } else {
            self.checked.insert(original_idx);
        }
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

    let mut state = FuzzySelectState::new(items.to_vec(), false);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, &mut state, prompt_msg);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    match result {
        Ok(items) => items
            .into_iter()
            .next()
            .ok_or(KoiError::Cancelled),
        Err(e) => Err(e),
    }
}

pub fn select_multiple_from_list(items: &[String], prompt_msg: &str) -> Result<Vec<String>> {
    if items.is_empty() {
        return Err(KoiError::SkillNotFound("no items available".to_string()));
    }

    let mut state = FuzzySelectState::new(items.to_vec(), true);

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
) -> Result<Vec<String>> {
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
                    if state.multi_select && !state.checked.is_empty() {
                        let results: Vec<String> = state
                            .items
                            .iter()
                            .enumerate()
                            .filter(|(idx, _)| state.checked.contains(idx))
                            .map(|(_, item)| item.clone())
                            .collect();
                        return Ok(results);
                    }
                    if let Some((_, item)) = filtered.get(state.selected) {
                        return Ok(vec![(*item).clone()]);
                    }
                    return Err(KoiError::Cancelled);
                }
                KeyCode::Tab if state.multi_select => {
                    let filtered = state.filtered();
                    if let Some((original_idx, _)) = filtered.get(state.selected) {
                        state.toggle_checked(*original_idx);
                        state.move_down(filtered_len);
                    }
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
    filtered: &[(usize, &String)],
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
    let selected_info = if state.multi_select && !state.checked.is_empty() {
        format!("  ({} selected)", state.checked.len())
    } else {
        String::new()
    };
    let input_text = format!("> {}_{}", state.query, selected_info);
    let input = Paragraph::new(input_text).style(Style::default().fg(Color::Yellow));
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
        .map(|(i, (original_idx, item))| {
            let is_cursor = i == state.selected;
            let is_checked = state.multi_select && state.checked.contains(original_idx);

            let prefix = if is_cursor && is_checked {
                "> ● "
            } else if is_cursor {
                ">   "
            } else if is_checked {
                "  ● "
            } else {
                "    "
            };

            let style = if is_cursor {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Rgb(40, 40, 80))
                    .add_modifier(Modifier::BOLD)
            } else if is_checked {
                Style::default()
                    .fg(Color::Green)
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
    let help_text = if state.multi_select {
        format!(
            "↑↓: 移動  Tab: 選択  Enter: 決定  Esc: 取消{}",
            count_info
        )
    } else {
        format!("↑↓: 移動  Enter: 決定  Esc: 取消{}", count_info)
    };
    let help = Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, chunks[4]);
}
