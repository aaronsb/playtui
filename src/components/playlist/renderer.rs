use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, List, ListItem},
};
use crate::theme::Theme;
use super::Playlist;

pub(super) fn render(playlist: &Playlist, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
    let block = Block::default()
        .title("Playlist")
        .borders(Borders::ALL)
        .border_type(if focused { BorderType::Thick } else { BorderType::Rounded })
        .border_style(if focused {
            theme.get_style("border_focused")
        } else {
            theme.get_style("border_unfocused")
        });

    let items: Vec<ListItem> = playlist.tracks
        .iter()
        .enumerate()
        .map(|(i, track)| {
            let style = if Some(i) == playlist.list_state.selected() {
                theme.get_style("list_selected")
            } else {
                theme.get_style("list_item")
            };
            
            // Extract filename from path for display
            let display_name = track
                .split('/')
                .last()
                .unwrap_or(track)
                .to_string();
            
            ListItem::new(display_name).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(theme.get_style("list_selected"))
        .highlight_symbol(">");

    let mut list_state = playlist.list_state.clone();
    frame.render_stateful_widget(list, area, &mut list_state);
}
