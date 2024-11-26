use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Paragraph},
    layout::{Layout, Direction, Constraint},
};

use crate::theme::Theme;
use super::{Controls, Section};

pub fn render(controls: &Controls, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
    // Store the area for mouse hit testing
    *controls.area.borrow_mut() = Some(area);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(80),
            Constraint::Percentage(20),
        ])
        .split(area);

    let controls_area = chunks[0];
    let volume_area = chunks[1];

    render_controls_section(controls, frame, controls_area, focused, theme);
    render_volume_section(controls, frame, volume_area, focused, theme);
}

fn render_controls_section(controls: &Controls, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
    let controls_block = Block::default()
        .title("Controls")
        .borders(Borders::ALL)
        .border_type(if focused && controls.focused_section == Section::Controls { 
            BorderType::Thick 
        } else { 
            BorderType::Rounded 
        })
        .border_style(if focused && controls.focused_section == Section::Controls {
            theme.get_style("border_focused")
        } else {
            theme.get_style("border_unfocused")
        });

    let inner_controls = controls_block.inner(area);
    frame.render_widget(controls_block, area);

    // Create control buttons with themed icons and states
    let control_buttons = vec![
        (controls.is_recording, &theme.controls.record, "Record"),
        (!controls.is_playing, &theme.controls.play, "Play"),
        (controls.is_seeking_backward, &theme.controls.rewind, "Rew"),
        (controls.is_seeking_forward, &theme.controls.fast_forward, "FF"),
        (false, &theme.controls.stop, "Stop/Eject"),
        (controls.is_playing, &theme.controls.pause, "Pause"),
        (false, &theme.controls.next, "Next"),
        (false, &theme.controls.previous, "Prev"),
    ];

    let button_width = inner_controls.width / control_buttons.len() as u16;

    // Render each button with highlight and shadow effects
    for (i, (active, icon, label)) in control_buttons.iter().enumerate() {
        let x = inner_controls.x + (i as u16 * button_width);
        let button_area = Rect::new(x, inner_controls.y, button_width, inner_controls.height);

        render_button(
            frame,
            button_area,
            icon,
            label,
            *active,
            focused && controls.focused_section == Section::Controls && controls.focused_button == i,
            theme,
        );
    }
}

fn render_volume_section(controls: &Controls, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
    let volume_block = Block::default()
        .title("Volume")
        .borders(Borders::ALL)
        .border_type(if focused && controls.focused_section == Section::Volume { 
            BorderType::Thick 
        } else { 
            BorderType::Rounded 
        })
        .border_style(if focused && controls.focused_section == Section::Volume {
            theme.get_style("border_focused")
        } else {
            theme.get_style("border_unfocused")
        });

    let inner_volume = volume_block.inner(area);
    frame.render_widget(volume_block, area);

    // TODO: Implement volume slider widget
    frame.render_widget(
        Paragraph::new("Volume Slider")
            .alignment(Alignment::Center)
            .style(if focused && controls.focused_section == Section::Volume {
                theme.get_style("list_selected")
            } else {
                theme.get_style("text_normal")
            }),
        inner_volume,
    );
}

fn render_button(
    frame: &mut Frame,
    area: Rect,
    icon: &str,
    label: &str,
    active: bool,
    focused: bool,
    theme: &Theme,
) {
    // Button shadow effect
    let shadow_style = theme.get_style("button_shadow");
    let shadow_area = Rect::new(area.x + 1, area.y + 1, area.width - 1, area.height - 1);
    frame.render_widget(Block::default().style(shadow_style), shadow_area);

    // Determine button style based on state
    let button_style = if focused {
        theme.get_style("list_selected")
    } else if active && label == "Record" {
        theme.get_style("record_button_active")
    } else if active {
        theme.get_style("button_active")
    } else {
        theme.get_style("button")
    };

    let button_text = format!("{} {}", icon, label);
    frame.render_widget(
        Paragraph::new(button_text)
            .alignment(Alignment::Center)
            .style(button_style),
        area,
    );
}
