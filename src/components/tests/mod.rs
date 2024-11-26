use crate::components::Component;

pub mod mock;
pub mod theme;
pub mod events;
pub mod state;

// Re-export commonly used test utilities
pub use mock::MockComponent;
pub use theme::create_test_theme;

// Any shared test utilities can go here
pub fn setup_test_component() -> MockComponent {
    let mut component = MockComponent::new();
    component.set_focused(true);
    component
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup_test_component() {
        let component = setup_test_component();
        assert!(component.focused());
    }
}
