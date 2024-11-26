use ratatui::prelude::*;
use std::collections::HashMap;

/// Stores and manages component areas for mouse interaction
pub struct AreaManager {
    areas: HashMap<String, Rect>,
}

impl AreaManager {
    pub fn new() -> Self {
        Self {
            areas: HashMap::new(),
        }
    }

    /// Updates the stored area for a component
    pub fn update_area(&mut self, component_name: &str, area: Rect) {
        self.areas.insert(component_name.to_string(), area);
    }

    /// Finds which component contains the given coordinates
    pub fn component_at_position(&self, x: u16, y: u16) -> Option<&str> {
        for (name, area) in &self.areas {
            if x >= area.x && x < area.x + area.width &&
               y >= area.y && y < area.y + area.height {
                return Some(name);
            }
        }
        None
    }

    /// Gets the area for a specific component
    pub fn get_area(&self, component_name: &str) -> Option<Rect> {
        self.areas.get(component_name).copied()
    }
}
