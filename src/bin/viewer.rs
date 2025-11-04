use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use palace_skills::*;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

#[derive(Debug, Clone)]
struct MaskVersion {
    version: u32,
    timestamp: String,
    score: Option<f64>,
    changes: String,
}

struct App {
    masks: Vec<(String, String)>, // (specialty, model)
    selected_mask: usize,
    mask_history: Vec<MaskVersion>,
    view_mode: ViewMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ViewMode {
    MaskList,
    VersionHistory,
    ScoreGraph,
    Comparison,
}

impl App {
    fn new() -> Result<Self> {
        let masks = list_masks()?;
        Ok(Self {
            masks,
            selected_mask: 0,
            mask_history: Vec::new(),
            view_mode: ViewMode::MaskList,
        })
    }

    fn load_selected_mask_history(&mut self) -> Result<()> {
        if self.masks.is_empty() {
            return Ok(());
        }

        let (specialty, model) = &self.masks[self.selected_mask];
        let mask = load_mask_from_file(specialty, model)?;

        // Parse version history from mask content
        self.mask_history = parse_version_history(&mask.content);

        Ok(())
    }

    fn next_mask(&mut self) {
        if !self.masks.is_empty() {
            self.selected_mask = (self.selected_mask + 1) % self.masks.len();
            let _ = self.load_selected_mask_history();
        }
    }

    fn prev_mask(&mut self) {
        if !self.masks.is_empty() {
            if self.selected_mask == 0 {
                self.selected_mask = self.masks.len() - 1;
            } else {
                self.selected_mask -= 1;
            }
            let _ = self.load_selected_mask_history();
        }
    }

    fn cycle_view(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::MaskList => ViewMode::VersionHistory,
            ViewMode::VersionHistory => ViewMode::ScoreGraph,
            ViewMode::ScoreGraph => ViewMode::Comparison,
            ViewMode::Comparison => ViewMode::MaskList,
        };
    }
}

fn parse_version_history(content: &str) -> Vec<MaskVersion> {
    let mut versions = Vec::new();
    let mut in_improvement_notes = false;
    let mut current_version: Option<u32> = None;
    let mut current_timestamp = String::new();
    let mut current_changes = String::new();

    for line in content.lines() {
        if line.contains("## Improvement Notes") {
            in_improvement_notes = true;
            continue;
        }

        if in_improvement_notes {
            if line.starts_with("### Version") {
                // Save previous version if exists
                if let Some(ver) = current_version {
                    versions.push(MaskVersion {
                        version: ver,
                        timestamp: current_timestamp.clone(),
                        score: None, // TODO: Parse from benchmark results
                        changes: current_changes.trim().to_string(),
                    });
                }

                // Parse new version header
                // Format: "### Version 2 (2025-11-04 23:45) - Description"
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(ver) = parts[2].parse::<u32>() {
                        current_version = Some(ver);

                        // Extract timestamp if present
                        if let Some(ts_start) = line.find('(') {
                            if let Some(ts_end) = line.find(')') {
                                current_timestamp = line[ts_start + 1..ts_end].to_string();
                            }
                        }

                        current_changes = String::new();
                    }
                }
            } else if !line.trim().is_empty() && !line.starts_with("<!--") {
                current_changes.push_str(line);
                current_changes.push('\n');
            }
        }
    }

    // Save last version
    if let Some(ver) = current_version {
        versions.push(MaskVersion {
            version: ver,
            timestamp: current_timestamp,
            score: None,
            changes: current_changes.trim().to_string(),
        });
    }

    versions
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new()?;
    if !app.masks.is_empty() {
        app.load_selected_mask_history()?;
    }

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('j') | KeyCode::Down => app.next_mask(),
                KeyCode::Char('k') | KeyCode::Up => app.prev_mask(),
                KeyCode::Tab => app.cycle_view(),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Header
    let header = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            "Palace Mask Viewer",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" - "),
        Span::styled(
            format!("{:?}", app.view_mode),
            Style::default().fg(Color::Yellow),
        ),
    ])])
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Main content
    match app.view_mode {
        ViewMode::MaskList => render_mask_list(f, app, chunks[1]),
        ViewMode::VersionHistory => render_version_history(f, app, chunks[1]),
        ViewMode::ScoreGraph => render_score_graph(f, app, chunks[1]),
        ViewMode::Comparison => render_comparison(f, app, chunks[1]),
    }

    // Footer
    let footer = Paragraph::new("↑/↓ or j/k: Navigate | Tab: Change View | q: Quit")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn render_mask_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .masks
        .iter()
        .enumerate()
        .map(|(i, (specialty, model))| {
            let style = if i == app.selected_mask {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(format!("  {}/{}", specialty, model)).style(style)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Available Masks"),
    );

    f.render_widget(list, area);
}

fn render_version_history(f: &mut Frame, app: &App, area: Rect) {
    if app.masks.is_empty() {
        let para = Paragraph::new("No masks available")
            .block(Block::default().borders(Borders::ALL).title("Version History"));
        f.render_widget(para, area);
        return;
    }

    let (specialty, model) = &app.masks[app.selected_mask];

    let mut lines = vec![
        Line::from(vec![
            Span::styled("Mask: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{}/{}", specialty, model),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
    ];

    if app.mask_history.is_empty() {
        lines.push(Line::from(Span::styled(
            "No version history found in mask",
            Style::default().fg(Color::DarkGray),
        )));
    } else {
        for version in &app.mask_history {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("v{}", version.version),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - "),
                Span::styled(&version.timestamp, Style::default().fg(Color::Blue)),
            ]));

            if let Some(score) = version.score {
                lines.push(Line::from(vec![
                    Span::raw("  Score: "),
                    Span::styled(
                        format!("{:.2}", score),
                        Style::default().fg(Color::Magenta),
                    ),
                ]));
            }

            // Show first line of changes
            if let Some(first_line) = version.changes.lines().next() {
                let preview = if first_line.len() > 60 {
                    format!("  {}...", &first_line[..60])
                } else {
                    format!("  {}", first_line)
                };
                lines.push(Line::from(Span::styled(
                    preview,
                    Style::default().fg(Color::DarkGray),
                )));
            }

            lines.push(Line::from(""));
        }
    }

    let para = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Version History"),
    );

    f.render_widget(para, area);
}

fn render_score_graph(f: &mut Frame, app: &App, area: Rect) {
    if app.masks.is_empty() {
        let para = Paragraph::new("No masks available")
            .block(Block::default().borders(Borders::ALL).title("Score Evolution"));
        f.render_widget(para, area);
        return;
    }

    let (specialty, model) = &app.masks[app.selected_mask];

    // Extract data points from version history
    let data_points: Vec<(f64, f64)> = app
        .mask_history
        .iter()
        .filter_map(|v| v.score.map(|s| (v.version as f64, s)))
        .collect();

    if data_points.is_empty() {
        let para = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Mask: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{}/{}", specialty, model),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "No benchmark scores recorded yet",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from("Run benchmarks to generate score data"),
        ])
        .block(Block::default().borders(Borders::ALL).title("Score Evolution"));
        f.render_widget(para, area);
        return;
    }

    let max_version = app.mask_history.iter().map(|v| v.version).max().unwrap_or(1) as f64;

    let dataset = Dataset::default()
        .name(format!("{}/{}", specialty, model))
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(Color::Cyan))
        .data(&data_points);

    let chart = Chart::new(vec![dataset])
        .block(Block::default().borders(Borders::ALL).title("Score Evolution"))
        .x_axis(
            Axis::default()
                .title("Version")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, max_version + 1.0])
                .labels(vec![
                    Span::raw("v0"),
                    Span::raw(format!("v{}", max_version as u32)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Score")
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, 10.0])
                .labels(vec![Span::raw("0.0"), Span::raw("5.0"), Span::raw("10.0")]),
        );

    f.render_widget(chart, area);
}

fn render_comparison(f: &mut Frame, _app: &App, area: Rect) {
    let para = Paragraph::new(vec![
        Line::from(Span::styled(
            "Comparison View",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Coming soon: Compare multiple masks side-by-side",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from("Features:"),
        Line::from("  - Version-to-version diff"),
        Line::from("  - Score comparison across masks"),
        Line::from("  - Improvement velocity metrics"),
    ])
    .block(Block::default().borders(Borders::ALL).title("Comparison"));

    f.render_widget(para, area);
}
