# UI Layout and Navigation Documentation

## Overview
This document details the UI layout structure, component organization, and tab navigation flow for the TUI music player. The interface is divided into three main rows with specific height allocations and sub-components.

## Layout Structure

### 1. Primary Row (60% of screen height)
#### Library Browser (33% width) - Tab Index 1
- Purpose: Music library navigation and organization
- Components:
  * Folder/playlist structure browser
  * Library organization view
  * Source selection interface
- Focus: First tab stop in navigation flow

#### Track List (34% width) - Tab Index 2
- Purpose: Display and select tracks from chosen source
- Components:
  * Track listing with numbers
  * Title display
  * Duration information
- Focus: Second tab stop in navigation flow

#### Track Details (33% width) - Tab Index 3
- Purpose: Extended information for selected track
- Components:
  * Detailed metadata display
  * Album artwork (when available)
  * Technical information (format, bitrate, etc.)
- Focus: Third tab stop in navigation flow

### 2. Secondary Row (25% of screen height)
#### Current Track Info (50% width) - Tab Index 4
- Purpose: Now playing track information
- Components:
  * Current track metadata
  * Artist/Album/Title display
  * Album artwork
- Focus: Fourth tab stop in navigation flow

#### Playback Status (50% width) - Tab Index 5
- Purpose: Playback progress and status
- Components:
  * Progress bar
  * Time position
  * Playback mode indicators (repeat/shuffle)
  * Audio quality display
- Focus: Fifth tab stop in navigation flow

### 3. Control Row (15% of screen height)
All controls are equal width, distributed across the row

#### Previous Track - Tab Index 6
- Purpose: Skip to previous track
- Focus: Sixth tab stop in navigation flow

#### Play/Pause - Tab Index 7
- Purpose: Toggle playback state
- Focus: Seventh tab stop in navigation flow

#### Next Track - Tab Index 8
- Purpose: Skip to next track
- Focus: Eighth tab stop in navigation flow

#### Volume Control - Tab Index 9
- Purpose: Adjust playback volume
- Focus: Ninth tab stop in navigation flow

## Navigation Flow

### Tab Order
1. Forward Navigation (Tab key):
   - Moves left-to-right within each row
   - Proceeds to leftmost component of next row when reaching row end
   - Loops back to first component (Library Browser) after last component (Volume Control)

2. Backward Navigation (Shift+Tab):
   - Reverses the forward navigation pattern
   - Moves right-to-left within each row
   - Proceeds to rightmost component of previous row when reaching row start
   - Loops to last component (Volume Control) when backing past first component

### Focus Indication
- Active component highlighted in yellow
- Clear visual distinction between focused and unfocused components
- Consistent highlighting style across all components

## Implementation Notes

### Layout Scaling
- Components should scale proportionally with terminal window size
- Maintain aspect ratios (60/25/15 vertical, 33/34/33 horizontal where applicable)
- Ensure minimum usable dimensions for each component

### Focus Management
- Implement focus tracking in UI state
- Maintain single focused component at all times
- Handle focus changes through event system

### Visual Consistency
- Use consistent borders and styling across components
- Maintain clear visual hierarchy
- Ensure readability at different terminal sizes

## Technical Considerations
- Implement using ratatui layout system
- Use Constraint-based sizing for proper scaling
- Handle terminal resize events appropriately
- Maintain performance with larger libraries/playlists
