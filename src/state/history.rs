use super::*;
use std::collections::VecDeque;

const MAX_HISTORY_SIZE: usize = 100;

/// Tracks state history for debugging and undo/redo functionality
pub trait StateHistory {
    /// Records a new state in history
    fn record(&mut self, state: AppState);
    
    /// Returns the previous state if available
    fn undo(&mut self) -> Option<AppState>;
    
    /// Returns the next state if available
    fn redo(&mut self) -> Option<AppState>;
    
    /// Clears the history
    fn clear(&mut self);
}

/// Implementation of state history tracking
pub struct AppStateHistory {
    past_states: VecDeque<AppState>,
    future_states: VecDeque<AppState>,
    current_state: Option<AppState>,
}

impl AppStateHistory {
    pub fn new() -> Self {
        Self {
            past_states: VecDeque::with_capacity(MAX_HISTORY_SIZE),
            future_states: VecDeque::new(),
            current_state: None,
        }
    }

    fn push_past_state(&mut self, state: AppState) {
        if self.past_states.len() >= MAX_HISTORY_SIZE {
            self.past_states.pop_front();
        }
        self.past_states.push_back(state);
    }
}

impl StateHistory for AppStateHistory {
    fn record(&mut self, state: AppState) {
        if let Some(current) = self.current_state.take() {
            self.push_past_state(current);
        }
        self.current_state = Some(state);
        self.future_states.clear(); // Clear redo history on new state
    }

    fn undo(&mut self) -> Option<AppState> {
        if let Some(current) = self.current_state.take() {
            self.future_states.push_front(current);
            if let Some(past) = self.past_states.pop_back() {
                self.current_state = Some(past.clone());
                return Some(past);
            }
            // If no past state, restore the current one
            self.current_state = Some(self.future_states.pop_front().unwrap());
        }
        None
    }

    fn redo(&mut self) -> Option<AppState> {
        if let Some(current) = self.current_state.take() {
            self.past_states.push_back(current);
            if let Some(future) = self.future_states.pop_front() {
                self.current_state = Some(future.clone());
                return Some(future);
            }
            // If no future state, restore the current one
            self.current_state = Some(self.past_states.pop_back().unwrap());
        }
        None
    }

    fn clear(&mut self) {
        self.past_states.clear();
        self.future_states.clear();
        self.current_state = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_recording() {
        let mut history = AppStateHistory::new();
        let state1 = AppState::default();
        let mut state2 = AppState::default();
        state2.player.volume = 50;

        history.record(state1.clone());
        history.record(state2.clone());

        let undo_state = history.undo();
        assert!(undo_state.is_some());
        assert_eq!(undo_state.unwrap().player.volume, state1.player.volume);

        let redo_state = history.redo();
        assert!(redo_state.is_some());
        assert_eq!(redo_state.unwrap().player.volume, state2.player.volume);
    }

    #[test]
    fn test_history_size_limit() {
        let mut history = AppStateHistory::new();
        
        // Fill history beyond max size
        for i in 0..MAX_HISTORY_SIZE + 10 {
            let mut state = AppState::default();
            state.player.volume = i as u8;
            history.record(state);
        }

        // Verify oldest states were removed
        let mut count = 0;
        while history.undo().is_some() {
            count += 1;
        }
        assert!(count <= MAX_HISTORY_SIZE);
    }
}
