use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::{Block, Gauge, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let cpus = &app.system_data.cpus;

    if cpus.is_empty() {
        let paragraph = Paragraph::new("No CPU data available");
        frame.render_widget(paragraph, area);
        return;
    }

    // Calculate average CPU usage
    let avg_usage: f32 = cpus.iter().map(|c| c.usage).sum::<f32>() / cpus.len() as f32;

    // Split area into average gauge and per-core bars
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(0),
        ])
        .split(area);

    // Average CPU gauge
    let gauge = Gauge::default()
        .block(Block::default().title("Overall CPU Usage"))
        .gauge_style(
            Style::default()
                .fg(usage_color(avg_usage))
                .bg(Color::DarkGray),
        )
        .percent(avg_usage as u16)
        .label(format!("{:.1}%", avg_usage));

    frame.render_widget(gauge, chunks[0]);

    // Frequency info
    if let Some(cpu) = cpus.first() {
        let freq_info = Paragraph::new(format!("Frequency: {} MHz", cpu.frequency))
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(freq_info, chunks[1]);
    }

    // Per-core display
    let cores_per_row = 4;
    let core_height = 3;
    let rows_needed = (cpus.len() + cores_per_row - 1) / cores_per_row;

    let constraints: Vec<Constraint> = (0..rows_needed)
        .map(|_| Constraint::Length(core_height))
        .collect();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(chunks[2]);

    for (row_idx, row_area) in rows.iter().enumerate() {
        let start_idx = row_idx * cores_per_row;
        let end_idx = (start_idx + cores_per_row).min(cpus.len());
        let row_cpus = &cpus[start_idx..end_idx];

        let col_constraints: Vec<Constraint> = row_cpus
            .iter()
            .map(|_| Constraint::Ratio(1, cores_per_row as u32))
            .collect();

        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(*row_area);

        for (col_idx, (col_area, cpu)) in cols.iter().zip(row_cpus.iter()).enumerate() {
            let core_num = start_idx + col_idx;
            let gauge = Gauge::default()
                .block(Block::default().title(format!("Core {}", core_num)))
                .gauge_style(
                    Style::default()
                        .fg(usage_color(cpu.usage))
                        .bg(Color::DarkGray),
                )
                .percent(cpu.usage as u16)
                .label(format!("{:.0}%", cpu.usage));

            frame.render_widget(gauge, *col_area);
        }
    }
}

fn usage_color(usage: f32) -> Color {
    if usage >= 90.0 {
        Color::Red
    } else if usage >= 70.0 {
        Color::Yellow
    } else if usage >= 50.0 {
        Color::LightYellow
    } else {
        Color::Green
    }
}
