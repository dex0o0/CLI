use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame, Terminal,
};

use super::scan_sys::Sysinfo;

pub struct TuiApp;

impl TuiApp {
    pub fn show_status() {
        enable_raw_mode().expect("Error from enable raw mode");

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("Error");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Error");

        let tick_rate = Duration::from_secs(2);
        let mut last_tick = Instant::now();

        loop {
            let data = Self::load_data();
            terminal.draw(|f| Self::ui(&data, f)).expect("Error");

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout).expect("event poll failed") {
                if let Event::Key(key) = event::read().expect("event read failed") {
                    if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                        break;
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }

        disable_raw_mode().expect("Error from disable raw mode");
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )
        .expect("Error");
        terminal.show_cursor().expect("show cursor failed");
    }

    fn load_data() -> Vec<(String, String)> {
        let mut data = Sysinfo::new();
        data.auto_fill().expect("Error");
        data.data_vec()
    }

    fn ui(data: &[(String, String)], f: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(f.area());

        let title = Block::default()
            .borders(Borders::ALL)
            .title(" System Status ")
            .title_alignment(Alignment::Left)
            .border_style(Style::default().fg(Color::Cyan));
        f.render_widget(title, chunks[0]);

        let header = Row::new(vec!["Key", "Value"]).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

        let rows: Vec<Row> = data
            .iter()
            .enumerate()
            .map(|(i, (k, v))| {
                let row_style = if i % 2 == 0 {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::Gray)
                };

                Row::new(vec![Cell::from(k.as_str()), Cell::from(v.as_str())]).style(row_style)
            })
            .collect();

        let table = Table::new(
            rows,
            [Constraint::Percentage(28), Constraint::Percentage(72)],
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Hardware Info "),
        )
        .header(header)
        .column_spacing(2);

        f.render_widget(table, chunks[1]);

        let footer = Paragraph::new("Auto refresh: 2s  |  Press q or Esc to exit")
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[2]);
    }
}
