use std::{io, time::{Duration, Instant}};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
};

pub struct App {
pub tasks: Vec<String>,
pub selected: usize,
pub total_secs: u64,
pub left_secs: u64,
pub running: bool,
pub last_tick: Instant,
}

impl App {
    pub fn new() -> Self {
    Self {
    tasks: vec![
    "Learn Rust ownership".into(),
    "Build TUI skeleton".into(),
    "Implement Pomodoro".into(),
    ],
    selected: 0,
    total_secs: 25 * 60,
    left_secs: 25 * 60,
    running: false,
    last_tick: Instant::now(),
    }
    }
    
    pub fn on_tick(&mut self) {
    if self.running && self.left_secs > 0 && self.last_tick.elapsed() >= Duration::from_secs(1) {
    self.left_secs -= 1;
    self.last_tick = Instant::now();
    }
    }
    
    pub fn progress(&self) -> f64 {
    if self.total_secs == 0 { return 0.0; }
    1.0 - (self.left_secs as f64 / self.total_secs as f64)
    }
    
    pub fn time_text(&self) -> String {
    let m = self.left_secs / 60;
    let s = self.left_secs % 60;
    format!("{:02}:{:02}", m, s)
    }
    }
    
    pub fn run_tui() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut app = App::new();
    let tick_rate = Duration::from_millis(100);
    
    loop {
    terminal.draw(|f| ui(f, &app))?;
    
    if event::poll(tick_rate)? {
    if let Event::Key(key) = event::read()? {
    match key.code {
    KeyCode::Char('q') => break,
    KeyCode::Up => {
    if app.selected > 0 { app.selected -= 1; }
    }
    KeyCode::Down => {
    if app.selected + 1 < app.tasks.len() { app.selected += 1; }
    }
    KeyCode::Char(' ') => app.running = !app.running, // start/pause
    KeyCode::Char('r') => {
    app.left_secs = app.total_secs;
    app.running = false;
    }
    _ => {}
    }
    }
    }
    
    app.on_tick();
    }
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
    }
    
    fn ui(f: &mut Frame, app: &App) {
    let root = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(5), Constraint::Length(3)])
    .split(f.area());
    
    let top = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
    .split(root[0]);
    
    // 左：任务列表
    let items: Vec<ListItem> = app.tasks.iter().enumerate().map(|(i, t)| {
    let prefix = if i == app.selected { "➤ " } else { " " };
    ListItem::new(format!("{prefix}{t}"))
    }).collect();
    
    let task_list = List::new(items).block(Block::default().title("Tasks").borders(Borders::ALL));
    f.render_widget(task_list, top[0]);
    
    // 右：番茄钟
    let timer_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(1)])
    .split(top[1]);
    
    let timer_text = Paragraph::new(format!(
    "Pomodoro: {} [{}]",
    app.time_text(),
    if app.running { "Running" } else { "Paused" }
    ))
    .block(Block::default().title("Timer").borders(Borders::ALL));
    f.render_widget(timer_text, timer_layout[0]);
    
    let gauge = Gauge::default()
    .block(Block::default().title("Progress").borders(Borders::ALL))
    .ratio(app.progress());
    f.render_widget(gauge, timer_layout[1]);
    
    // 下：快捷键
    let help = Paragraph::new("q: quit | ↑/↓: select task | space: start/pause | r: reset")
    .block(Block::default().title("Keys").borders(Borders::ALL));
    f.render_widget(help, root[1]);
    }