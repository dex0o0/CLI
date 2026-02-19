use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment,Constraint,Layout},
    widgets::{Block,Borders,Row,Table,},
    Frame,Terminal
};
use crossterm::{
    ExecutableCommand, event::{self,DisableMouseCapture,EnableMouseCapture,Event,KeyCode}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}

};
use tabled::grid::config::Border;
use std::{io,thread,time::Duration};
use super::scan_sys::Sysinfo;

pub struct TuiApp;

impl TuiApp{
    pub fn show_status(){
        enable_raw_mode().expect("Error from enable raw mod");
        let mut stdout= io::stdout();
        execute!(stdout,EnterAlternateScreen,EnableMouseCapture).expect("Error");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("Error");
        let mut data = Sysinfo::new();
        data.auto_fill().expect("Error");
        let vec = data.data_vec();
            
            terminal.draw(|f| Self::ui(vec, f)).expect("Error");
        std::thread::sleep(std::time::Duration::from_secs(5));
        disable_raw_mode().expect("Error from disable raw mod");
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        ).expect("Error");
        terminal.show_cursor().unwrap();

    }
    fn ui(data:Vec<(String,String)>,f:&mut Frame){
        let chunks = Layout::vertical([Constraint::Length(3),Constraint::Min(1)]).split(
            f.size()
        );
        let title = Block::default()
        .borders(Borders::ALL)
        .title("status")
        .title_alignment(Alignment::Left);
        f.render_widget(title, chunks[0]);

        let widths = [Constraint::Percentage(30),Constraint::Percentage(70)];
        let row:Vec<Row>=data.iter()
        .map(|(k,v)| Row::new(vec![k.to_string(),v.to_string()]))
        .collect();
        
        let table = Table::new(row, widths)
        .block(Block::default().borders(Borders::ALL))
        .header(Row::new(vec!["Key","Value"]))
        .style(ratatui::style::Style::default().add_modifier(ratatui::style::Modifier::BOLD))
        .column_spacing(1)
        .widths(&[Constraint::Percentage(30),Constraint::Percentage(70)]);
        
        f.render_widget(table, chunks[1]);

    }
}
