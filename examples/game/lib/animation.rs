use crate::lib::{CurPosition, GameState, Player, PlayerState};
use bevy::prelude::*;
#[derive(Component)]
pub struct AnimationTimer(pub Timer);

pub struct AnimationPlugin2;

impl Plugin for AnimationPlugin2 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_player_animation.run_if(in_state(GameState::InGame)));
    }
}

fn update_player_animation(
    cursor_pos: Res<CurPosition>,
    time: Res<Time>,
    mut player_query: Query<(&mut Sprite, &mut AnimationTimer, &Transform,&PlayerState), With<Player>>,
) {
    let Ok((mut player_sprite, mut animation_timer, transform,state)) = player_query.get_single_mut()
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
                PlayerState::Idle => {
                    atlas.index = 0;
                }
                PlayerState::Moving => {
                    atlas.index = (atlas.index + 1) % 7;
                }
            }
        }
        player_sprite.flip_x = cursor_pos.x < transform.translation.x;


    }
}
