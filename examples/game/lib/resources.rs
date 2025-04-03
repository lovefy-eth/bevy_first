use bevy::a11y::AccessibilitySystem::Update;
use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::image::Image;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{in_state, Camera, GlobalTransform, IntoSystemConfigs, NextState, OnEnter, Query, Res, ResMut, Resource, Single, TextureAtlasLayout, Window};
use crate::lib::{GameState, SPRITE_SHEET_H, SPRITE_SHEET_PATH, SPRITE_SHEET_W, TILE_H, TILE_W};

#[derive(Resource)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}
#[derive(Resource)]
pub struct CurPosition(pub(crate) Option<Vec2>);

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GlobalTextureAtlas::default());
        app.insert_resource(CurPosition(None));
        app.add_systems(OnEnter(GameState::Loading), load_assets);
        app.add_systems(bevy::app::Update, update_cursor_position.run_if(in_state(GameState::InGame)));
    }
}

impl Default for GlobalTextureAtlas{
    fn default() -> Self {
        Self{
            layout:None,
            image:None,
        }
    }
}

fn load_assets(
    mut handle:ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut state: ResMut<NextState<GameState>>,
) {
    handle.image = Some(asset_server.load(SPRITE_SHEET_PATH));

    // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(TILE_W, TILE_H),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    handle.layout = Option::from(texture_atlas_layouts.add(layout));
    state.set(GameState::GameInit);
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
