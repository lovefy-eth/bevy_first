use crate::lib::attribute::Health;
use crate::lib::{
    AnimationTimer, ENEMY_SPAWN_INTERCAL, ENEMY_SPEED, GameState, GlobalTextureAtlas, MAX_ENEMIES,
    NUM_ENEMIES, Player, RoleState, SPRITE_SCALE_FACTOR, WORLD_H, WORLD_W,
};
use bevy::app::{App, Update};
use bevy::math::{Vec3, vec2};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;
use std::f32::consts::PI;
use std::time::Duration;

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
    mut enemy_query: Query<(&mut Transform, &mut RoleState), (With<Enemy>, Without<Player>)>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };
    for (mut enemy, mut state) in enemy_query.iter_mut() {
        if player.translation.distance(enemy.translation).abs() < 500.0 {
            let dir = (player.translation - enemy.translation).normalize();
            enemy.translation += dir * ENEMY_SPEED;
            *state = RoleState::Moving;
        } else {
            *state = RoleState::Idle;
        }
    }
}
fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    if num_enemies >= MAX_ENEMIES {
        return;
    }
    let spawn_count = (MAX_ENEMIES - num_enemies).min(NUM_ENEMIES);
    //let mut rng = rand::rng();
    let player_transform = player_query.single();
    for _ in 0..spawn_count {
        // let mut x = rng.random_range(-WORLD_W..WORLD_W);
        // let mut y = rng.random_range(-WORLD_H..WORLD_H);

        // 在玩家附近生成怪物
        let (x, y) = get_random_position_around(player_transform.translation.truncate());
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
            RoleState::default(),
        ));
    }
}

fn get_random_position_around(pos: Vec2) -> (f32, f32) {
    let mut rng = rand::rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let distance = rng.gen_range(500.0..2000.0);
    (
        pos.x + angle.sin() * distance,
        pos.y + angle.cos() * distance,
    )
}
