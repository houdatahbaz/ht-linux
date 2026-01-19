use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Cell, Paragraph, Row, Table},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, selected_item: usize) {
    let devices = &app.system_data.devices;

    if devices.is_empty() {
        let paragraph = Paragraph::new("No devices found. Try running with appropriate permissions.");
        frame.render_widget(paragraph, area);
        return;
    }

    // Header
    let header = Row::new(vec![
        Cell::from("Name").style(Style::default().bold().fg(Color::Cyan)),
        Cell::from("Type").style(Style::default().bold().fg(Color::Cyan)),
        Cell::from("Subsystem").style(Style::default().bold().fg(Color::Cyan)),
        Cell::from("Size").style(Style::default().bold().fg(Color::Cyan)),
        Cell::from("State").style(Style::default().bold().fg(Color::Cyan)),
    ])
    .height(1);

    let rows: Vec<Row> = devices
        .iter()
        .enumerate()
        .map(|(i, dev)| {
            let style = if i == selected_item {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };

            let type_style = match dev.subsystem.as_str() {
                "block" => Style::default().fg(Color::Green),
                "usb" => Style::default().fg(Color::Yellow),
                "pci" => Style::default().fg(Color::Magenta),
                "input" => Style::default().fg(Color::Cyan),
                _ => Style::default(),
            };

            Row::new(vec![
                Cell::from(truncate_string(&dev.name, 30)),
                Cell::from(dev.device_type.clone()).style(type_style),
                Cell::from(dev.subsystem.clone()),
                Cell::from(dev.size.clone()),
                Cell::from(dev.state.clone().unwrap_or_else(|| "-".to_string())),
            ])
            .style(style)
        })
        .collect();

    let widths = [
        Constraint::Min(20),
        Constraint::Length(15),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(12),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(Style::default().bg(Color::DarkGray));

    frame.render_widget(table, area);

    // Show hint at bottom
    let hint = Paragraph::new(" Press Enter to view device details | j/k to navigate ")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);

    let hint_area = Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    frame.render_widget(hint, hint_area);
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
