use crate::lib::{CAMERA_DECAY_RATE, GameState, Player};
use bevy::app::{App, Plugin, Update};
use bevy::math::Vec3;
use bevy::prelude::{
    Camera2d, Commands, IntoSystemConfigs, OnEnter, Query, Res, StableInterpolate, Time, Transform,
    With, Without, in_state,
};
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default());
        app.add_systems(OnEnter(GameState::GameInit), setup_camera);
        app.add_systems(Update, update_camera.run_if(in_state(GameState::InGame)));
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d).insert(PanCam::default());
}
fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    // camera
    //     .translation
    //     .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());

    camera.translation.lerp(direction, CAMERA_DECAY_RATE);
}
