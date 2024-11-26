use crate::components::ComponentState;

#[test]
fn test_component_state_default() {
    let state = ComponentState::default();
    assert!(!state.focused);
}

#[test]
fn test_component_state_clone() {
    let state = ComponentState { focused: true };
    let cloned = state.clone();
    assert_eq!(state.focused, cloned.focused);
}

#[test]
fn test_component_state_modification() {
    let mut state = ComponentState::default();
    assert!(!state.focused);
    
    state.focused = true;
    assert!(state.focused);
}

#[test]
fn test_component_state_equality() {
    let state1 = ComponentState { focused: true };
    let state2 = ComponentState { focused: true };
    let state3 = ComponentState { focused: false };
    
    assert_eq!(state1, state2);
    assert_ne!(state1, state3);
}
