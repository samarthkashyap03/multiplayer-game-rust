use macroquad::color::Color;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;

// World bounds
pub const WORLD_WIDTH: f32 = 1280.0;
pub const WORLD_BOTTOM_Y: f32 = 3000.0;

// Physics constants
pub const GRAVITY: f32 = 950.0;
pub const MOVE_SPEED: f32 = 240.0;
pub const JUMP_VELOCITY: f32 = -520.0;
pub const CLIMB_SPEED: f32 = 170.0;
pub const PLAYER_WIDTH: f32 = 30.0;
pub const PLAYER_HEIGHT: f32 = 46.0;

// Section checkpoints (Y-coordinates in descending world space)
pub const SECTION_1_START_Y: f32 = 2900.0;
pub const CHECKPOINT_1_Y: f32 = 2050.0;
pub const CHECKPOINT_2_Y: f32 = 1050.0;
pub const CHECKPOINT_3_Y: f32 = 180.0;

// Colors inspired by medieval Kaiserpfalz
pub const RED_SANDSTONE: Color = Color::new(0.72, 0.32, 0.24, 1.0);
pub const RED_SANDSTONE_DARK: Color = Color::new(0.55, 0.22, 0.16, 1.0);
pub const RED_SANDSTONE_LIGHT: Color = Color::new(0.85, 0.45, 0.35, 1.0);
pub const WOOD_BROWN: Color = Color::new(0.48, 0.30, 0.16, 1.0);
pub const WOOD_LIGHT: Color = Color::new(0.65, 0.42, 0.22, 1.0);
pub const WATER_BLUE: Color = Color::new(0.20, 0.45, 0.70, 0.85);
pub const GOLD_ACCENT: Color = Color::new(0.95, 0.78, 0.20, 1.0);
pub const PARCHMENT: Color = Color::new(0.96, 0.92, 0.82, 1.0);

// Player colors
pub const PLAYER_COLORS: [Color; 4] = [
    Color::new(0.90, 0.20, 0.20, 1.0), // P1: Barbarossa Red
    Color::new(0.20, 0.60, 0.90, 1.0), // P2: Lutra Water Blue
    Color::new(0.25, 0.75, 0.35, 1.0), // P3: Palatinate Green
    Color::new(0.95, 0.75, 0.15, 1.0), // P4: Habsburg Gold
];

pub const PLAYER_NAMES: [&str; 4] = [
    "P1: Barbarossa",
    "P2: Imperial Pike",
    "P3: Casimir",
    "P4: Rudolf",
];
