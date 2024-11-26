use super::*;
use crate::events::{Event, KeyEvent, Action, TrackMetadata, MetadataAction};
use ratatui::{
    backend::TestBackend,
    Terminal,
    layout::Rect,
};

#[test]
fn test_track_details_new() {
    let details = TrackDetails::new();
    assert!(!details.focused(), "Should not be focused by default");
}

#[test]
fn test_track_details_focus() {
    let mut details = TrackDetails::new();
    assert!(!details.focused(), "Should start unfocused");
    
    details.set_focused(true);
    assert!(details.focused(), "Should be focused after set_focused(true)");
    
    details.set_focused(false);
    assert!(!details.focused(), "Should be unfocused after set_focused(false)");
}

#[test]
fn test_track_details_render() {
    let details = TrackDetails::new();
    let theme = crate::theme::Theme::load_default().unwrap();
    let area = Rect::new(0, 0, 20, 3);
    let backend = TestBackend::new(20, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Test unfocused render
    terminal.draw(|frame| {
        details.render(frame, area, false, &theme);
    }).unwrap();
    
    let buffer = terminal.backend().buffer();
    let content = buffer.content.iter()
        .map(|cell| cell.symbol.clone())
        .collect::<String>();
    assert!(content.contains("Track Details"), "Should display title");
    
    // Test focused render
    terminal.draw(|frame| {
        details.render(frame, area, true, &theme);
    }).unwrap();
    
    let buffer = terminal.backend().buffer();
    let content = buffer.content.iter()
        .map(|cell| cell.symbol.clone())
        .collect::<String>();
    assert!(content.contains("Track Details"), "Should display title when focused");
}

#[test]
fn test_track_details_event_handling() {
    let mut details = TrackDetails::new();
    details.set_focused(true);
    
    let result = details.handle_event(Event::Key(KeyEvent::Enter));
    assert!(result.is_none(), "Should not handle any events currently");
}

// Tests for future functionality - marked as ignored until implemented
#[test]
#[ignore]
fn test_track_details_metadata_display() {
    let mut details = TrackDetails::new();
    let metadata = TrackMetadata {
        title: Some("Test Song".to_string()),
        artist: Some("Test Artist".to_string()),
        album: Some("Test Album".to_string()),
        duration: Some(180), // 3 minutes
    };
    
    // Future: Test updating metadata
    details.update(Action::Metadata(MetadataAction::Update(metadata)));
    
    // Future: Test rendering with metadata
    let theme = crate::theme::Theme::load_default().unwrap();
    let area = Rect::new(0, 0, 40, 5);
    let backend = TestBackend::new(40, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    
    terminal.draw(|frame| {
        details.render(frame, area, false, &theme);
    }).unwrap();
    
    let buffer = terminal.backend().buffer();
    let content = buffer.content.iter()
        .map(|cell| cell.symbol.clone())
        .collect::<String>();
    assert!(content.contains("Test Song"), "Should display track title");
    assert!(content.contains("Test Artist"), "Should display artist");
    assert!(content.contains("Test Album"), "Should display album");
    assert!(content.contains("3:00"), "Should display duration");
}

#[test]
#[ignore]
fn test_track_details_metadata_clear() {
    let mut details = TrackDetails::new();
    
    // Future: Test clearing metadata
    details.update(Action::Metadata(MetadataAction::Clear));
    
    // Future: Test rendering with cleared metadata
    let theme = crate::theme::Theme::load_default().unwrap();
    let area = Rect::new(0, 0, 40, 5);
    let backend = TestBackend::new(40, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    
    terminal.draw(|frame| {
        details.render(frame, area, false, &theme);
    }).unwrap();
    
    let buffer = terminal.backend().buffer();
    let content = buffer.content.iter()
        .map(|cell| cell.symbol.clone())
        .collect::<String>();
    assert!(content.contains("No track selected"), "Should display default message when no track");
}

#[test]
#[ignore]
fn test_track_details_partial_metadata() {
    let mut details = TrackDetails::new();
    let metadata = TrackMetadata {
        title: Some("Test Song".to_string()),
        artist: None,
        album: None,
        duration: Some(180),
    };
    
    // Future: Test updating with partial metadata
    details.update(Action::Metadata(MetadataAction::Update(metadata)));
    
    // Future: Test rendering with partial metadata
    let theme = crate::theme::Theme::load_default().unwrap();
    let area = Rect::new(0, 0, 40, 5);
    let backend = TestBackend::new(40, 5);
    let mut terminal = Terminal::new(backend).unwrap();
    
    terminal.draw(|frame| {
        details.render(frame, area, false, &theme);
    }).unwrap();
    
    let buffer = terminal.backend().buffer();
    let content = buffer.content.iter()
        .map(|cell| cell.symbol.clone())
        .collect::<String>();
    assert!(content.contains("Test Song"), "Should display track title");
    assert!(content.contains("Unknown Artist"), "Should display placeholder for missing artist");
    assert!(content.contains("Unknown Album"), "Should display placeholder for missing album");
    assert!(content.contains("3:00"), "Should display duration");
}
