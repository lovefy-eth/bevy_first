use bevy::color::palettes::basic::{RED,YELLOW};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new(String::from("Hello, world!")),
        Node{
            position_type: PositionType::Absolute,
            bottom:Val::Px(300.),
            right:Val::Px(10.),
            ..Default::default()
        },
        TextColor(RED.into())
    )).with_child((
        TextSpan::new("Hello, world!"),
        #[cfg(feature = "default_font")]
        TextColor(YELLOW.into())
        ));
}
