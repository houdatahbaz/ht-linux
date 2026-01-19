use crate::app::App;
use crate::system::format_bytes;
use ratatui::{
    prelude::*,
    widgets::{Cell, Paragraph, Row, Table},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let networks = &app.system_data.network_list;

    if networks.is_empty() {
        let paragraph = Paragraph::new("No network interfaces found");
        frame.render_widget(paragraph, area);
        return;
    }

    let header = Row::new(vec![
        Cell::from("Interface").style(Style::default().bold().fg(Color::Cyan)),
        Cell::from("Received").style(Style::default().bold().fg(Color::Cyan)),
        Cell::from("Transmitted").style(Style::default().bold().fg(Color::Cyan)),
    ])
    .height(1);

    let rows: Vec<Row> = networks
        .iter()
        .map(|net| {
            Row::new(vec![
                Cell::from(net.name.clone()),
                Cell::from(format_bytes(net.received)).style(Style::default().fg(Color::Green)),
                Cell::from(format_bytes(net.transmitted)).style(Style::default().fg(Color::Yellow)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Percentage(40),
        Constraint::Percentage(30),
        Constraint::Percentage(30),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .row_highlight_style(Style::default().bg(Color::DarkGray));

    frame.render_widget(table, area);
}
