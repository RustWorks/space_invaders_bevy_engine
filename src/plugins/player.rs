use bevy::{
	prelude::*,
	math::Vec3,
	input::Input
};

use crate::setup::*;

pub struct Player;
pub struct PlayerReady(bool);

pub fn player_spawn(
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
					scale: Vec3::new(SCALE, SCALE, 1.1),

					..Default::default()
				},
				..Default::default()
			}
		)
		.insert(Player)
		.insert(PlayerReady(true))
		.insert(Speed::default());
}

pub fn movement(
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

pub fn shooting(
	kbd: Res<Input<KeyCode>>,
	lasers: Res<Lasers>,
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
							material: lasers.red.clone(),
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

pub fn laser_movement(
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
