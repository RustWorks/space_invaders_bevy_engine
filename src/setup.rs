use {
	bevy::{
		prelude::*,
		window::WindowId,
		winit::WinitWindows,
	},
	winit::window::Icon
};

const WINDOW_ICON: &str = "assets/icon.jpg";

pub fn setup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	windows: Res<WinitWindows>
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

	// Set window icon
	let primary =
		windows.get_window
			(
				WindowId::primary()
			).unwrap();

	let (icon_rgba, icon_width, icon_height) =
		{
			let image =
				image::open(WINDOW_ICON)
					.expect("Failed to open icon path")
					.into_rgba8();

			let (width, height) =
				image.dimensions();

			let rgba =
				image.into_raw();

			(rgba, width, height)
		};

	let icon =
		Icon::from_rgba
			(
				icon_rgba,
				icon_width,
				icon_height
			).unwrap();

	primary.set_window_icon(Some(icon));
}
