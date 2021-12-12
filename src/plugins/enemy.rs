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
	sprites: Res<Sprites>,
	window: Res<WindowSize>,
	mut cmds: Commands,
	mut active: ResMut<ActiveEnemies>,
) {
	if active.0 < 1 {
		let mut rng =
			thread_rng();

		let w_span =
			window.w / 2.0 - 100.0;

		let h_span =
			window.h / 2.0 - 100.0;

		let x =
			rng.gen_range(-w_span..w_span) as f32;

		let y =
			rng.gen_range(-h_span..h_span) as f32;

		cmds.spawn_bundle(
			SpriteBundle {
				material: sprites.gopher.clone(),
				transform: Transform {
					translation: Vec3::new(x, y, 10.0),
					scale: Vec3::new(SCALE, SCALE, 1.0),

					..Default::default()
				},
				..Default::default()
			}
		).insert(Enemy);

		active.0 += 1;
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
	mut active: ResMut<ActiveEnemies>
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

			let collision =
				collide(
					blue_laser_trans.translation,
					blue_laser_sprite.size * blue_laser_scale,
					enemy_trans.translation,
					enemy_sprite.size * enemy_scale,
				);

			if let Some(_) = collision {
				cmds
					.entity(enemy_entity)
					.despawn();

				active.0 -= 1;

				cmds
					.entity(blue_laser_entity)
					.despawn();
			}
		}
	}
}
