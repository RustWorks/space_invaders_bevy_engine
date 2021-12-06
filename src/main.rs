use bevy::{
	prelude::{
		App, Color,
		IntoSystem,
		DefaultPlugins,
		WindowDescriptor
	},
	render::pass::ClearColor
};

mod setup;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Space Invaders".to_string(),
            width: 598.0,
            height: 675.0,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(setup::setup.system())
        .add_plugins(DefaultPlugins)
        .run();
}
