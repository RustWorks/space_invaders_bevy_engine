use {
	bevy::{
		prelude::{
			Res, ResMut, Windows,
			Commands, Transform,
			Assets, AssetServer,
			OrthographicCameraBundle,
			SpriteBundle, ColorMaterial
		},
		math::Vec3
	}
};

const SPACE_SHIP: &str = r#"sprites\ferris.png"#;

pub fn setup(
	server: Res<AssetServer>,
	mut windows: ResMut<Windows>,
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	// Camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	let window =
		windows
			.get_primary_mut()
			.unwrap();

	let pos_bottom =
		-window.height() / 2.0;
	// Spawn sprite
	commands.spawn_bundle
		(
			SpriteBundle
				{
					material: materials.add(server.load(SPACE_SHIP).into()),
					transform: Transform
						{
							translation: Vec3::new
								(
									0.0,
									pos_bottom + 75.0 / 2.0,
									10.0
								),

							..Default::default()
						},

					..Default::default()
				}
		);
}
