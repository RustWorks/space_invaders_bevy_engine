// TODO: Implement shooting system and movement for enemys

use bevy::{
	prelude::*,
	sprite::collide_aabb::collide
};
use rand::*;

use crate::types::*;

// Enemy's types
pub struct Enemy;
pub struct ActiveEnemies(pub u32);

// Spawn enemy's sprite
pub fn enemy_spawn(
	actor: Res<LoadActor>,
	win_size: Res<GetWinSize>,
	mut cmds: Commands,
	mut active_enemy: ResMut<ActiveEnemies>,
) {
	if active_enemy.0 < 1 {
		let mut rng =
			thread_rng();

		let w_span =
			win_size.w / 2.0 - 100.0;

		let h_span =
			win_size.h / 2.0 - 100.0;

		let x =
			rng.gen_range(-w_span..w_span) as f32;

		let y =
			rng.gen_range(-h_span..h_span) as f32;

		cmds.spawn_bundle(
			SpriteBundle {
				material: actor.gopher.clone(),
				transform: Transform {
					translation: Vec3::new(x, y, 10.0),
					scale: Vec3::new(SCALE, SCALE, 1.0),

					..Default::default()
				},
				..Default::default()
			}
		).insert(Enemy);

		active_enemy.0 += 1;
	}
}

pub fn enemy_despawn(
	mut cmds: Commands,
	mut blue_laser_query: Query<
		(
			Entity,
			&Transform,
			&Sprite,
		),
		With<Laser>,
	>,
	mut enemy_query: Query<
		(
			Entity,
			&Transform,
			&Sprite,
		),
		With<Enemy>,
	>,
	mut active_enemy: ResMut<ActiveEnemies>
) {
	for (
		blue_laser_entity,
		blue_laser_trans,
		blue_laser_sprite
	) in blue_laser_query.iter_mut() {
		for (
			enemy_entity,
			enemy_trans,
			enemy_sprite
		) in enemy_query.iter_mut() {
			let blue_laser_scale =
				Vec2::from(blue_laser_trans.scale);

			let enemy_scale =
				Vec2::from(enemy_trans.scale);

			let on_collision =
				collide(
					blue_laser_trans.translation,
					blue_laser_sprite.size * blue_laser_scale,
					enemy_trans.translation,
					enemy_sprite.size * enemy_scale,
				);

			if let Some(_) = on_collision {
				cmds.entity(enemy_entity).despawn();

				active_enemy.0 -= 1;

				cmds.entity(blue_laser_entity).despawn();
			}
		}
	}
}
