use {
	bevy::{
		prelude::{
			Res, ResMut,
			With, Query, Entity,
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
const LASER: &str = r#"sprites\laser.png"#;

const TIME_STEP: f32 = 1.0 / 60.0;

pub struct Materials {
	ferris: Handle<ColorMaterial>,
	laser: Handle<ColorMaterial>
}

#[allow(unused)]
pub struct WindowSize {
	h: f32,
	w: f32
}

pub struct Player;
pub struct Laser;

pub struct Speed(f32);
impl Default for Speed {
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

pub fn spawn(
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
					translation: Vec3::new(0.0, pos_btm + 70.0 / 2.0 + 5.0, 10.0),
					scale: Vec3::new(0.8, 0.8, 1.1),

					..Default::default()
				},
				..Default::default()
			}
		)
		.insert(Player)
		.insert(Speed::default());
}

pub fn player_move(
	kbd: Res<Input<KeyCode>>,
	mut query: Query<(
		&Speed,
		&mut Transform,
		With<Player>
	)>
) {
	if let Ok((speed, mut transform, _)) =
		query.single_mut() {
			let dir_x =
				if kbd.pressed(KeyCode::Left) || kbd.pressed(KeyCode::A) {
					-0.7
				} else if kbd.pressed(KeyCode::Right) || kbd.pressed(KeyCode::D) {
					0.7
				} else {
					0.0
				};
				transform.translation.x += dir_x * speed.0 * TIME_STEP;

			let dir_y =
				if kbd.pressed(KeyCode::Up) || kbd.pressed(KeyCode::W) {
					0.7
				} else if kbd.pressed(KeyCode::Down) || kbd.pressed(KeyCode::S) {
					-0.7
				} else {
					0.0
				};
				transform.translation.y += dir_y * speed.0 * TIME_STEP;
		}
}

pub fn shoot(
	kbd: Res<Input<KeyCode>>,
	sprite: Res<Materials>,
	mut cmds: Commands,
	mut query: Query<(
		&Transform,
		With<Player>
	)>
) {
	if let Ok((player_tf, _)) =
		query.single_mut() {
			if kbd.pressed(KeyCode::Space) {
				let x =
					player_tf.translation.x;
				let y =
					player_tf.translation.y;

				cmds.spawn_bundle
					(
						SpriteBundle {
							material: sprite.laser.clone(),
							transform: Transform {
								translation: Vec3::new(x, y, 0.0),

								..Default::default()
							},
							..Default::default()
						}

					)
					.insert(Laser)
					.insert(Speed::default);
			}
		}
}

pub fn shoot_move(
	window: Res<WindowSize>,
	mut cmds: Commands,
	mut query: Query<(
		Entity,
		&Speed,
		&mut Transform,
		With<Laser>
	)>
) {
	for (
		laser_entity,
		speed,
		mut laser_tf, _
	) in query.iter_mut() {
		let trans =
			&mut laser_tf.translation;

			trans.y += speed.0 * TIME_STEP;
			if trans.y > window.h {
				cmds
					.entity(laser_entity)
					.despawn();
			}
	}
}
