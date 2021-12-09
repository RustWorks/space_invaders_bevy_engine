use bevy::prelude::*;

// TODO: Load sprites from assets/store.ron instead
pub const FERRIS: &str = r#"sprites\player\ferris.png"#;
pub const FERRIS_LASER: &str = r#"sprites\player\laser.png"#;

// TODO: Draw gopher sprite
pub const GOPHER: &str = r#"sprites\gopher.png"#;

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Laser;

pub struct Sprites {
	pub ferris: Handle<ColorMaterial>,
	pub gopher: Handle<ColorMaterial>,
}

pub struct Lasers {
	pub ferris: Handle<ColorMaterial>,
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

pub fn store(
	server: Res<AssetServer>,
	mut cmds: Commands,
	mut windows: ResMut<Windows>,
	mut material: ResMut<Assets<ColorMaterial>>,
) {
	let win =
		windows
			.get_primary_mut()
			.unwrap();

	cmds.spawn_bundle(OrthographicCameraBundle::new_2d());

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

	cmds.insert_resource
		(
			Lasers {
				ferris: material.add(
					server.load(FERRIS_LASER).into()
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
}
