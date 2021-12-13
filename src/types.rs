use bevy::prelude::*;

// Shared types
pub const SCALE: f32 = 0.8;
pub const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Laser;
pub struct Speed(pub f32);

// Sprite loader
pub struct LoadActor {
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
