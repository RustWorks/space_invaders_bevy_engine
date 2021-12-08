use {
	bevy::{
		prelude::{
			Res, ResMut,
			With, Query,
			Commands, Transform,
			Assets, AssetServer,
			Handle, KeyCode, Windows,
			OrthographicCameraBundle,
			SpriteBundle, ColorMaterial
		},
		math::Vec3,
		input::Input
	}
};

const FERRIS: &str = r#"sprites\ferris.png"#;
const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Materials {
	ferris: Handle<ColorMaterial>
}

pub struct WindowSize {
	h: f32,
	w: f32
}

pub struct Player;
pub struct PlayerSpeed(f32);

impl Default for PlayerSpeed {
	fn default() -> Self {
		Self(500.0)
	}
}

pub fn setup(
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
			Materials {
				ferris: materials.add(asset_srv.load(FERRIS).into()),
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

pub fn spawn_player(
	mut cmds: Commands,
	sprite: Res<Materials>,
	window: Res<WindowSize>
) {
	let pos_btm = -window.h / 2.0;

	// Spawn ferris
	cmds.spawn_bundle
		(
			SpriteBundle {
				material: sprite.ferris.clone(),
				transform: Transform {
					translation: Vec3::new(0.0, pos_btm + 75.0 / 2.0, 10.0),
					scale: Vec3::new(0.8, 0.8, 1.1),

					..Default::default()
				},
				..Default::default()
			}
		)
		.insert(Player)
		.insert(PlayerSpeed::default);
}

pub fn player_movement(
	kbd: Res<Input<KeyCode>>,
	mut query: Query<(&PlayerSpeed, &mut Transform, With<Player>)>
) {
	if let Ok((speed, mut translation, _)) =
		query.single_mut() {
			let dir =
				if kbd.pressed(KeyCode::Left) {
					-1.0
				} else if kbd.pressed(KeyCode::Right) {
					1.0
				} else {
					0.0
				};
				translation.translation.x += dir * speed.0 * TIME_STEP;
		}
}
