use crate::app::App;
use crate::system::{format_bytes, format_uptime};
use ratatui::{
    prelude::*,
    widgets::{Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let info = &app.system_data.overview;

    let lines = vec![
        Line::from(vec![
            Span::styled("Hostname:       ", Style::default().fg(Color::Cyan)),
            Span::raw(&info.hostname),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Operating System: ", Style::default().fg(Color::Cyan)),
            Span::raw(&info.os_name),
        ]),
        Line::from(vec![
            Span::styled("OS Version:       ", Style::default().fg(Color::Cyan)),
            Span::raw(&info.os_version),
        ]),
        Line::from(vec![
            Span::styled("Kernel Version:   ", Style::default().fg(Color::Cyan)),
            Span::raw(&info.kernel_version),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Uptime:         ", Style::default().fg(Color::Cyan)),
            Span::raw(format_uptime(info.uptime)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("CPU Cores:      ", Style::default().fg(Color::Cyan)),
            Span::raw(info.cpu_count.to_string()),
        ]),
        Line::from(vec![
            Span::styled("Total Memory:   ", Style::default().fg(Color::Cyan)),
            Span::raw(format_bytes(info.total_memory)),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Press ? for help",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}
