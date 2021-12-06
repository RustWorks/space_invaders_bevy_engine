#![allow(unused)] // shut up rust-analyzer

use {
    bevy::{
        prelude::{
			App, Color,
            Res, IntoSystem,
            WindowDescriptor,
            DefaultPlugins,
        },
        window::WindowId,
        winit::WinitWindows,
        render::pass::ClearColor,
    },
    winit::window::Icon
};

// copied from beavy cheat-book
fn set_icon(
    windows: Res<WinitWindows>,
) {
    let primary=
		windows.get_window
			(
				WindowId::primary()
            ).unwrap();

    let (icon_rgba, icon_width, icon_height) =
        {
            let image =
                image::open("assets/icon.jpg")
                    .expect("Failed to open icon path")
                    .into_rgba8();

            let (width, height) =
                image.dimensions();

            let rgba =
                image.into_raw();

            (rgba, width, height)
    };

    let icon =
        Icon::from_rgba
            (
                icon_rgba,
                icon_width,
                icon_height
            ).unwrap();

    primary.set_window_icon(Some(icon));
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Space Invaders".to_string(),
            width: 598.0,
            height: 675.0,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(set_icon.system())
        .add_plugins(DefaultPlugins)
        .run();
}
