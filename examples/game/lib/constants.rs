pub const SPRITE_SHEET_PATH: &str = "gabe-idle-run.png";
pub const TILE_W: u32 = 24;
pub const TILE_H: u32 = 24;
pub const SPRITE_SHEET_W: u32 = 7;
pub const SPRITE_SHEET_H: u32 = 3;
pub const WW: f32 = 1200.0;
pub const WH: f32 = 900.0;

// Sprites
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const SPRITE_SPEED: f32 = 5.0;
// World
pub const NUM_WORLD_DECORATIONS: usize = 1000;
pub const WORLD_W:f32 = 3000.;
pub const WORLD_H:f32 = 4000.;

//GUN
pub const BULLET_SPEED: f32 = 20.0;//子弹飞行速度
pub const BULLET_SPAWN_INTERVAL: f32 = 0.1;//子弹冷却时间
pub const BULLET_TIME_SECONDS: f32 = 1.; //子弹飞行时间
pub const BULLET_DAMAGE:f32 = 50.; //子弹伤害
pub const BULLET_PER_SHOT_NUM:usize=6;

// Colors
pub const BG_COLOR: (u8, u8, u8) = (192, 204, 184);

// CAMERA
pub const CAMERA_DECAY_RATE: f32 = 2.;

// Enemy
pub const NUM_ENEMIES: usize = 1;
pub const ENEMY_SPAWN_INTERCAL:f32 = 1.;
pub const MAX_ENEMIES: usize = 100_000;
pub const ENEMY_SPEED:f32 = 2.;