use std::{io, time::Duration};

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    Frame, Terminal, backend::CrosstermBackend, layout::{Alignment, Constraint, Layout}, style::{Color, Modifier, Style}, symbols::line::BOTTOM_LEFT, text::Line, widgets::{Block, Borders, Cell, Paragraph, Row, Table, TitlePosition}
};

use super::scan_sys::Sysinfo;

pub struct TuiApp;

struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalGuard {
    fn new() -> Result<Self> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        );
        let _ = self.terminal.show_cursor();
    }
}

impl TuiApp {
    pub fn show_status() -> Result<()> {
        let mut terminal_guard = TerminalGuard::new()?;
        let tick_rate = Duration::from_secs(2);
        let mut data = Self::load_data();

        loop {
            terminal_guard.terminal.draw(|f| Self::ui(&data, f))?;

            if event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press
                        && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                    {
                        break;
                    }
                }
            } else {
                data = Self::load_data();
            }
        }

        Ok(())
    }

    fn load_data() -> Vec<(String, String)> {
        let mut data = Sysinfo::new();
        if data.auto_fill().is_err() {
            return vec![(
                "error".to_string(),
                "failed to load system information".to_string(),
            )];
        }
        data.data_vec()
    }

    fn ui(data: &[(String, String)], f: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(1),
            Constraint::Length(4),
        ])
        .split(f.area());

        let title = Block::default()
            .borders(Borders::ALL)
            .title(" System Status ")
            .title_alignment(Alignment::Center)
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

        // let footer = Paragraph::new("[exit<Q>]")
        //     .style(Style::default().fg(Color::Green))
        //     .alignment(Alignment::Left)
        //     .block(Block::default().borders(Borders::ALL));
        let footer = Block::default()
        .borders(Borders::ALL)
        .title("[<Q>exit]")
        .title(Line::from("[Refresh at 2s]").left_aligned())
        .border_style(Style::default().fg(Color::Blue))
        .title_alignment(Alignment::Center)
        .title_style(Style::default().fg(Color::White));
        
        f.render_widget(footer, chunks[2]);
}
}
