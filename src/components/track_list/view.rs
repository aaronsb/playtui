use ratatui::{
    prelude::*,
    widgets::{List, ListItem},
};
use crate::theme::Theme;
use super::{create_block, state::TrackListState};

pub fn render(state: &TrackListState, frame: &mut Frame, area: Rect, theme: &Theme) {
    let block = create_block("Track List", state.focused(), theme);
    
    // Create a list of tracks
    let entries: Vec<ListItem> = state.tracks
        .iter()
        .enumerate()
        .map(|(i, track)| {
            let style = if Some(i) == state.selected_index {
                theme.get_style("list_selected")
            } else {
                theme.get_style("list_item")
            };
            
            ListItem::new(format!("🎵 {}", track))
                .style(style)
        })
        .collect();

    let list = List::new(entries)
        .block(block)
        .highlight_style(theme.get_style("list_selected"));

    frame.render_widget(list, area);
}
