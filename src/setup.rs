use bevy::{
	prelude::*,
	window::WindowMode
};

// TODO: Create gopher sprite
pub const FERRIS: &str = "sprites\\ferris.png";
pub const GOPHER: &str = "sprites\\ferris.png";

#[allow(unused)]
pub const BLUE_LASER: &str = "sprites\\blue_laser.png";
pub const RED_LASER: &str = "sprites\\red_laser.png";

pub const SCALE: f32 = 0.8;
pub const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Laser;

pub struct Sprites {
	pub ferris: Handle<ColorMaterial>,
	pub gopher: Handle<ColorMaterial>,
}

pub struct Lasers {
	pub red: Handle<ColorMaterial>,
}

pub struct WindowSize {
	pub h: f32,
	pub w: f32
}

pub struct Speed(pub f32);
impl Default for Speed {
	fn default() -> Self {
		Self(500.0)
	}
}

pub fn assets(
	server: Res<AssetServer>,
	mut cmds: Commands,
	mut windows: ResMut<Windows>,
	mut material: ResMut<Assets<ColorMaterial>>,
) {
	let win =
		windows
			.get_primary_mut()
			.unwrap();

	// Enable hot reload
	server
		.watch_for_changes()
		.unwrap();

	cmds.insert_resource
		(
			Sprites {
				ferris: material.add(
					server.load(FERRIS).into()
				),
				gopher: material.add(
					server.load(GOPHER).into()
				),
			}
		);

	// TODO: Turn laser(s) from images to rectangles
	cmds.insert_resource
		(
			Lasers {
				red: material.add(
					server.load(RED_LASER).into()
				)
			}
		);

	cmds.insert_resource
		(
			WindowSize {
				h: win.height(),
				w: win.width()
			}
		);

	cmds.spawn_bundle(
		OrthographicCameraBundle::new_2d()
	);
}

pub fn exit_geme(
	input: Res<Input<KeyCode>>
) {
	if input.pressed(
		KeyCode::Delete
	) {
		println!("Goodbye!");

		std::process::exit(0);
	}
}

pub fn fullscreen(
	input: Res<Input<KeyCode>>,
	mut windows: ResMut<Windows>
) {
	let win =
		windows
			.get_primary_mut()
			.unwrap();

	if input.just_pressed(
		KeyCode::F
	) {
		win.set_mode(
			WindowMode::Fullscreen {
				use_size: false
			}
		)
	} else if input.just_pressed(
		KeyCode::Escape
	) {
		win.set_mode(
			WindowMode::Windowed
		)
	}
}
