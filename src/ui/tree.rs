use crate::app::{App, Focus};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.focus == Focus::Tree;

    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let block = Block::default()
        .title(" Navigator ")
        .borders(Borders::ALL)
        .border_style(border_style);

    let items: Vec<ListItem> = app
        .tree_nodes
        .iter()
        .enumerate()
        .map(|(i, node)| {
            let is_open = app.tabs.iter().any(|t| t.node == *node);
            let marker = if is_open { ">" } else { " " };
            let content = format!("{} {} {}", marker, node.icon(), node.name());

            let style = if i == app.selected_tree_index && is_focused {
                Style::default()
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .bold()
            } else if i == app.selected_tree_index {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
            } else if is_open {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            };

            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items).block(block);

    let mut state = ListState::default();
    state.select(Some(app.selected_tree_index));

    frame.render_stateful_widget(list, area, &mut state);
}
