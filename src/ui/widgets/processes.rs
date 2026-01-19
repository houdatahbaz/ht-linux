use crate::app::App;
use crate::system::format_bytes;
use ratatui::{
    prelude::*,
    widgets::{Cell, Paragraph, Row, Table},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, scroll_offset: u16) {
    let processes = &app.system_data.processes;

    if processes.is_empty() {
        let paragraph = Paragraph::new("No processes found");
        frame.render_widget(paragraph, area);
        return;
    }

    // Header
    let header = Row::new(vec![
        Cell::from("PID").style(Style::default().bold().fg(Color::Cyan)),
        Cell::from("Name").style(Style::default().bold().fg(Color::Cyan)),
        Cell::from("CPU %").style(Style::default().bold().fg(Color::Cyan)),
        Cell::from("Memory").style(Style::default().bold().fg(Color::Cyan)),
        Cell::from("Status").style(Style::default().bold().fg(Color::Cyan)),
    ])
    .height(1);

    // Calculate visible range
    let visible_height = area.height.saturating_sub(2) as usize; // account for header and borders
    let offset = scroll_offset as usize;
    let end = (offset + visible_height).min(processes.len());
    let visible_processes = &processes[offset..end];

    let rows: Vec<Row> = visible_processes
        .iter()
        .map(|proc| {
            let cpu_style = if proc.cpu_usage >= 50.0 {
                Style::default().fg(Color::Red)
            } else if proc.cpu_usage >= 20.0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            };

            Row::new(vec![
                Cell::from(proc.pid.to_string()),
                Cell::from(truncate_string(&proc.name, 25)),
                Cell::from(format!("{:.1}", proc.cpu_usage)).style(cpu_style),
                Cell::from(format_bytes(proc.memory)),
                Cell::from(proc.status.clone()),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(8),
        Constraint::Min(20),
        Constraint::Length(8),
        Constraint::Length(12),
        Constraint::Length(10),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(Style::default().bg(Color::DarkGray));

    frame.render_widget(table, area);

    // Show scroll indicator if there are more items
    if processes.len() > visible_height {
        let indicator = format!(
            " [{}-{}/{}] Use j/k to scroll ",
            offset + 1,
            end,
            processes.len()
        );
        let indicator_widget = Paragraph::new(indicator)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Right);

        let indicator_area = Rect {
            x: area.x,
            y: area.y + area.height.saturating_sub(1),
            width: area.width,
            height: 1,
        };
        frame.render_widget(indicator_widget, indicator_area);
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
