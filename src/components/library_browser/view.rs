use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};
use crate::components::create_block;
use crate::theme::Theme;
use super::state::LibraryBrowserState;

pub fn render(state: &LibraryBrowserState, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
    let block = create_block("Library Browser", focused, theme);
    
    // Get all the data we need in one borrow
    let entries = state.get_entries();
    let selected = state.get_selected_index();
    
    // Create list items with proper styling
    let items: Vec<ListItem> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let prefix = if entry.is_dir() { "📁 " } else { "📄 " };
            let style = if Some(index) == selected {
                if focused {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::DarkGray)
                }
            } else {
                if focused {
                    theme.get_style("list_item")
                } else {
                    Style::default().fg(Color::DarkGray)
                }
            };
            
            ListItem::new(format!("{}{}", prefix, entry.name()))
                .style(style)
        })
        .collect();

    // Create list widget with explicit highlight style
    let list = List::new(items)
        .block(block)
        .highlight_style(
            if focused {
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::Black)
            }
        );

    // Create and render list state
    let mut list_state = ListState::default();
    list_state.select(selected);
    frame.render_stateful_widget(list, area, &mut list_state);
}
