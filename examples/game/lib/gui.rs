use crate::lib::{Enemy, GameState, Player};
use bevy::app::{App, Update};
use bevy::color::Color;
use bevy::color::palettes::css::YELLOW;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::text::TextFont;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;

pub struct GuiPlugin;
#[derive(Component)]
struct FpsText;
#[derive(Component)]
struct EnemiesCountText;
impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), spawn_debug_text);
        app.add_systems(
            Update,
            update_debug_text
                .run_if(in_state(GameState::InGame))
                .run_if(on_timer(Duration::from_secs_f32(0.2))),
        );
    }
}

fn spawn_debug_text(mut commands: Commands) {
    commands
        .spawn((
            Text::new("FPS:"),
            TextFont {
                font: Default::default(),
                font_size: 25.0,
                font_smoothing: Default::default(),
            },
            Node{
                top:Val::Px(0.),
                ..default()
            }
        ))
        .with_child((TextSpan::default(), FpsText));
    commands
        .spawn((
            Text::new("Enemies:"),
            TextFont {
                font: Default::default(),
                font_size: 25.0,
                font_smoothing: Default::default(),
            },
            Node{
                top:Val::Px(30.),
                ..default()
            }
        ))
        .with_child((TextSpan::default(), EnemiesCountText));
}

fn update_debug_text(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_query: Query<&mut TextSpan, With<FpsText>>,
    mut enemies_count_query: Query<&mut TextSpan, (With<EnemiesCountText>,Without<FpsText>)>,
    enemy_query: Query<&Transform, (With<Enemy>)>,

) {
    for mut span in &mut fps_query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                **span = format!("{value:.2}");
            }
        }
    }
    for mut span in &mut enemies_count_query {
        **span = enemy_query.iter().len().to_string();
    }
}
