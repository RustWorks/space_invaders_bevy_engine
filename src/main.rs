mod setup;

use bevy::{
	prelude::{
		App, Color,
		IntoSystem,
		DefaultPlugins,
		WindowDescriptor, SystemStage
	},
	render::pass::ClearColor
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
        .add_startup_system(setup::setup.system())
		.add_startup_stage(
			"game_setup_actors",
			SystemStage::single(setup::spawn.system())
		)
		.add_system(setup::player_move.system())
		.add_system(setup::shoot.system())
		.add_system(setup::shoot_move.system())
        .add_plugins(DefaultPlugins)
        .run();
}
