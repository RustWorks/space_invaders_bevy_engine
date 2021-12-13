mod types;
mod setup;
mod plugins;

use bevy::{
	prelude::*,
	render::pass::ClearColor
};

use crate::{
	setup::*,
	plugins::*
};

fn main() {
	App::build()
		.insert_resource(
			WindowDescriptor
				{
					title: "Space Invaders!".into(),
					width: 1000.0,
					height: 600.0,
					vsync: true,
					resizable: false,

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
