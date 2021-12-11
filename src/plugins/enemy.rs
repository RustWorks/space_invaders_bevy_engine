// FIXME: Inspect why program panic instead of loading enemy

use bevy::prelude::*;
use rand::*;

use crate::setup::*;

pub struct Enemy;
pub struct ActiveEnemies(pub u32);

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
			rng.gen_range(
				- w_span .. w_span
			) as f32;

		let y =
			rng.gen_range(
				- h_span .. h_span
			) as f32;

		cmds.spawn_bundle
			(
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
