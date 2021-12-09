mod assets;
mod plugins {
	pub mod enemy;
	pub mod player;
}

use bevy::{
	prelude::*,
	render::pass::ClearColor
};

#[allow(unused)]
use crate::{
	plugins::{
		enemy::EnemyPlugin,
		player::PlayerPlugin
	},
	assets::store
};

fn main() {
    App::build()
        .insert_resource
			(
				WindowDescriptor
					{
						title: "Space Invaders".into(),
						width: 1000.0,
						height: 600.0,
						vsync: true,
						resizable: false,
						cursor_visible: true,

						..Default::default()
					}
			)
		.insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(store.system())
        .add_plugins(DefaultPlugins)
		.add_plugin(EnemyPlugin)
		.add_plugin(PlayerPlugin)
        .run();
}
