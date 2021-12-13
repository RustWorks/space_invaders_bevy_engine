use bevy::prelude::*;
use serde::Deserialize;

// TODO: Finish gopher & blue laser sprites
pub const FERRIS: &str = r#"sprites\ferris.png"#;
pub const GOPHER: &str = r#"sprites\ferris.png"#;
pub const RED_LASER: &str = r#"sprites\red_laser.png"#;

#[allow(unused)]
pub const BLUE_LASER: &str = r#"sprites\blue_laser.png"#;

pub const SCALE: f32 = 0.8;
pub const TIME_STEP: f32 = 1.0 / 60.0;

// Shared types
pub struct Laser;
pub struct Speed(pub f32);

// Sprite loader
pub struct LoadSprite {
	pub ferris: Handle<ColorMaterial>,
	pub gopher: Handle<ColorMaterial>,
}

// Laser loader
pub struct LoadLaser {
	pub red: Handle<ColorMaterial>,
}

// Window info
pub struct GetWinSize {
	pub h: f32,
	pub w: f32
}

// Deserialize settings.ron
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct LoadSettings {
	pub window: WinOpts
}

#[derive(Debug, Deserialize)]
pub struct WinOpts {
	pub width: f32,
	pub height: f32,
	pub vsync: bool,
	pub resizable: bool,
	pub cursor_visible: bool
}
