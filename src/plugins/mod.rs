pub mod enemy;
pub mod player;
pub mod discord;

use bevy::{
	prelude::*,
	core::FixedTimestep
};

use self::{
	discord::{
		presence,
		presence_error
	},
	enemy::{
		enemy_spawn,
		enemy_despawn,
		ActiveEnemies
	},
	player::{
		player_spawn,
		player_movement,
		player_shooting,
		player_laser_movement
	}
};

pub struct DiscordPlugin;
pub struct EnemyPlugin;
pub struct PlayerPlugin;

impl Plugin for DiscordPlugin {
	fn build(
		&self,
		app: &mut AppBuilder
	) {
		app.add_system(
			presence
				.system()
				.chain(
					presence_error.system()
				)
		);
	}
}

impl Plugin for EnemyPlugin {
	fn build(
		&self,
		app: &mut AppBuilder
	) {
		app
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(
						FixedTimestep::step(1.0)
					)
					.with_system(
						enemy_spawn.system()
					)
			)
			.insert_resource(
				ActiveEnemies(0)
			)
			.add_system(
				enemy_despawn.system()
			);
	}
}

impl Plugin for PlayerPlugin {
	fn build(
		&self,
		app: &mut AppBuilder
	) {
		app
			.add_startup_stage(
				"game_setup_actors",
				SystemStage::single(
					player_spawn.system()
				)
			)
			.add_system(
				player_movement.system()
			)
			.add_system(
				player_shooting.system()
			)
			.add_system(
				player_laser_movement.system()
			);
	}
}
