mod types;
mod setup;
mod plugins;

use std::{
	fs::File,
	process::exit
};

use bevy::{
	prelude::*,
	render::pass::ClearColor
};
use ron::de::from_reader;

use crate::{
	types::*,
	setup::*,
	plugins::*
};

fn main() {
	let path =
		format!(r#"{}\settings\settings.ron"#, env!("CARGO_MANIFEST_DIR"));

	let open =
		File::open(&path).expect("Failed opening file");

	let config: Settings =
		match from_reader(open) {
			Ok(o) => o,
			Err(e) => {
				println!("Failed to load config: {}", e);

				exit(1);
		}
	};

    App::build()
        .insert_resource(
			WindowDescriptor
				{
					title: "Space Invaders!".into(),
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
        .run();
}
