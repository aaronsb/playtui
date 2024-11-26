use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Gauge},
};

use crate::events::{Event, Action, TrackMetadata};
use super::{Component, ComponentState};

#[derive(Clone)]
pub struct NowPlaying {
    state: ComponentState,
    current_track: Option<TrackMetadata>,
    progress: f64, // 0.0 to 1.0
    volume: u8,    // 0 to 100
}

impl Component for NowPlaying {
    fn new() -> Self {
        NowPlaying {
            state: ComponentState::default(),
            current_track: None,
            progress: 0.0,
            volume: 100,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let block = Block::default()
            .title("Now Playing")
            .borders(Borders::ALL)
            .border_style(if focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        let inner_area = block.inner(area);

        // Split area for different sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Track info
                Constraint::Length(1), // Progress bar
                Constraint::Length(1), // Volume
            ])
            .split(inner_area);

        // Render the block first
        frame.render_widget(block, area);

        // Render track info
        let track_info = match &self.current_track {
            Some(metadata) => {
                let title = metadata.title.as_deref().unwrap_or("Unknown Title");
                let artist = metadata.artist.as_deref().unwrap_or("Unknown Artist");
                let album = metadata.album.as_deref().unwrap_or("Unknown Album");
                format!("{}\n{}\n{}", title, artist, album)
            }
            None => "No track playing".to_string(),
        };

        frame.render_widget(
            Paragraph::new(track_info)
                .alignment(Alignment::Center),
            chunks[0],
        );

        // Render progress bar
        let progress_percent = (self.progress * 100.0) as u16;
        frame.render_widget(
            Gauge::default()
                .gauge_style(Style::default().fg(Color::Yellow))
                .ratio(self.progress)
                .label(format!("{}%", progress_percent)),
            chunks[1],
        );

        // Render volume bar
        frame.render_widget(
            Gauge::default()
                .gauge_style(Style::default().fg(Color::Green))
                .ratio(self.volume as f64 / 100.0)
                .label(format!("Volume: {}%", self.volume)),
            chunks[2],
        );
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Metadata(crate::events::MetadataAction::Update(metadata)) => {
                self.current_track = Some(metadata);
                None
            }
            Action::Metadata(crate::events::MetadataAction::Clear) => {
                self.current_track = None;
                self.progress = 0.0;
                None
            }
            Action::Player(crate::events::PlayerAction::SetVolume(volume)) => {
                self.volume = volume;
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
        // NowPlaying component doesn't handle direct user input
        None
    }
}

impl NowPlaying {
    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress.clamp(0.0, 1.0);
    }
}
