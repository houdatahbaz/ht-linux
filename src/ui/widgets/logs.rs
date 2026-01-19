use crate::app::App;
use ratatui::{
    prelude::*,
    widgets::Paragraph,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, scroll_offset: u16) {
    let logs = &app.system_data.logs;

    if logs.is_empty() {
        let paragraph = Paragraph::new("No log entries available (try running with sudo for dmesg access)");
        frame.render_widget(paragraph, area);
        return;
    }

    // Calculate visible range
    let visible_height = area.height as usize;
    let offset = scroll_offset as usize;
    let end = (offset + visible_height).min(logs.len());
    let visible_logs = &logs[offset..end];

    let lines: Vec<Line> = visible_logs
        .iter()
        .map(|log| {
            // Color based on log level indicators
            let style = if log.contains("error") || log.contains("ERROR") || log.contains("fail") {
                Style::default().fg(Color::Red)
            } else if log.contains("warn") || log.contains("WARN") {
                Style::default().fg(Color::Yellow)
            } else if log.contains("info") || log.contains("INFO") {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };

            Line::from(Span::styled(log.as_str(), style))
        })
        .collect();

    let paragraph = Paragraph::new(lines);

    frame.render_widget(paragraph, area);

    // Show scroll indicator
    if logs.len() > visible_height {
        let indicator = format!(
            " [{}-{}/{}] Use j/k to scroll ",
            offset + 1,
            end,
            logs.len()
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
