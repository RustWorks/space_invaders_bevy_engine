mod assets;
mod plugins {
	pub mod player;
}

use bevy::{
	prelude::*,
	render::pass::ClearColor
};

use crate::plugins::player::PlayerPlugin;

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
        .add_startup_system(assets::materials.system())
        .add_plugins(DefaultPlugins)
		.add_plugin(PlayerPlugin)
        .run();
}
