//! Floating-note placement: when Super+N shows, focuses, or hides, and the
//! default top-right size from a 1512×945 logical display.

use serde::{Deserialize, Serialize};

/// What Super+N does to the main floating note.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleAction {
    /// The window is hidden. Show it and focus it.
    Show,
    /// The window is on screen but another window has focus. Focus the note.
    Focus,
    /// The note itself is focused. Hide it.
    Hide,
}

pub fn toggle_action(visible: bool, focused: bool) -> ToggleAction {
    match (visible, focused) {
        (true, false) => ToggleAction::Focus,
        (true, true) => ToggleAction::Hide,
        (false, _) => ToggleAction::Show,
    }
}

/// Layout-pixel box. `x` and `y` are global, the same coordinates `hyprctl clients` reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub const DEFAULT_WIDTH: i32 = 460;
pub const DEFAULT_HEIGHT: i32 = 420;
/// Sits just under the Omarchy bar on this machine.
pub const DEFAULT_TOP: i32 = 34;
pub const DEFAULT_RIGHT_MARGIN: i32 = 4;

/// Top-right of a monitor. `origin_*` is the monitor's position in the layout,
/// `monitor_w` / `monitor_h` are its size in layout pixels (physical size divided by scale).
pub fn default_geometry(origin_x: i32, origin_y: i32, monitor_w: i32, monitor_h: i32) -> WindowGeometry {
    let width = DEFAULT_WIDTH.min(monitor_w.max(1));
    let height = DEFAULT_HEIGHT.min(monitor_h.max(1));
    let x = origin_x + (monitor_w - width - DEFAULT_RIGHT_MARGIN).max(0);
    let y = origin_y + DEFAULT_TOP.min((monitor_h - height).max(0));
    WindowGeometry { x, y, width, height }
}

pub fn logical_extent(width: i32, height: i32, scale: f64) -> (i32, i32) {
    let scale = if scale <= 0.0 { 1.0 } else { scale };
    ((width as f64 / scale).round() as i32, (height as f64 / scale).round() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unfocused_visible_note_is_refocused() {
        assert_eq!(toggle_action(true, false), ToggleAction::Focus);
    }

    #[test]
    fn focused_note_hides() {
        assert_eq!(toggle_action(true, true), ToggleAction::Hide);
    }

    #[test]
    fn hidden_note_shows() {
        assert_eq!(toggle_action(false, false), ToggleAction::Show);
        assert_eq!(toggle_action(false, true), ToggleAction::Show);
    }

    #[test]
    fn default_geometry_matches_the_top_right_of_the_built_in_display() {
        // eDP-1 is 3024×1890 at scale 2, so the layout is 1512×945.
        let g = default_geometry(0, 0, 1512, 945);
        assert_eq!(g.x, 1048);
        assert_eq!(g.y, 34);
        assert_eq!(g.width, 460);
        assert_eq!(g.height, 420);
    }

    #[test]
    fn logical_extent_divides_the_panel_by_its_scale() {
        assert_eq!(logical_extent(3024, 1890, 2.0), (1512, 945));
        assert_eq!(logical_extent(1920, 1080, 1.0), (1920, 1080));
    }
}
