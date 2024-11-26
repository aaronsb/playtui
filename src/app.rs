use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, PrevTrack, PlayPause,
    NextTrack, VolumeControl
};
use crate::events::{Event, Action, EventHandler, EventDispatcher, EventResult, EventError};
use crate::state::{AppState, StateManager};

pub struct App {
    pub state: AppState,
    // Primary Row Components
    pub library_browser: LibraryBrowser,
    pub track_list: TrackList,
    pub track_details: TrackDetails,
    // Secondary Row Components
    pub current_track_info: CurrentTrackInfo,
    pub playback_status: PlaybackStatus,
    // Control Row Components
    pub prev_track: PrevTrack,
    pub play_pause: PlayPause,
    pub next_track: NextTrack,
    pub volume_control: VolumeControl,
    event_dispatcher: EventDispatcher,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            state: AppState::default(),
            // Initialize Primary Row Components
            library_browser: LibraryBrowser::new(),
            track_list: TrackList::new(),
            track_details: TrackDetails::new(),
            // Initialize Secondary Row Components
            current_track_info: CurrentTrackInfo::new(),
            playback_status: PlaybackStatus::new(),
            // Initialize Control Row Components
            prev_track: PrevTrack::new(),
            play_pause: PlayPause::new(),
            next_track: NextTrack::new(),
            volume_control: VolumeControl::new(),
            event_dispatcher: EventDispatcher::new(),
        };

        // Register components as event handlers
        app.register_components();
        app
    }

    fn register_components(&mut self) {
        // Create wrapper structs that implement EventHandler for each component
        struct ComponentWrapper<T: Component> {
            component: T,
        }

        impl<T: Component> EventHandler for ComponentWrapper<T> {
            fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>> {
                Ok(self.component.handle_event(event.clone()))
            }

            fn can_handle(&self, _event: &Event) -> bool {
                self.component.focused()
            }
        }

        // Register all components with the event dispatcher
        let components: Vec<Box<dyn EventHandler>> = vec![
            Box::new(ComponentWrapper { component: self.library_browser.clone() }),
            Box::new(ComponentWrapper { component: self.track_list.clone() }),
            Box::new(ComponentWrapper { component: self.track_details.clone() }),
            Box::new(ComponentWrapper { component: self.current_track_info.clone() }),
            Box::new(ComponentWrapper { component: self.playback_status.clone() }),
            Box::new(ComponentWrapper { component: self.prev_track.clone() }),
            Box::new(ComponentWrapper { component: self.play_pause.clone() }),
            Box::new(ComponentWrapper { component: self.next_track.clone() }),
            Box::new(ComponentWrapper { component: self.volume_control.clone() }),
        ];

        // Register each component
        for component in components {
            self.event_dispatcher.register_handler(component);
        }
    }

    pub fn handle_event(&mut self, event: Event) -> EventResult<()> {
        // Dispatch event and handle resulting actions
        match self.event_dispatcher.dispatch(&event) {
            Ok(actions) => {
                for action in actions {
                    self.handle_action(action)?;
                }
                Ok(())
            }
            Err(e) => Err(EventError::DispatchError(format!("Event dispatch failed: {}", e))),
        }
    }

    fn handle_action(&mut self, action: Action) -> EventResult<()> {
        // Update state based on action
        if let Some(follow_up_action) = self.state.update(action.clone()) {
            // Handle any follow-up actions
            self.handle_follow_up_action(follow_up_action)?;
        }

        // Update all components based on action
        self.library_browser.update(action.clone());
        self.track_list.update(action.clone());
        self.track_details.update(action.clone());
        self.current_track_info.update(action.clone());
        self.playback_status.update(action.clone());
        self.prev_track.update(action.clone());
        self.play_pause.update(action.clone());
        self.next_track.update(action.clone());
        self.volume_control.update(action);

        Ok(())
    }

    fn handle_follow_up_action(&mut self, action: Action) -> EventResult<()> {
        // Handle any cascading actions that result from state updates
        if let Some(next_action) = self.state.update(action.clone()) {
            self.handle_follow_up_action(next_action)?;
        }

        // Update all components with the follow-up action
        self.library_browser.update(action.clone());
        self.track_list.update(action.clone());
        self.track_details.update(action.clone());
        self.current_track_info.update(action.clone());
        self.playback_status.update(action.clone());
        self.prev_track.update(action.clone());
        self.play_pause.update(action.clone());
        self.next_track.update(action.clone());
        self.volume_control.update(action);

        Ok(())
    }

    pub fn next_frame(&mut self) -> EventResult<()> {
        self.handle_event(Event::Key(crate::events::KeyEvent::Focus(
            crate::events::FocusDirection::Next
        )))
    }

    pub fn previous_frame(&mut self) -> EventResult<()> {
        self.handle_event(Event::Key(crate::events::KeyEvent::Focus(
            crate::events::FocusDirection::Previous
        )))
    }

    // Update component focus states based on UI state
    pub fn update_focus_states(&mut self) {
        let focused = &self.state.ui.focused_component;
        self.library_browser.set_focused(focused == "library_browser");
        self.track_list.set_focused(focused == "track_list");
        self.track_details.set_focused(focused == "track_details");
        self.current_track_info.set_focused(focused == "current_track_info");
        self.playback_status.set_focused(focused == "playback_status");
        self.prev_track.set_focused(focused == "prev_track");
        self.play_pause.set_focused(focused == "play_pause");
        self.next_track.set_focused(focused == "next_track");
        self.volume_control.set_focused(focused == "volume_control");

        // Re-register components to update their can_handle status
        self.register_components();
    }
}
