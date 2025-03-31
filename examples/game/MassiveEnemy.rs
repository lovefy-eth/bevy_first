use std::string::ToString;
use bevy::color::palettes::css::{BLACK, GRAY};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::Texture;

// Window
const SPRITE_SHEET_PATH:String = "gabe-idle-run.png".to_string();
const TILE_W:u32 = 24;
const TILE_H:u32 = 24;
const WW: f32 = 1200.0;
const WH: f32 = 900.0;

// Sprites
const SPRITE_SCALE_FACTOR: f32 = 3.0;

// Colors
const BG_COLOR: (u8, u8, u8) = (192, 204, 184);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}

// Resources
#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);
#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WW, WH).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        // custom resources
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin)
        // systems
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(Startup, (setup_camera, spawn_player))
        .add_systems(Update, close_on_esc)
        .run();
}

fn load_assets(
    mut texture_atlas_handle: ResMut<GlobalTextureAtlasHandle>,
    mut image_handles: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    image_handles.0 = Some(asset_server.load(SPRITE_SHEET_PATH));

    // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(UVec2::new(TILE_W,TILE_H), 7, 1, None, None);
    texture_atlas_handle.0 = Option::from(texture_atlas_layouts.add(layout));
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // load the sprite sheet using the `AssetServer`
    let texture = asset_server.load(SPRITE_SHEET_PATH);

    // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(UVec2::new(TILE_W,TILE_H), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // the first (left-hand) sprite runs at 10 FPS

    // create the first (left-hand) sprite
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
