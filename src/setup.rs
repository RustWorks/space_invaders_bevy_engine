use {
	bevy::{
		prelude::{
			ResMut, Commands,
			Color, ColorMaterial,
			OrthographicCameraBundle,
			Assets, Sprite, SpriteBundle,
		},
		math::Vec2,
	},
};

pub fn setup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	// Camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());

	// Spawn sprite
	commands.spawn_bundle
		(
			SpriteBundle
				{
					material: materials.add
						(
							Color::rgb(1.0, 0.7, 0.7).into()
						),
					sprite: Sprite::new
						(
							Vec2::new(200.0, 100.0)
						),

					..Default::default()
				}
		);
}
