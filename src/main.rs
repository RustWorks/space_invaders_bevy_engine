mod setup;
mod plugins;

use std::{
	fs::File,
	process::exit
};

use bevy::{
	prelude::*,
	render::pass::ClearColor,
	window::WindowMode,
	// diagnostic::{
	// 	LogDiagnosticsPlugin,
	// 	FrameTimeDiagnosticsPlugin
	// }
};
use serde::Deserialize;
use ron::de::from_reader;

use crate::{
	setup::{
		assets,
		exit_geme,
		fullscreen,
	},
	plugins::{
		EnemyPlugin,
		PlayerPlugin
	},
};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Settings {
	window: Win
}

#[derive(Debug, Deserialize)]
struct Win {
	width: f32,
	height: f32,
	vsync: bool,
	resizable: bool,
	cursor_visible: bool
}

fn main() {
	let path =
		format!(
			"{}\\settings\\window.ron",
			env!("CARGO_MANIFEST_DIR")
		);

	let file =
		File::open(&path).expect("Failed opening file");

	let config: Settings =
		match from_reader(file) {
			Ok(o) => o,
			Err(e) => {
				println!("Failed to load config: {}", e);

			exit(1);
		}
	};

    App::build()
        .insert_resource
			(
				WindowDescriptor
					{
						title: "Space Invaders!".into(),
						mode: WindowMode::Windowed,
						width: config.window.width,
						height: config.window.height,
						vsync: config.window.vsync,
						resizable: config.window.resizable,
						cursor_visible: config.window.cursor_visible,

						..Default::default()
					}
			)
		.insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(assets.system())
		.add_system(fullscreen.system())
		.add_system(exit_geme.system())
        .add_plugins(DefaultPlugins)
		.add_plugin(EnemyPlugin)
		.add_plugin(PlayerPlugin)
		// .add_plugin(LogDiagnosticsPlugin::default())
		// .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
