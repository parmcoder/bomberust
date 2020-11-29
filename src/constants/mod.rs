// game dimensions
pub const GAME_WIDTH: f32 = 360.0;
pub const GAME_HEIGHT: f32 = 270.0;

// gamemaster
pub const STARTING_TICK: usize = 0;
pub const STARTING_PHASE_IDX: usize = 0;
pub const TICK_LENGTH: f32 = 1.0;

// arena dimensions, area excluding the side bars
pub const ARENA_MIN_Y: f32 = 0.0;
pub const ARENA_MAX_Y: f32 = GAME_HEIGHT;
pub const ARENA_MIN_X: f32 = GAME_WIDTH / 8.0;
pub const ARENA_MAX_X: f32 = GAME_WIDTH - ARENA_MIN_X;
pub const ARENA_WIDTH: f32 = ARENA_MAX_X - ARENA_MIN_X;
pub const ARENA_HEIGHT: f32 = ARENA_MAX_Y - ARENA_MIN_Y;
pub const ARENA_SPAWN_OFFSET: f32 = 20.0;

// camera
pub const CAMERA_X: f32 = GAME_WIDTH * 0.5;
pub const CAMERA_Y: f32 = GAME_HEIGHT * 0.5;
pub const CAMERA_Z: f32 = 237.0;
//pub const CAMERA_Z: f32 = 500.0;

pub const BOARD_WIDTH: u32 = 10;
pub const BOARD_HEIGHT: u32 = 20;

pub const BOUNCE_SOUND: &str = "audio/bounce.ogg";
pub const SCORE_SOUND: &str = "audio/score.ogg";
pub const MUSIC_TRACKS: &[&str] = &[
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];