use playtui::components::{
    Component, VolumeControl, TrackDetails,
    current_track_info::CurrentTrackInfo,
    playback_status::PlaybackStatus
};
use playtui::events::{Event, Action, KeyEvent, TrackMetadata, MetadataAction};
use playtui::theme::Theme;
use ratatui::{
    backend::TestBackend,
    Terminal,
    layout::{Layout, Direction, Constraint},
};

// Helper struct to simulate component interactions
struct TestLayout {
    volume: VolumeControl,
    track_details: TrackDetails,
    current_track: CurrentTrackInfo,
    playback: PlaybackStatus,
    theme: Theme,
}

impl TestLayout {
    fn new() -> Self {
        Self {
            volume: VolumeControl::new(),
            track_details: TrackDetails::new(),
            current_track: CurrentTrackInfo::new(),
            playback: PlaybackStatus::new(),
            theme: Theme::load_default().unwrap(),
        }
    }

    fn handle_event(&mut self, event: Event) -> Vec<Action> {
        let mut actions = Vec::new();
        
        // Collect actions from all components
        if let Some(action) = self.volume.handle_event(event.clone()) {
            actions.push(action);
        }
        if let Some(action) = self.track_details.handle_event(event.clone()) {
            actions.push(action);
        }
        if let Some(action) = self.current_track.handle_event(event.clone()) {
            actions.push(action);
        }
        if let Some(action) = self.playback.handle_event(event) {
            actions.push(action);
        }
        
        // Process the actions
        for action in actions.clone() {
            self.update(action);
        }
        
        actions
    }

    fn update(&mut self, action: Action) -> Vec<Action> {
        let mut actions = Vec::new();
        
        // Propagate action to all components
        if let Some(new_action) = self.volume.update(action.clone()) {
            actions.push(new_action.clone());
            self.update(new_action); // Process any follow-up actions
        }
        if let Some(new_action) = self.track_details.update(action.clone()) {
            actions.push(new_action.clone());
            self.update(new_action); // Process any follow-up actions
        }
        if let Some(new_action) = self.current_track.update(action.clone()) {
            actions.push(new_action.clone());
            self.update(new_action); // Process any follow-up actions
        }
        if let Some(new_action) = self.playback.update(action) {
            actions.push(new_action.clone());
            self.update(new_action); // Process any follow-up actions
        }
        
        actions
    }

    fn render(&self) -> String {
        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        
        terminal.draw(|frame| {
            // Layout components
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),  // Volume
                    Constraint::Length(3),  // Track Details
                    Constraint::Length(3),  // Current Track
                    Constraint::Length(3),  // Playback
                ].as_ref())
                .split(frame.size());
            
            self.volume.render(frame, chunks[0], self.volume.focused(), &self.theme);
            self.track_details.render(frame, chunks[1], self.track_details.focused(), &self.theme);
            self.current_track.render(frame, chunks[2], self.current_track.focused(), &self.theme);
            self.playback.render(frame, chunks[3], self.playback.focused(), &self.theme);
        }).unwrap();
        
        let buffer = terminal.backend().buffer();
        buffer.content.iter()
            .map(|cell| cell.symbol.clone())
            .collect()
    }
}

#[test]
fn test_volume_control_interaction() {
    let mut layout = TestLayout::new();
    
    // Focus volume control
    layout.volume.set_focused(true);
    
    // Test volume up interaction
    let actions = layout.handle_event(Event::Key(KeyEvent::Up));
    assert!(!actions.is_empty(), "Should produce volume up action");
    
    let rendered = layout.render();
    assert!(rendered.contains("Volume: 55%"), "Volume should increase to 55%");
    
    // Test volume down interaction
    let actions = layout.handle_event(Event::Key(KeyEvent::Down));
    assert!(!actions.is_empty(), "Should produce volume down action");
    
    let rendered = layout.render();
    assert!(rendered.contains("Volume: 50%"), "Volume should decrease to 50%");
}

#[test]
fn test_focus_cycling() {
    let mut layout = TestLayout::new();
    
    // Initially focus volume
    layout.volume.set_focused(true);
    layout.track_details.set_focused(false);
    layout.current_track.set_focused(false);
    layout.playback.set_focused(false);
    
    let rendered = layout.render();
    assert!(rendered.contains("Volume: 50%"), "Volume control should show focused border");
    
    // Move focus to track details
    layout.volume.set_focused(false);
    layout.track_details.set_focused(true);
    
    let rendered = layout.render();
    assert!(rendered.contains("Track Details"), "Track details should show focused border");
}

#[test]
fn test_action_propagation() {
    let mut layout = TestLayout::new();
    
    // Simulate playing a track
    let _actions = layout.update(Action::Play);
    
    // Verify the action was processed
    let rendered = layout.render();
    assert!(rendered.contains("Status: Playing"), "Playback status should update");
    
    // Test stopping
    let _actions = layout.update(Action::Stop);
    
    // Verify the stop action was processed
    let rendered = layout.render();
    assert!(rendered.contains("Status: Stopped"), "Playback status should update to stopped");
}

#[test]
fn test_component_independence() {
    let mut layout = TestLayout::new();
    
    // Focus volume control
    layout.volume.set_focused(true);
    
    // Volume events shouldn't affect other components
    let actions = layout.handle_event(Event::Key(KeyEvent::Up));
    assert!(actions.iter().all(|a| matches!(a, Action::VolumeUp | Action::SetVolume(_))),
        "Only volume-related actions should be produced");
    
    // Other components should maintain their state
    let rendered = layout.render();
    assert!(rendered.contains("Track Details"), "Track details should remain unchanged");
    assert!(rendered.contains("Current Track"), "Current track should remain unchanged");
}

// Test for future functionality - marked as ignored
#[test]
#[ignore]
fn test_metadata_propagation() {
    let mut layout = TestLayout::new();
    
    // Future: Test metadata updates propagating to relevant components
    let metadata = TrackMetadata {
        title: Some("Test Song".to_string()),
        artist: Some("Test Artist".to_string()),
        album: Some("Test Album".to_string()),
        duration: Some(180),
    };
    
    let action = Action::Metadata(MetadataAction::Update(metadata));
    layout.update(action);
    
    let rendered = layout.render();
    assert!(rendered.contains("Test Song"), "Track title should appear in multiple components");
    assert!(rendered.contains("Test Artist"), "Artist should appear in track details");
    assert!(rendered.contains("3:00"), "Duration should appear in appropriate components");
}
