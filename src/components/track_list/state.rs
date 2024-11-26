use super::ComponentState;

#[derive(Clone)]
pub struct TrackListState {
    pub state: ComponentState,
    pub tracks: Vec<String>,
    pub selected_index: Option<usize>,
}

impl Default for TrackListState {
    fn default() -> Self {
        Self {
            state: ComponentState::default(),
            tracks: Vec::new(),
            selected_index: None,
        }
    }
}

impl TrackListState {
    pub fn select_next(&mut self) -> bool {
        if self.tracks.is_empty() {
            return false;
        }
        let max_index = self.tracks.len().saturating_sub(1);
        self.selected_index = Some(self.selected_index
            .map(|i| if i < max_index { i + 1 } else { max_index })
            .unwrap_or(0));
        true
    }

    pub fn select_previous(&mut self) -> bool {
        if self.tracks.is_empty() {
            return false;
        }
        self.selected_index = Some(self.selected_index
            .map(|i| if i > 0 { i - 1 } else { 0 })
            .unwrap_or(0));
        true
    }

    pub fn select_index(&mut self, index: usize) -> bool {
        if index < self.tracks.len() {
            self.selected_index = Some(index);
            true
        } else {
            false
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_index = None;
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn focused(&self) -> bool {
        self.state.focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }
}
