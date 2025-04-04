use crate::lib::{CurPosition, Enemy, GameState, Player, RoleState};
use bevy::prelude::*;
#[derive(Component)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin2;

impl Plugin for AnimationPlugin2 {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_player_animation, update_enemy_animation).run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_enemy_animation(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<
        (&mut Sprite, &Transform, &mut AnimationTimer, &RoleState),
        (With<Enemy>, Without<Player>),
    >,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };
    for (mut sprite, trans, mut timer,state) in enemy_query.iter_mut() {
        sprite.flip_x = player_transform.translation.x < trans.translation.x;
        if timer.0.tick(time.delta()).just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index  = match state {
                    RoleState::Moving => {
                        ((atlas.index - 14 + 1) % 7) + 14
                    }
                    RoleState::Idle => {
                        14
                    }
                };
            }
        }
    }
}
fn update_player_animation(
    cursor_pos: Res<CurPosition>,
    time: Res<Time>,
    mut player_query: Query<
        (&mut Sprite, &mut AnimationTimer, &Transform, &RoleState),
        With<Player>,
    >,
) {
    let Ok((mut player_sprite, mut animation_timer, transform, state)) =
        player_query.get_single_mut()
    else {
        return;
    };

    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => transform.translation.truncate(),
    };
    if animation_timer.0.tick(time.delta()).just_finished() {
        if let Some(atlas) = &mut player_sprite.texture_atlas {
            match state {
                RoleState::Idle => {
                    atlas.index = 0;
                }
                RoleState::Moving => {
                    atlas.index = (atlas.index + 1) % 7;
                }
            }
        }
        player_sprite.flip_x = cursor_pos.x < transform.translation.x;
    }
}
