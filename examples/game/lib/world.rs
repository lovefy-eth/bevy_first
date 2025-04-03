use crate::lib::{
    BULLET_SPAWN_INTERVAL, Decoration, GameState, GlobalTextureAtlas, Gun,
    GunTimer, NUM_WORLD_DECORATIONS, Player, SPRITE_SCALE_FACTOR, WORLD_H, WORLD_W,
};
use bevy::app::{App, Plugin};
use bevy::math::{Vec3, vec2};
use bevy::prelude::*;
use rand::Rng;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(
            OnEnter(GameState::GameInit),
            (init_world, spawn_world_decorations),
        );

    }
}
fn init_world(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Sprite {
            image: handle.image.clone().unwrap(),
            texture_atlas: Some(TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        Player,
    ));
    commands.spawn((
        Sprite {
            image: handle.image.clone().unwrap(),
            texture_atlas: Some(TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 8,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        Gun,
        GunTimer(Timer::from_seconds(
            BULLET_SPAWN_INTERVAL,
            TimerMode::Repeating,
        )),
    ));

    state.set(GameState::InGame);
}

fn spawn_world_decorations(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    let mut rng = rand::rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.random_range(-WORLD_W..WORLD_W);
        let y = rng.random_range(-WORLD_H..WORLD_H);
        commands.spawn((
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 10,
                }),
                ..default()
            },
            Transform::from_translation(vec2(x, y).extend(0.))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Decoration,
        ));
    }
}


