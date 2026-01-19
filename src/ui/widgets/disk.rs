use crate::app::App;
use crate::system::format_bytes;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Gauge},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let disks = &app.system_data.disk_list;

    if disks.is_empty() {
        let paragraph = ratatui::widgets::Paragraph::new("No disk information available");
        frame.render_widget(paragraph, area);
        return;
    }

    // Calculate max items that fit
    let item_height = 4;
    let max_items = (area.height as usize / item_height).max(1);

    let constraints: Vec<Constraint> = disks
        .iter()
        .take(max_items)
        .map(|_| Constraint::Length(item_height as u16))
        .chain(std::iter::once(Constraint::Min(0)))
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, disk) in disks.iter().take(max_items).enumerate() {
        let used = disk.total.saturating_sub(disk.available);
        let percent = if disk.total > 0 {
            ((used as f64 / disk.total as f64) * 100.0) as u16
        } else {
            0
        };

        let title = format!(
            "{} ({})",
            disk.mount_point,
            disk.file_system
        );

        let gauge = Gauge::default()
            .block(Block::default().title(title).borders(Borders::NONE))
            .gauge_style(
                Style::default()
                    .fg(usage_color(percent))
                    .bg(Color::DarkGray),
            )
            .percent(percent)
            .label(format!(
                "{} / {} ({:.1}%)",
                format_bytes(used),
                format_bytes(disk.total),
                percent
            ));

        frame.render_widget(gauge, chunks[i]);
    }

    // Show count if there are more
    if disks.len() > max_items {
        let more_text = ratatui::widgets::Paragraph::new(format!(
            "... and {} more disk(s)",
            disks.len() - max_items
        ))
        .style(Style::default().fg(Color::DarkGray));

        if let Some(last_chunk) = chunks.last() {
            frame.render_widget(more_text, *last_chunk);
        }
    }
}

fn usage_color(percent: u16) -> Color {
    if percent >= 90 {
        Color::Red
    } else if percent >= 80 {
        Color::Yellow
    } else if percent >= 60 {
        Color::LightYellow
    } else {
        Color::Green
    }
}
