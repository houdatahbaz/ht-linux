use crate::app::{App, TreeNode};
use crate::ui::widgets;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let border_style = Style::default().fg(Color::Green);

    // Draw active tab content directly (no tab bar)
    if let Some(tab) = app.active_tab() {
        let content_block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(format!(" {} ", tab.node.name()));

        let inner_area = content_block.inner(area);
        frame.render_widget(content_block, area);

        match tab.node {
            TreeNode::Overview => widgets::overview::draw(frame, app, inner_area),
            TreeNode::Cpu => widgets::cpu::draw(frame, app, inner_area),
            TreeNode::Memory => widgets::memory::draw(frame, app, inner_area),
            TreeNode::Disks => widgets::disk::draw(frame, app, inner_area),
            TreeNode::Network => widgets::network::draw(frame, app, inner_area),
            TreeNode::Processes => widgets::processes::draw(frame, app, inner_area, tab.scroll_offset, tab.selected_item),
            TreeNode::Devices => widgets::devices::draw(frame, app, inner_area, tab.selected_item),
            TreeNode::Logs => widgets::logs::draw(frame, app, inner_area, tab.scroll_offset),
        }
    }
}
