use bevy::app::{App, Update};
use bevy::input::ButtonInput;
use bevy::math::{vec3, Vec2};
use bevy::prelude::{in_state, Component, IntoSystemConfigs, KeyCode, Plugin, Query, Res, Transform, With};
use crate::lib::{GameState, SPRITE_SPEED};

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,player_input_system.run_if(in_state(GameState::InGame)));
    }
}

fn player_input_system(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }
    let mut delta = Vec2::ZERO;
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    if w_key || a_key || s_key || d_key || d_key {
        if w_key {
            delta.y += 1.0;
        }
        if a_key {
            delta.x -= 1.0;
        }
        if s_key {
            delta.y -= 1.0;
        }
        if d_key {
            delta.x += 1.0;
        }
        delta = delta.normalize_or_zero();
        if delta.is_finite() {
            let mut transform = player_query.single_mut();
            transform.translation += vec3(delta.x, delta.y, 0.0) * SPRITE_SPEED;
        }
    }
}