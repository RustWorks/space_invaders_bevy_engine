use bevy::{
	prelude::*,
	math::Vec3,
	input::Input
};

use crate::types::*;

// Player's types
pub struct Player;
pub struct PlayerReady(pub bool);

pub fn player_spawn(
	actor: Res<LoadActor>,
	win_size: Res<GetWinSize>,
	mut cmds: Commands
) {
	let pos_btm = -win_size.h / 2.0;

	// Spawn player's sprite
	cmds.spawn_bundle
		(
			SpriteBundle {
				material: actor.ferris.clone(),
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

// Uses "WASD" for movement
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
		player_speed,
		mut player_trans
	)) = query.single_mut() {
			let dir_x =
				if input.pressed(KeyCode::A) {
					-0.7
				} else if input.pressed(KeyCode::D) {
					0.7
				} else {
					0.0
				};
				player_trans.translation.x += dir_x * player_speed.0 * TIME_STEP;

			let dir_y =
				if input.pressed(KeyCode::W) {
					0.7
				} else if input.pressed(KeyCode::S) {
					-0.7
				} else {
					0.0
				};
				player_trans.translation.y += dir_y * player_speed.0 * TIME_STEP;
		}
}

// Spawn laser sprite on
pub fn player_shooting(
	input: Res<Input<KeyCode>>,
	laser: Res<LoadLaser>,
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

				cmds.spawn_bundle(
					SpriteBundle {
						material: laser.red.clone(),
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
	win_size: Res<GetWinSize>,
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
		red_laser_entity,
		red_laser_speed,
		mut red_laser_trans,
	) in query.iter_mut() {
		let trans =
			&mut red_laser_trans.translation;

			trans.y += red_laser_speed.0 * TIME_STEP;
			if trans.y > win_size.h {
				cmds.entity(red_laser_entity).despawn();
			}
	}
}
