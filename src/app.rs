use crate::components::{Component, Controls, NowPlaying, Playlist};

pub enum FocusedFrame {
    Playlist,
    NowPlaying,
    Controls,
}

pub struct App {
    pub focused_frame: FocusedFrame,
    pub playlist: Playlist,
    pub now_playing: NowPlaying,
    pub controls: Controls,
}

impl App {
    pub fn new() -> App {
        App {
            focused_frame: FocusedFrame::Playlist,
            playlist: Playlist::new(),
            now_playing: NowPlaying::new(),
            controls: Controls::new(),
        }
    }

    pub fn next_frame(&mut self) {
        self.focused_frame = match self.focused_frame {
            FocusedFrame::Playlist => FocusedFrame::NowPlaying,
            FocusedFrame::NowPlaying => FocusedFrame::Controls,
            FocusedFrame::Controls => FocusedFrame::Playlist,
        };
    }

    pub fn previous_frame(&mut self) {
        self.focused_frame = match self.focused_frame {
            FocusedFrame::Playlist => FocusedFrame::Controls,
            FocusedFrame::NowPlaying => FocusedFrame::Playlist,
            FocusedFrame::Controls => FocusedFrame::NowPlaying,
        };
    }
}
