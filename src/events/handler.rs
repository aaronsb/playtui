use super::error::EventResult;
use super::types::Event;
use super::actions::Action;

pub trait EventHandler {
    fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>>;
    fn can_handle(&self, event: &Event) -> bool;
}

pub struct EventDispatcher {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    pub fn dispatch(&mut self, event: &Event) -> EventResult<Vec<Action>> {
        let mut actions = Vec::new();
        
        for handler in self.handlers.iter_mut() {
            if handler.can_handle(event) {
                match handler.handle_event(event) {
                    Ok(Some(action)) => actions.push(action),
                    Ok(None) => continue,
                    Err(e) => {
                        eprintln!("Handler error: {}", e);
                        continue;
                    }
                }
            }
        }

        Ok(actions)
    }

    pub fn dispatch_filtered<F>(&mut self, event: &Event, filter: F) -> EventResult<Vec<Action>>
    where
        F: Fn(&dyn EventHandler) -> bool,
    {
        let mut actions = Vec::new();
        
        for handler in self.handlers.iter_mut() {
            if filter(handler.as_ref()) && handler.can_handle(event) {
                match handler.handle_event(event) {
                    Ok(Some(action)) => actions.push(action),
                    Ok(None) => continue,
                    Err(e) => {
                        eprintln!("Handler error: {}", e);
                        continue;
                    }
                }
            }
        }

        Ok(actions)
    }
}
