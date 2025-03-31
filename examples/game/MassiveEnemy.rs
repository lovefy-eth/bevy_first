use bevy::math::vec3;
use bevy::prelude::*;
// Window
const SPRITE_SHEET_PATH: &str = "gabe-idle-run.png";
const TILE_W: u32 = 24;
const TILE_H: u32 = 24;
const SPRITE_SHEET_W: u32 = 7;
const SPRITE_SHEET_H: u32 = 2;
const WW: f32 = 1200.0;
const WH: f32 = 900.0;

// Sprites
const SPRITE_SCALE_FACTOR: f32 = 3.0;
const SPRITE_SPEED:f32 = 2.0;

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

// Components
#[derive(Component)]
struct Player;

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
        .add_systems(OnEnter(GameState::GameInit), (setup_camera, init_world))
        .add_systems(
            Update,
            (
                close_on_esc,
                player_input_system.run_if(in_state(GameState::InGame)),
            ),
        )
        .run();
}

fn load_assets(
    mut texture_atlas_handle: ResMut<GlobalTextureAtlasHandle>,
    mut image_handles: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut state: ResMut<NextState<GameState>>,
) {
    image_handles.0 = Some(asset_server.load(SPRITE_SHEET_PATH));

    // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    texture_atlas_handle.0 = Option::from(texture_atlas_layouts.add(layout));
    state.set(GameState::GameInit);
}

fn init_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
    mut state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Sprite {
            image: image_handle.0.clone().unwrap(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        Player,
    ));
    state.set(GameState::InGame);
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

fn player_input_system(
    mut player_query: Query<&mut Transform,With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
    if player_query.is_empty(){
        return;
    }
    let mut delta = Vec2::ZERO;
    let w_key = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let a_key = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);
    let s_key = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let d_key = keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
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

        let mut transform = player_query.single_mut();
        transform.translation+=vec3(delta.x, delta.y, 0.0) * SPRITE_SPEED;
    }
}
