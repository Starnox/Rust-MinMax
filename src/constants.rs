use bevy::prelude::Color;

pub const DEFAULT_BOARD_SIZE: u32 = 3;
pub const DEFAULT_AI_DEPTH: u32 = 4;

pub const DEFAULT_VOLUME: u32 = 50; 
pub const TIME_STEP: f32 = 1.0 / 60.0;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESS_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub const BUTTON_WIDTH : f32 = 250.0;
pub const BUTTON_HEIGHT: f32 = 65.0;
pub const BUTTON_MARGIN: f32 = 20.0;

pub const BUTTON_FONT_SIZE: f32 = 40.0;

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub const LENGTH: f32 = 600.0;

pub const GAME_STRING_FONT_SIZE: f32 = 80.0;
pub const GAME_STRING_MARGIN: f32 = 50.0;

pub const GAME_STRING: &str = "Tic Tac Toe";
pub const PLAY_AI_STRING: &str = "Play vs AI";
pub const PLAY_AGAINST_PLAYER_STRING: &str = "Play 1vs1";
pub const SETTINGS_STRING: &str = "Settings";
pub const QUIT_STRING: &str = "Quit";
pub const AI_DEPTH_SETTING_STRING: &str = "AI Depth";
pub const MATRIX_SIZE_SETTING_STRING: &str = "Matrix size";
pub const VOLUME_SETTING_STRING: &str = "Volume";
pub const BACK_STRING: &str = "Back";

pub const FONT_LOCATION: &str = "fonts/FiraSans-Bold.ttf";
