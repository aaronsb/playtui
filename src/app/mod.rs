// Declare submodules
mod components;
mod event_handling;
mod focus;
mod areas;
mod initialization;
mod lifecycle;
mod state;

// Re-export public items
pub use components::{
    ComponentManager,
    registry::ComponentRegistry,
    lifecycle::ComponentLifecycle,
    relationships::ComponentRelationships
};
pub use event_handling::EventManager;
pub use focus::FocusManager;
pub use areas::AreaManager;
pub use state::App;

#[cfg(test)]
mod tests;
