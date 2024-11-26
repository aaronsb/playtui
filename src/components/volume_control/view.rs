use ratatui::prelude::*;
use crate::components::create_block;
use crate::theme::Theme;
use super::state::VolumeState;

pub fn render(state: &VolumeState, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
    let title = format!("Volume: {}%", state.get_volume());
    let block = create_block(title.as_str(), focused, theme);
    
    // Store the area for mouse interaction calculations
    state.set_area(area);
    
    frame.render_widget(block, area);
}
