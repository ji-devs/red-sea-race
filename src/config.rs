// See ji-bytes project for how this could be used for CDN style
pub const MEDIA_URL:&'static str = "/media";

pub const STAGE_WIDTH:f64 = 2048.0;
pub const STAGE_HEIGHT:f64 = 1536.0;

pub const STAGE_RATIO:f64 = STAGE_WIDTH / STAGE_HEIGHT;

pub const CAMERA_DEPTH:f64 = 100.0;

pub const BG_LAYER_DEPTH_START: f64 = -50.0;
pub const BG_LAYER_VELOCITY:[f64;3] = [0.1, 0.2, 0.3];
pub const BG_SPRITE_DEPTH: f64 = -40.0;

pub const BG_SPRITE_SPAWN_THRESHHOLD:[f64;3] = [1200.0, 1600.0, 700.0];
pub const BG_SPRITE_SPAWN_VELOCITY_MINMAX:[(f64, f64);3] = [(0.4, 0.6), (0.2, 0.4), (0.2, 1.5)];
pub const BG_SPRITE_SPAWN_Y_MINMAX:[(f64, f64);3] = [(0.0, 1.0), (0.0, 10.0), (1000.0, 1500.0)];