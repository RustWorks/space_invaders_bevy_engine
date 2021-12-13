// FIXME: Inspect why this hangs game.

use std::error::Error;

use bevy::prelude::*;
use discord_rich_presence::{
	activity::{
		Assets, Activity
	},
	new_client,
	DiscordIpc
};

pub fn presence(
	mut is_connected: Local<bool>
) -> Result<(), Box<dyn Error>> {
	if *is_connected {
		return Ok(());
    }

    let mut client =
		new_client("919271943843237919")?;

    client.connect()?;

	let assets =
		Assets::new()
			.large_image("large-image")
			.large_text("ferris");

	let payload =
		Activity::new()
			.state("Space Invaders!")
			.details("Flying...")
			.assets(assets);

    client.set_activity(payload)?;

    *is_connected = true;

    Ok(())
}

pub fn presence_error(
	In(result): In<Result<(), Box<dyn Error>>>
) {
	if let Err(e) = result {
		eprint!("Failed to set presence: {}", e)
	}
}
