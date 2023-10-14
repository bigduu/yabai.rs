use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// An **enum** representing a command that can be sent to yabai.
///
/// Used with the `yabai::send_command` function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    FocusSpace(FocusSpaceOption),
    RotateSpace(SpaceRotation),
    BalanceSpace,
    MoveActiveWindowToSpace(i64),
    FocusWindow(i64),
    FocusWindowDirection(Direction),
    SwapWindowDirection(Direction),
    WarpWindowDirection(Direction),
    ToggleWindowFloating,
    ToggleZoomFullscreen,
}

/// An **enum** representing the options passed to the `space --focus` command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum FocusSpaceOption {
    #[strum(serialize = "next")]
    Next,
    #[strum(serialize = "prev")]
    Prev,
    #[strum(serialize = "first")]
    First,
    #[strum(serialize = "last")]
    Last,
    #[strum(serialize = "recent")]
    Recent,
    Space(i64),
}

/// An **enum** representing the options passed to the `space --rotate` command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum SpaceRotation {
    #[strum(serialize = "90")]
    Rotate90,
    #[strum(serialize = "180")]
    Rotate180,
    #[strum(serialize = "270")]
    Rotate270,
}

/// An **enum** representing a cardinal direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum Direction {
    #[strum(serialize = "north")]
    North,
    #[strum(serialize = "south")]
    South,
    #[strum(serialize = "east")]
    East,
    #[strum(serialize = "west")]
    West,
}

/// Information about a mission control space.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceInfo {
    pub id: i64,
    pub uuid: String,
    pub index: i64,
    pub label: String,
    pub r#type: String,
    pub display: i64,
    pub windows: Vec<i64>,
    pub first_window: i64,
    pub last_window: i64,
    pub has_focus: bool,
    pub is_visible: bool,
    pub is_native_fullscreen: bool,
}

/// Information about a display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayInfo {
    pub id: i64,
    pub uuid: String,
    pub index: i64,
    pub frame: Frame,
    pub spaces: Vec<i64>,
}

/// Information about a window.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct WindowInfo {
    pub id: i64,
    pub pid: i64,
    pub app: String,
    pub title: String,
    pub frame: Frame,
    pub role: String,
    pub subrole: String,
    pub display: i64,
    pub space: i64,
    pub level: i64,
    pub layer: String,
    pub opacity: f64,
    pub split_type: String,
    pub split_child: String,
    pub stack_index: i64,
    pub can_move: bool,
    pub can_resize: bool,
    pub has_focus: bool,
    pub has_shadow: bool,
    pub has_parent_zoom: bool,
    pub has_fullscreen_zoom: bool,
    pub is_native_fullscreen: bool,
    pub is_visible: bool,
    pub is_minimized: bool,
    pub is_hidden: bool,
    pub is_floating: bool,
    pub is_sticky: bool,
    pub is_grabbed: bool,
}

/// A rectangle representing the position and size of a window or display.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Frame {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
