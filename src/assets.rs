use bevy::prelude::*;

pub const FERRIS: &str = r#"sprites\ferris.png"#;
pub const LASER: &str = r#"sprites\laser.png"#;

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Sprites {
	pub ferris: Handle<ColorMaterial>,
	pub laser: Handle<ColorMaterial>
}

#[allow(unused)]
pub struct WindowSize {
	pub h: f32,
	w: f32
}

pub fn materials(
	mut cmds: Commands,
	mut windows: ResMut<Windows>,
	asset_srv: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let win =
		windows
			.get_primary_mut()
			.unwrap();

	cmds.spawn_bundle(OrthographicCameraBundle::new_2d());

	cmds.insert_resource
		(
			Sprites {
				ferris: materials.add(asset_srv.load(FERRIS).into()),
    			laser: materials.add(asset_srv.load(LASER).into()),
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
