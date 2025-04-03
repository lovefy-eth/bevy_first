use std::f32::consts::PI;
use bevy::input::mouse::MouseButtonInput;
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
const SPRITE_SPEED: f32 = 2.0;

//GUN
const BULLET_SPEED: f32 = 2.0;
const BULLET_SPAWN_INTERVAL: f32 = 0.1;

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
#[derive(Resource)]
struct CurPosition(Option<Vec2>);
#[derive(Component)]
struct GunTimer(Timer);


// Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gun;
#[derive(Component)]
struct Bullet;
#[derive(Component)]
struct BulletDirection(Vec3);


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
        .insert_resource(CurPosition(None))
        //.insert_resource(GunTimer(Timer::from_seconds(0.1,TimerMode::Repeating)))
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin)
        // systems
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(OnEnter(GameState::GameInit), (setup_camera, init_world))
        .add_systems(
            Update,
            (
                close_on_esc,
                (
                    player_input_system,
                    update_gun_transform,
                    update_gun_input,
                    update_cursor_position,
                    update_bullet,
                )
                    .run_if(in_state(GameState::InGame)),
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
    commands.spawn((
        Sprite {
            image: image_handle.0.clone().unwrap(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 8,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
        Gun,
        GunTimer(Timer::from_seconds(BULLET_SPAWN_INTERVAL,TimerMode::Repeating)),
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

fn update_bullet(
    mut bullet_query: Query<(&mut Transform, &mut Bullet,&BulletDirection)>,
){
    if bullet_query.is_empty(){
        return;
    }
    for (mut transform, mut bulled, direction) in bullet_query.iter_mut(){
        transform.translation+=direction.0.normalize() * SPRITE_SPEED;
    }
}
fn update_gun_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform,&mut GunTimer), With<Gun>>,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
){
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }
    let (gun_transform, mut gun_time) = gun_query.single_mut();
    if  gun_time.0.tick(time.delta()).finished() {
        let gun_pos = gun_transform.translation;
        commands.spawn((
            Sprite {
                image: image_handle.0.clone().unwrap(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas.0.clone().unwrap(),
                    index: 9,
                }),
                ..default()
            },
            Transform::from_translation(gun_pos),
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
    let angle = to_player.y.atan2(to_player.x) ;
    let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.0));
    gun_transform.rotation = rotate_to_player;//Quat::from_rotation_z(angle);
    let offset = 60.0;
    gun_transform.translation = player_pos + vec3(
        offset*angle.cos(),
        offset*angle.sin(),
        0.0);
}

fn update_cursor_position(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut cursor_pos: ResMut<CurPosition>,
) {
    let (camera, camera_transform) = *camera_query;

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    cursor_pos.0 = Some(point);
}
