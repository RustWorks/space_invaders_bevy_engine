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

pub fn player_movement(
	input: Res<Input<KeyCode>>,
	mut query: Query<
		(
			&Speed,
			&mut Transform,
		),
		With<Player>
	>
) {
	if let Ok((
		speed,
		mut transform
	)) = query.single_mut() {
			let dir_x =
				if input.pressed(KeyCode::A) {
					-0.7
				} else if input.pressed(KeyCode::D) {
					0.7
				} else {
					0.0
				};
				transform.translation.x += dir_x * speed.0 * TIME_STEP;

			let dir_y =
				if input.pressed(KeyCode::W) {
					0.7
				} else if input.pressed(KeyCode::S) {
					-0.7
				} else {
					0.0
				};
				transform.translation.y += dir_y * speed.0 * TIME_STEP;
		}
}

pub fn player_shooting(
	input: Res<Input<KeyCode>>,
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
	if let Ok((
		player_trans,
		mut player_ready
	)) = query.single_mut() {
			if player_ready.0 && input.pressed(KeyCode::Space) {
				let x =
					player_trans.translation.x;
				let y =
					player_trans.translation.y;

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

				player_ready.0 = false;
			}
			if input.just_released(KeyCode::Space) {
				player_ready.0 = true;
			}
		}
}

pub fn player_laser_movement(
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
		laser_speed,
		mut laser_trans,
	) in query.iter_mut() {
		let trans =
			&mut laser_trans.translation;

			trans.y += laser_speed.0 * TIME_STEP;
			if trans.y > window.h {
				cmds
					.entity(laser_entity)
					.despawn();
			}
	}
}
