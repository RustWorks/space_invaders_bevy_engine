use bevy::{
	prelude::*,
	math::Vec3,
	input::Input
};

use crate::assets::{
	Sprites,
	WindowSize,
	TIME_STEP
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
	fn build(
		&self,
		app: &mut AppBuilder
	) {
		app
			.add_startup_stage(
				"game_setup_actors",
				SystemStage::single(spawn.system())
			)
			.add_system(r#move.system())
			.add_system(shoot.system())
			.add_system(shoot_move.system());
	}
}

pub struct Player;
pub struct PlayerReady(bool);
pub struct Laser;

pub struct Speed(f32);
impl Default for Speed {
	fn default() -> Self {
		Self(500.0)
	}
}

pub fn spawn(
	mut cmds: Commands,
	sprites: Res<Sprites>,
	window: Res<WindowSize>
) {
	let pos_btm = -window.h / 2.0;

	// Spawn ferris
	cmds.spawn_bundle
		(
			SpriteBundle {
				material: sprites.ferris.clone(),
				transform: Transform {
					translation: Vec3::new(0.0, pos_btm + 70.0 / 2.0 + 5.0, 10.0),
					scale: Vec3::new(0.8, 0.8, 1.1),

					..Default::default()
				},
				..Default::default()
			}
		)
		.insert(Player)
		.insert(PlayerReady(true))
		.insert(Speed::default());
}

pub fn r#move(
	kbd: Res<Input<KeyCode>>,
	mut query: Query<
		(
			&Speed,
			&mut Transform,
		),
		With<Player>
	>
) {
	if let Ok((speed, mut transform)) =
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
	sprites: Res<Sprites>,
	mut cmds: Commands,
	mut query: Query<
		(
			&Transform,
			&mut PlayerReady,
		),
		With<Player>
	>
) {
	if let Ok((player_tf, mut ready)) =
		query.single_mut() {
			if ready.0 && kbd.pressed(KeyCode::Space) {
				let x =
					player_tf.translation.x;
				let y =
					player_tf.translation.y;

				cmds.spawn_bundle
					(
						SpriteBundle {
							material: sprites.laser.clone(),
							transform: Transform {
								translation: Vec3::new(x, y + 50.0, 0.0),

								..Default::default()
							},
							..Default::default()
						}

					)
					.insert(Laser)
					.insert(Speed::default());

				ready.0 = false;
			}
			if kbd.just_released(KeyCode::Space) {
				ready.0 = true;
			}
		}
}

pub fn shoot_move(
	window: Res<WindowSize>,
	mut cmds: Commands,
	mut query: Query<
		(
			Entity,
			&Speed,
			&mut Transform,
		),
		With<Laser>
	>
) {
	for (
		laser_entity,
		speed,
		mut laser_tf,
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
