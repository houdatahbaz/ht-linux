use crate::app::App;
use crate::system::format_bytes;
use ratatui::{
    prelude::*,
    widgets::{Cell, Paragraph, Row, Table},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, scroll_offset: u16, selected_item: usize) {
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
    let visible_height = area.height.saturating_sub(3) as usize; // account for header, hint, and borders
    let offset = scroll_offset as usize;
    let end = (offset + visible_height).min(processes.len());
    let visible_processes = &processes[offset..end];

    let rows: Vec<Row> = visible_processes
        .iter()
        .enumerate()
        .map(|(i, proc)| {
            let actual_index = offset + i;
            let is_selected = actual_index == selected_item;

            let base_style = if is_selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };

            let cpu_style = if proc.cpu_usage >= 50.0 {
                base_style.fg(Color::Red)
            } else if proc.cpu_usage >= 20.0 {
                base_style.fg(Color::Yellow)
            } else if is_selected {
                base_style.fg(Color::Green)
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
            .style(base_style)
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

    // Show hint at bottom
    let hint = format!(
        " [{}-{}/{}] j/k:navigate | x:kill | Tab:switch ",
        offset + 1,
        end,
        processes.len()
    );
    let hint_widget = Paragraph::new(hint)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);

    let hint_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    frame.render_widget(hint_widget, hint_area);
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
