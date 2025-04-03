use crate::lib::{
    BULLET_SPEED, CurPosition, GameState, GlobalTextureAtlas, Player, SPRITE_SCALE_FACTOR,
};
use bevy::app::{App, Update};
use bevy::input::ButtonInput;
use bevy::math::{Quat, Vec3, vec3};
use bevy::prelude::{Commands, Component, IntoSystemConfigs, MouseButton, Plugin, Query, Res, Sprite, TextureAtlas, Time, Transform, Vec2Swizzles, Vec3Swizzles, With, Without, default, in_state, Timer};

#[derive(Component)]
pub struct Gun;
#[derive(Component)]
pub struct GunTimer(pub Timer);

#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct BulletDirection(Vec3);
#[derive(Component)]
pub struct Decoration;
pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_gun_transform, update_gun_input, update_bullet)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer), With<Gun>>,
    handle: Res<GlobalTextureAtlas>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }
    let (gun_transform, mut gun_time) = gun_query.single_mut();
    if gun_time.0.tick(time.delta()).finished() {
        let gun_pos = gun_transform.translation;
        commands.spawn((
            Sprite {
                image: handle.image.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.layout.clone().unwrap(),
                    index: 9,
                }),
                ..default()
            },
            Transform::from_translation(gun_pos).with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            Bullet,
            BulletDirection(*gun_transform.local_y()),
        ));
    }
}

fn update_gun_transform(
    cursor_pos: Res<CurPosition>,
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }
    let player_pos = player_query.single().translation;
    let mut gun_transform = gun_query.single_mut();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos.truncate(),
    };

    let to_player = (cursor_pos.xy() - player_pos.xy()).normalize();
    let angle = to_player.y.atan2(to_player.x);
    let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.0));
    gun_transform.rotation = rotate_to_player; //Quat::from_rotation_z(angle);
    let offset = 60.0;
    gun_transform.translation = player_pos + vec3(offset * angle.cos(), offset * angle.sin(), 0.0);
}
fn update_bullet(mut bullet_query: Query<(&mut Transform, &Bullet, &BulletDirection)>) {
    if bullet_query.is_empty() {
        return;
    }
    for (mut transform, _, direction) in bullet_query.iter_mut() {
        transform.translation += direction.0.normalize() * BULLET_SPEED;
    }
}
