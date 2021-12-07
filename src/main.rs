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
        .insert_resource
			(
				WindowDescriptor
					{
						title: "Space Invaders".into(),
						width: 598.0,
						height: 675.0,
						vsync: true,
						resizable: false,
						cursor_visible: true,

						..Default::default()
					}
			)
		.insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup::setup.system())
        .add_plugins(DefaultPlugins)
        .run();
}
