use crate::lib::attribute::Health;
use crate::lib::{BULLET_DAMAGE, Bullet, Enemy, GameState};
use bevy::app::{App, Update};
use bevy::prelude::{
    Commands, Entity, IntoSystemConfigs, Plugin, Query, Transform, With, Without, in_state,
};

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_enemy_bullet_collision.run_if(in_state(GameState::InGame)),
        );
    }
}

fn check_enemy_bullet_collision(
    mut commands: Commands,
    mut bullet_query: Query<(&Transform,Entity), With<Bullet>>,
    mut enemy_query: Query<(&Transform, Entity, &mut Health), (With<Enemy>, Without<Bullet>)>,
) {
    for (bullet,bullet_entity) in bullet_query.iter() {
        for (enemy_trans, entity, mut health) in enemy_query.iter_mut() {
            if enemy_trans.translation.distance_squared(bullet.translation) <= 1000. {
                commands.entity(bullet_entity).despawn();
                **health -= BULLET_DAMAGE;
                if health.0 <= 0. {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}
