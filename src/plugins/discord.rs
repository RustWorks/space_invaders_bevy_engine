// FIXME: Inspect why this hangs game.

use std::error::Error;

use bevy::prelude::In;
use discord_rich_presence::{
	activity,
	new_client,
	DiscordIpc
};

pub fn presence() -> Result<(), Box<dyn Error>> {
    let mut client =
		new_client("919271943843237919")?;

    client.connect()?;

    client.set_activity(
		activity::Activity::new()
            .state("foo")
            .details("bar")
    )?;

    client.close()?;

    Ok(())
}

pub fn presence_error(
	In(result): In<Result<(), Box<dyn Error>>>
) {
	if let Err(e) = result {
		eprint!("failed to get presence: {}", e)
	}
}
