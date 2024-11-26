use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, List, ListItem, ListState},
};

use crate::events::{Event, Action, MouseEvent, KeyEvent, PlaylistAction};
use super::{Component, ComponentState};
use crate::theme::Theme;

#[derive(Clone)]
pub struct Playlist {
    state: ComponentState,
    list_state: ListState,
    tracks: Vec<String>,
    scroll_offset: usize,
}

impl Component for Playlist {
    fn new() -> Self {
        Playlist {
            state: ComponentState::default(),
            list_state: ListState::default(),
            tracks: Vec::new(),
            scroll_offset: 0,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        let block = Block::default()
            .title("Playlist")
            .borders(Borders::ALL)
            .border_type(if focused { BorderType::Thick } else { BorderType::Rounded })
            .border_style(if focused {
                theme.get_style("border_focused")
            } else {
                theme.get_style("border_unfocused")
            });

        let items: Vec<ListItem> = self.tracks
            .iter()
            .enumerate()
            .map(|(i, track)| {
                let style = if Some(i) == self.list_state.selected() {
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

        let mut list_state = self.list_state.clone();
        frame.render_stateful_widget(list, area, &mut list_state);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Playlist(PlaylistAction::AddTrack(path)) => {
                self.tracks.push(path);
                // Select first track if none selected
                if self.list_state.selected().is_none() && !self.tracks.is_empty() {
                    self.list_state.select(Some(0));
                }
                None
            }
            Action::Playlist(PlaylistAction::RemoveTrack(index)) => {
                if index < self.tracks.len() {
                    self.tracks.remove(index);
                    // Adjust selection if necessary
                    if let Some(selected) = self.list_state.selected() {
                        if selected >= self.tracks.len() {
                            self.list_state.select(if self.tracks.is_empty() {
                                None
                            } else {
                                Some(self.tracks.len() - 1)
                            });
                        }
                    }
                }
                None
            }
            Action::Playlist(PlaylistAction::SelectTrack(index)) => {
                if index < self.tracks.len() {
                    self.list_state.select(Some(index));
                    if let Some(track) = self.tracks.get(index) {
                        Some(Action::Player(crate::events::PlayerAction::LoadTrack(
                            track.clone(),
                        )))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Action::Playlist(PlaylistAction::Clear) => {
                self.tracks.clear();
                self.list_state.select(None);
                self.scroll_offset = 0;
                None
            }
            _ => None,
        }
    }

    fn focused(&self) -> bool {
        self.state.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        if !self.state.focused {
            return None;
        }

        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            _ => None,
        }
    }
}

impl Playlist {
    fn handle_key_event(&mut self, event: KeyEvent) -> Option<Action> {
        match event {
            KeyEvent::Next => self.select_next(),
            KeyEvent::Previous => self.select_previous(),
            _ => None,
        }
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> Option<Action> {
        match event {
            MouseEvent::Click { x: _, y } => {
                // Calculate which track was clicked based on y coordinate
                // Accounting for border and title
                let index = y.saturating_sub(1) as usize + self.scroll_offset;
                if index < self.tracks.len() {
                    Some(Action::Playlist(PlaylistAction::SelectTrack(index)))
                } else {
                    None
                }
            }
            MouseEvent::Scroll { delta } => {
                // Update scroll offset
                if delta < 0 && self.scroll_offset < self.tracks.len().saturating_sub(1) {
                    self.scroll_offset += 1;
                } else if delta > 0 && self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
                None
            }
        }
    }

    fn select_next(&mut self) -> Option<Action> {
        if self.tracks.is_empty() {
            return None;
        }

        let next_index = match self.list_state.selected() {
            Some(i) if i + 1 < self.tracks.len() => i + 1,
            Some(_) => 0, // Wrap around to start
            None => 0,
        };

        Some(Action::Playlist(PlaylistAction::SelectTrack(next_index)))
    }

    fn select_previous(&mut self) -> Option<Action> {
        if self.tracks.is_empty() {
            return None;
        }

        let prev_index = match self.list_state.selected() {
            Some(0) => self.tracks.len() - 1, // Wrap around to end
            Some(i) => i - 1,
            None => 0,
        };

        Some(Action::Playlist(PlaylistAction::SelectTrack(prev_index)))
    }
}
