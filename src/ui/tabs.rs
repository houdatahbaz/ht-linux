use crate::app::{App, Focus, TreeNode};
use crate::ui::widgets;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Tabs as RatTabs},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.focus == Focus::Tabs;

    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    // Split area into tab bar and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Draw tab bar
    let tab_titles: Vec<Line> = app
        .tabs
        .iter()
        .enumerate()
        .map(|(i, tab)| {
            let num = i + 1;
            let title = format!(" {}:{} ", num, tab.node.name());
            Line::from(title)
        })
        .collect();

    let tab_bar = RatTabs::new(tab_titles)
        .block(
            Block::default()
                .title(" Tabs ")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .select(app.active_tab_index)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tab_bar, chunks[0]);

    // Draw active tab content
    if let Some(tab) = app.active_tab() {
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(format!(" {} ", tab.node.name()));

        let inner_area = content_block.inner(chunks[1]);
        frame.render_widget(content_block, chunks[1]);

        match tab.node {
            TreeNode::Overview => widgets::overview::draw(frame, app, inner_area),
            TreeNode::Cpu => widgets::cpu::draw(frame, app, inner_area),
            TreeNode::Memory => widgets::memory::draw(frame, app, inner_area),
            TreeNode::Disks => widgets::disk::draw(frame, app, inner_area),
            TreeNode::Network => widgets::network::draw(frame, app, inner_area),
            TreeNode::Processes => widgets::processes::draw(frame, app, inner_area, tab.scroll_offset),
            TreeNode::Devices => widgets::devices::draw(frame, app, inner_area, tab.selected_item),
            TreeNode::Logs => widgets::logs::draw(frame, app, inner_area, tab.scroll_offset),
        }
    }
}
