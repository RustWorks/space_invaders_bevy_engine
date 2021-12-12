pub mod enemy;
pub mod player;

use bevy::{
	prelude::*,
	core::FixedTimestep
};

use self::{
	enemy::*,
	player::*
};

pub struct EnemyPlugin;
pub struct PlayerPlugin;

impl Plugin for EnemyPlugin {
	fn build(
		&self,
		app: &mut AppBuilder
	) {
		app
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(1.0))
					.with_system(enemy_spawn.system())
			)
			.insert_resource(ActiveEnemies(0))
			.add_system(enemy_despawn.system());
	}
}

impl Plugin for PlayerPlugin {
	fn build(
		&self,
		app: &mut AppBuilder
	) {
		app
			.add_startup_stage(
				"setup_actors",
				SystemStage::single(
					player_spawn.system()
				)
			)
			.add_system(player_movement.system())
			.add_system(player_shooting.system())
			.add_system(player_laser_movement.system());
	}
}
