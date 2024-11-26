use std::cell::RefCell;
use ratatui::prelude::*;

#[derive(Clone)]
pub struct VolumeState {
    pub volume: u8,
    pub area: RefCell<Option<Rect>>,
}

impl Default for VolumeState {
    fn default() -> Self {
        Self {
            volume: 50, // Default volume 50%
            area: RefCell::new(None),
        }
    }
}

impl VolumeState {
    pub fn increase_volume(&mut self) -> u8 {
        if self.volume < 100 {
            self.volume = self.volume.saturating_add(5);
        }
        self.volume
    }

    pub fn decrease_volume(&mut self) -> u8 {
        if self.volume > 0 {
            self.volume = self.volume.saturating_sub(5);
        }
        self.volume
    }

    pub fn set_volume(&mut self, vol: u8) {
        self.volume = vol.min(100);
    }

    pub fn get_volume(&self) -> u8 {
        self.volume
    }

    pub fn set_area(&self, area: Rect) {
        *self.area.borrow_mut() = Some(area);
    }

    pub fn get_area(&self) -> Option<Rect> {
        *self.area.borrow()
    }
}
