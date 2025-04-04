use crate::lib::{ENEMY_SPAWN_INTERCAL, GameState, GlobalTextureAtlas, MAX_ENEMIES, NUM_ENEMIES, Player, SPRITE_SCALE_FACTOR, WORLD_H, WORLD_W, ENEMY_SPEED, AnimationTimer};
use bevy::app::{App, Update};
use bevy::math::{Vec3, vec2};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;
use std::time::Duration;
use crate::lib::attribute::Health;

#[derive(Component)]
pub struct Enemy;
pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(OnEnter(GameState::InGame),spawn_enemies);
        app.add_systems(
            Update,
            (
                update_enemy_transform,
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERCAL))),
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}
fn update_enemy_transform(
    player: Query<&Transform, With<Player>>,
    mut enemy_query:Query<&mut Transform,(With<Enemy>,Without<Player>)>,

) {
    let Ok(player) = player.get_single() else {
        return;
    };
    for mut enemy in enemy_query.iter_mut() {
        let dir = (player.translation-enemy.translation).normalize();
        enemy.translation+=dir*ENEMY_SPEED;
    }


}
fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    if num_enemies >= MAX_ENEMIES {
        return;
    }
    let spawn_count = (MAX_ENEMIES - num_enemies).min(NUM_ENEMIES);
    let mut rng = rand::rng();
    for _ in 0..spawn_count {
        let x = rng.random_range(-WORLD_W..WORLD_W);
        let y = rng.random_range(-WORLD_H..WORLD_H);
        commands.spawn((
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 14,
                }),
                ..default()
            },
            Transform::from_translation(vec2(x, y).extend(1.))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Enemy,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Health::default(),
        ));
    }
}
