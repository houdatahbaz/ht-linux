use crate::app::App;
use crate::system::format_bytes;
use ratatui::{
    prelude::*,
    widgets::{Block, Gauge, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let mem = &app.system_data.memory;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Length(1),
            Constraint::Length(4),
            Constraint::Min(0),
        ])
        .split(area);

    // RAM usage
    let ram_percent = if mem.total > 0 {
        ((mem.used as f64 / mem.total as f64) * 100.0) as u16
    } else {
        0
    };

    let ram_gauge = Gauge::default()
        .block(Block::default().title("RAM"))
        .gauge_style(
            Style::default()
                .fg(usage_color(ram_percent))
                .bg(Color::DarkGray),
        )
        .percent(ram_percent)
        .label(format!(
            "{} / {} ({:.1}%)",
            format_bytes(mem.used),
            format_bytes(mem.total),
            ram_percent
        ));

    frame.render_widget(ram_gauge, chunks[0]);

    // RAM details
    let ram_details = Paragraph::new(format!(
        "Available: {}",
        format_bytes(mem.available)
    ))
    .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(ram_details, chunks[1]);

    // Swap usage
    let swap_percent = if mem.swap_total > 0 {
        ((mem.swap_used as f64 / mem.swap_total as f64) * 100.0) as u16
    } else {
        0
    };

    let swap_gauge = Gauge::default()
        .block(Block::default().title("Swap"))
        .gauge_style(
            Style::default()
                .fg(usage_color(swap_percent))
                .bg(Color::DarkGray),
        )
        .percent(swap_percent)
        .label(if mem.swap_total > 0 {
            format!(
                "{} / {} ({:.1}%)",
                format_bytes(mem.swap_used),
                format_bytes(mem.swap_total),
                swap_percent
            )
        } else {
            "No swap configured".to_string()
        });

    frame.render_widget(swap_gauge, chunks[2]);

    // Memory breakdown
    let breakdown_text = vec![
        Line::from(""),
        Line::from(Span::styled("Memory Breakdown:", Style::default().bold())),
        Line::from(format!("  Total:     {}", format_bytes(mem.total))),
        Line::from(format!("  Used:      {}", format_bytes(mem.used))),
        Line::from(format!("  Available: {}", format_bytes(mem.available))),
        Line::from(""),
        Line::from(Span::styled("Swap:", Style::default().bold())),
        Line::from(format!("  Total:     {}", format_bytes(mem.swap_total))),
        Line::from(format!("  Used:      {}", format_bytes(mem.swap_used))),
    ];

    let breakdown = Paragraph::new(breakdown_text);
    frame.render_widget(breakdown, chunks[3]);
}

fn usage_color(percent: u16) -> Color {
    if percent >= 90 {
        Color::Red
    } else if percent >= 70 {
        Color::Yellow
    } else if percent >= 50 {
        Color::LightYellow
    } else {
        Color::Green
    }
}
