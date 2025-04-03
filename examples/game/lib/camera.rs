use crate::lib::{CAMERA_DECAY_RATE, GameState, Player};
use bevy::app::{App, Plugin, Update};
use bevy::input::ButtonInput;
use bevy::math::{vec3, Vec3};
use bevy::prelude::{Camera2d, Commands, IntoSystemConfigs, OnEnter, Query, Transform, With, Without, in_state, KeyCode, Res, StableInterpolate};
use bevy::time::Time;
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
    commands.spawn(Camera2d).insert(PanCam {
        grab_buttons:vec![],
        ..Default::default()
    });
}
fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
    if keyboard_input.pressed(KeyCode::Space) {
        camera
            .translation
            .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
    } else {
        camera.translation = camera.translation.lerp(vec3(x,y,0.), 0.1);
    }
}
