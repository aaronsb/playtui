use crate::components::{Component, Controls, NowPlaying, Playlist};
use crate::events::{Event, Action, EventHandler, EventDispatcher, EventResult, EventError};
use crate::state::{AppState, StateManager};

pub struct App {
    pub state: AppState,
    pub playlist: Playlist,
    pub now_playing: NowPlaying,
    pub controls: Controls,
    event_dispatcher: EventDispatcher,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            state: AppState::default(),
            playlist: Playlist::new(),
            now_playing: NowPlaying::new(),
            controls: Controls::new(),
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

        // Register components with the event dispatcher
        self.event_dispatcher.register_handler(Box::new(ComponentWrapper {
            component: self.playlist.clone(),
        }));
        self.event_dispatcher.register_handler(Box::new(ComponentWrapper {
            component: self.now_playing.clone(),
        }));
        self.event_dispatcher.register_handler(Box::new(ComponentWrapper {
            component: self.controls.clone(),
        }));
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

        // Update components based on action
        // Note: We're still cloning here because components need ownership of actions
        self.playlist.update(action.clone());
        self.now_playing.update(action.clone());
        self.controls.update(action);

        Ok(())
    }

    fn handle_follow_up_action(&mut self, action: Action) -> EventResult<()> {
        // Handle any cascading actions that result from state updates
        if let Some(next_action) = self.state.update(action.clone()) {
            self.handle_follow_up_action(next_action)?;
        }

        // Update components with the follow-up action
        self.playlist.update(action.clone());
        self.now_playing.update(action.clone());
        self.controls.update(action);

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
        self.playlist.set_focused(focused == "playlist");
        self.now_playing.set_focused(focused == "nowplaying");
        self.controls.set_focused(focused == "controls");

        // Re-register components to update their can_handle status
        self.register_components();
    }
}
