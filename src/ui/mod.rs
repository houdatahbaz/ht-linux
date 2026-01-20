mod tree;
mod tabs;
pub mod widgets;

use crate::app::{App, Mode};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame, app: &App) {
    // Main layout: content area + command line at bottom
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let content_area = main_chunks[0];
    let command_area = main_chunks[1];

    // Content area: tree + tabs
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(25), Constraint::Min(0)])
        .split(content_area);

    // Draw tree navigator (left pane)
    tree::draw(frame, app, chunks[0]);

    // Draw tab panel (right pane)
    tabs::draw(frame, app, chunks[1]);

    // Draw command line at bottom (vim-style)
    draw_command_line(frame, app, command_area);

    // Draw help overlay if active
    if app.show_help {
        draw_help(frame);
    }

    // Draw device popup if active
    if app.show_device_popup {
        draw_device_popup(frame, app);
    }

    // Draw kill confirmation popup if active
    if app.show_kill_confirm {
        draw_kill_confirm(frame, app);
    }
}

fn draw_command_line(frame: &mut Frame, app: &App, area: Rect) {
    let (content, style) = match app.mode {
        Mode::Command => {
            let cursor = if app.command_error.is_some() { "" } else { "_" };
            let text = format!(":{}{}", app.command_buffer, cursor);
            let style = if app.command_error.is_some() {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::White)
            };
            (text, style)
        }
        Mode::Normal => {
            if let Some(err) = &app.command_error {
                (err.clone(), Style::default().fg(Color::Red))
            } else if let Some(msg) = &app.status_message {
                (msg.clone(), Style::default().fg(Color::Green))
            } else {
                (String::new(), Style::default().fg(Color::DarkGray))
            }
        }
    };

    let paragraph = Paragraph::new(content).style(style);
    frame.render_widget(paragraph, area);
}

fn draw_kill_confirm(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 25, frame.area());

    let pid = app.kill_target_pid.unwrap_or(0);
    let name = app.kill_target_name.as_deref().unwrap_or("unknown");

    let lines = vec![
        Line::from(Span::styled(
            "Kill Process?",
            Style::default().bold().fg(Color::Red),
        )),
        Line::from(""),
        Line::from(format!("Process: {}", name)),
        Line::from(format!("PID: {}", pid)),
        Line::from(""),
        Line::from("This will forcefully terminate the process (SIGKILL)."),
        Line::from(""),
        Line::from(vec![
            Span::styled("[Y]", Style::default().fg(Color::Green).bold()),
            Span::raw("es  "),
            Span::styled("[N]", Style::default().fg(Color::Red).bold()),
            Span::raw("o"),
        ]),
    ];

    let block = Block::default()
        .title(" Confirm Kill ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .alignment(Alignment::Center);

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

fn draw_device_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(70, 60, frame.area());

    if let Some(idx) = app.selected_device_index {
        if let Some(device) = app.system_data.devices.get(idx) {
            let lines = vec![
                Line::from(vec![
                    Span::styled("Device Details", Style::default().bold().fg(Color::Cyan)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("Name:       ", Style::default().fg(Color::Yellow)),
                    Span::raw(&device.name),
                ]),
                Line::from(vec![
                    Span::styled("Type:       ", Style::default().fg(Color::Yellow)),
                    Span::raw(&device.device_type),
                ]),
                Line::from(vec![
                    Span::styled("Subsystem:  ", Style::default().fg(Color::Yellow)),
                    Span::raw(&device.subsystem),
                ]),
                Line::from(vec![
                    Span::styled("Size:       ", Style::default().fg(Color::Yellow)),
                    Span::raw(&device.size),
                ]),
                Line::from(vec![
                    Span::styled("Mount:      ", Style::default().fg(Color::Yellow)),
                    Span::raw(device.mountpoint.as_deref().unwrap_or("-")),
                ]),
                Line::from(vec![
                    Span::styled("Model:      ", Style::default().fg(Color::Yellow)),
                    Span::raw(device.model.as_deref().unwrap_or("-")),
                ]),
                Line::from(vec![
                    Span::styled("Vendor:     ", Style::default().fg(Color::Yellow)),
                    Span::raw(device.vendor.as_deref().unwrap_or("-")),
                ]),
                Line::from(vec![
                    Span::styled("Serial:     ", Style::default().fg(Color::Yellow)),
                    Span::raw(device.serial.as_deref().unwrap_or("-")),
                ]),
                Line::from(vec![
                    Span::styled("State:      ", Style::default().fg(Color::Yellow)),
                    Span::raw(device.state.as_deref().unwrap_or("-")),
                ]),
                Line::from(""),
                Line::from(Span::styled(
                    "Press Enter, Esc, or q to close",
                    Style::default().fg(Color::DarkGray),
                )),
            ];

            let block = Block::default()
                .title(format!(" {} ", device.name))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .style(Style::default().bg(Color::Black));

            let paragraph = Paragraph::new(lines)
                .block(block)
                .wrap(Wrap { trim: false });

            frame.render_widget(Clear, area);
            frame.render_widget(paragraph, area);
        }
    }
}

fn draw_help(frame: &mut Frame) {
    let area = centered_rect(60, 70, frame.area());

    let help_text = vec![
        Line::from("LINUX SYSTEM CENTER - HELP").style(Style::default().bold().fg(Color::Cyan)),
        Line::from(""),
        Line::from("Navigation:").style(Style::default().bold()),
        Line::from("  Tab         Switch focus between tree and tabs"),
        Line::from("  Up/k        Move up / Select previous item"),
        Line::from("  Down/j      Move down / Select next item"),
        Line::from("  Left/h      Previous tab (in tab focus)"),
        Line::from("  Right/l     Next tab / Open tab (in tree focus)"),
        Line::from("  Enter       Open selected item / View device details"),
        Line::from("  1-9         Quick switch to tab by number"),
        Line::from(""),
        Line::from("Vim Commands:").style(Style::default().bold()),
        Line::from("  :           Enter command mode"),
        Line::from("  :q          Quit application"),
        Line::from("  :help       Show this help"),
        Line::from(""),
        Line::from("Actions:").style(Style::default().bold()),
        Line::from("  w           Close current tab"),
        Line::from("  x/Delete    Kill selected process (in Processes tab)"),
        Line::from("  ?           Toggle this help"),
        Line::from(""),
        Line::from("Press ? or Esc to close").style(Style::default().fg(Color::DarkGray)),
    ];

    let block = Block::default()
        .title(" Help ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(Wrap { trim: false });

    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
