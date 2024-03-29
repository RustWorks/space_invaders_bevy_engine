use std::process::exit;

use bevy::{
	prelude::*,
	window::WindowMode
};

use crate::types::*;

// Default speed value
impl Default for Speed {
	fn default() -> Self {
		Self(500.0)
	}
}

// Asset loader
pub fn assets(
	server: Res<AssetServer>,
	mut cmds: Commands,
	mut windows: ResMut<Windows>,
	mut material: ResMut<Assets<ColorMaterial>>,
) {
	let win =
		windows.get_primary_mut().unwrap();

	// Load Actors
	cmds.insert_resource(
		LoadActor {
			ferris: material.add(
				server.load("actors\\ferris.png").into()
			),
			gopher: material.add(
				server.load("actors\\gopher.png").into()
			)
		}
	);

	// Load lasers
	// TODO: Turn laser(s) from images to rectangles
	cmds.insert_resource(
		LoadLaser {
			red: material.add(
				server.load("lasers\\red.png").into()
			)
		}
	);

	// Get window size
	cmds.insert_resource(
		GetWinSize {
			h: win.height(),
			w: win.width()
		}
	);

	// Enable hot reload
	server.watch_for_changes().unwrap();

	// Setup 2D camera view
	cmds.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// Press "Delete" to exit game
pub fn exit_geme(
	input: Res<Input<KeyCode>>
) {
	if input.pressed(KeyCode::Delete) {
		println!("Goodbye!");

		exit(0);
	}
}

// Press "F" to enable fullscreen
// Press "Escape" to disable it
pub fn fullscreen(
	input: Res<Input<KeyCode>>,
	mut windows: ResMut<Windows>
) {
	let win =
		windows.get_primary_mut().unwrap();

	if input.just_pressed(KeyCode::F) {
		win.set_mode(
			WindowMode::Fullscreen {
				use_size: false
			}
		)
	} else if input.just_pressed(KeyCode::Escape) {
		win.set_mode(WindowMode::Windowed)
	}
}
