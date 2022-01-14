#![allow(clippy::cast_precision_loss, clippy::suboptimal_flops)]

use ggez::graphics::{Color, Rect};

use crate::blocks::BlockColor;

// APP
pub const APP_NAME: &str = "Color Columns";
pub const BUILD_TIME: &str = include!(concat!(env!("OUT_DIR"), "/build-time"));

// WINDOW
//pub const WINDOW_TITLE: &str = APP_NAME;
pub const WINDOW_WIDTH: f32 = 400.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

// COLORS
pub const COLOR_GRAY: Color = Color::new(1.0, 1.0, 1.0, 0.2);
pub const COLOR_LIGHT_GRAY: Color = Color::new(1.0, 1.0, 1.0, 0.4);
pub const COLOR_RED: Color = Color::new(1.0, 0.0, 0.1, 1.0);
pub const COLOR_GREEN: Color = Color::new(0.0, 0.72, 0.0, 1.0);
pub const COLOR_BLUE: Color = Color::new(0.0, 0.15, 1.0, 1.0);
pub const COLOR_MAGENTA: Color = Color::new(0.7, 0.0, 0.65, 1.0);
pub const COLOR_ORANGE: Color = Color::new(1.0, 0.45, 0.0, 1.0);
pub const COLOR_YELLOW: Color = Color::new(0.85, 0.75, 0.0, 1.0);

// BLOCKS
pub const BLOCK_SIZE: f32 = 23.0;
pub const NO_BLOCK_CODE: char = '.';
pub const BLOCK_COLOR_RED: BlockColor = BlockColor {
    code: 'R',
    color: COLOR_RED,
};
pub const BLOCK_COLOR_GREEN: BlockColor = BlockColor {
    code: 'G',
    color: COLOR_GREEN,
};
pub const BLOCK_COLOR_BLUE: BlockColor = BlockColor {
    code: 'B',
    color: COLOR_BLUE,
};
pub const BLOCK_COLOR_CYAN: BlockColor = BlockColor {
    code: 'C',
    color: COLOR_ORANGE,
};
pub const BLOCK_COLOR_MAGENTA: BlockColor = BlockColor {
    code: 'M',
    color: COLOR_MAGENTA,
};
pub const BLOCK_COLOR_YELLOW: BlockColor = BlockColor {
    code: 'Y',
    color: COLOR_YELLOW,
};

// MAIN MENU
pub const MAIN_MENU_ITEM_CHAR_SCALE: f32 = 30.0;
pub const MAIN_MENU_ITEM_AREA_X: f32 = 50.0;
pub const MAIN_MENU_ITEM_AREA_WIDTH: f32 = 300.0;
pub const MAIN_MENU_ITEM_AREA_CENTER: f32 = MAIN_MENU_ITEM_AREA_X + MAIN_MENU_ITEM_AREA_WIDTH / 2.0;
pub const MAIN_MENU_SELECTED_ITEM_BLOCK_SIZE: f32 = 18.0;
pub const MAIN_MENU_SELECTED_ITEM_BLOCK_FADE_IN_TRESHOLD: f32 = 0.75;
pub const MAIN_MENU_SELECTED_ITEM_BLOCK_MARGIN_X: f32 = MAIN_MENU_SELECTED_ITEM_BLOCK_SIZE * 0.725;
pub const MAIN_MENU_SELECTED_ITEM_BLOCK_MARGIN_Y: f32 = 6.0;
pub const MAIN_MENU_ITEM_DELTA_Y: f32 = 125.0;
pub const MAIN_MENU_TOP_ITEM_Y: f32 = 160.0;
pub const MAIN_MENU_ITEMS_Y_POSITIONS: [f32; 3] = [
    MAIN_MENU_TOP_ITEM_Y,
    MAIN_MENU_TOP_ITEM_Y + MAIN_MENU_ITEM_DELTA_Y,
    MAIN_MENU_TOP_ITEM_Y + MAIN_MENU_ITEM_DELTA_Y * 2.0,
];
pub const MAIN_MENU_SELECTED_ITEM_BLOCK_ALPHA_INCREMENT_ACCELERATION: f32 = 0.001;

// NAVIGATION INSTRUCTIONS
pub const NAVIGATION_INSTRUCTIONS_CHAR_SCALE: f32 = 19.0;

// HOW TO PLAY & ABOUT
pub const GO_BACK_LABEL_POSITION: (f32, f32) = (GAME_ARENA_RECT.left() + 50.0, 30.0);
pub const HOWTOPLAY_AND_ABOUT_AREA_WIDTH: f32 = 300.0;
pub const HOWTOPLAY_AND_ABOUT_TEXT_POSITION_X: f32 = 73.0;
pub const HOWTOPLAY_CONTROLS_CHAR_SCALE: f32 = 22.0;
pub const HOWTOPLAY_CONTROLS_TEXT_POSITION_Y: f32 = 105.0;
pub const HOWTOPLAY_CONTROLS_LEFTSIDE_TEXT_POSITION_X: f32 = HOWTOPLAY_AND_ABOUT_TEXT_POSITION_X;
pub const HOWTOPLAY_CONTROLS_RIGHTSIDE_TEXT_POSITION_X: f32 =
    HOWTOPLAY_AND_ABOUT_TEXT_POSITION_X + 170.0;
pub const HOWTOPLAY_LINE_DELIMITER_WIDTH: f32 = 1.02;
pub const HOWTOPLAY_LINE_DELIMITER_START_POSITION_X: f32 = 147.0;
pub const HOWTOPLAY_LINE_DELIMITER_END_POSITION_X: f32 = 253.0;
pub const HOWTOPLAY_LINE_DELIMITER_POSITION_Y: f32 = 265.0;
pub const HOWTOPLAY_SCORING_CHAR_SCALE: f32 = 19.5;
pub const HOWTOPLAY_SCORING_RULES_TEXT_POSITION: (f32, f32) =
    (HOWTOPLAY_AND_ABOUT_TEXT_POSITION_X, 275.0);
pub const ABOUT_CHAR_SCALE: f32 = 22.0;
pub const ABOUT_TEXT_POSITION: (f32, f32) = (HOWTOPLAY_AND_ABOUT_TEXT_POSITION_X, 155.0);
pub const ABOUT_VERSION_AND_BUILDTIME_CHAR_SCALE: f32 = 17.0;
pub const ABOUT_VERSION_AND_BUILDTIME_POSITION: (f32, f32) =
    (HOWTOPLAY_AND_ABOUT_TEXT_POSITION_X, 470.0);
pub const ABOUT_VERSION_AND_BUILDTIME_AREA_WIDTH: f32 = 260.0;

// HUD
pub const HUD_LABEL_PLAYING_STATE_CHAR_SCALE: f32 = 40.0;
pub const HUD_LABEL_PLAYING_STATE_POSITION: (f32, f32) =
    (GAME_ARENA_RECT.left() + 1.0, GAME_ARENA_MARGIN);
pub const NUM_TICKS_FOR_PLAYING_STATE_GO_BLINKING: usize = 18;
pub const NUM_TICKS_FOR_PLAYING_STATE_SPEEDUP_BLINKING: usize = 15;
pub const HUD_LABEL_INSTRUCTIONS_POSITION: (f32, f32) = (GAME_ARENA_RECT.left() + 1.0, 85.0);
pub const HUD_LABEL_SCORING_CHAR_SCALE: f32 = 19.0;
pub const HUD_LABEL_SCORING_POSITION_X: f32 = GAME_ARENA_MARGIN;
pub const HUD_LABEL_SCORING_DELTA_POSITION_Y: f32 = 80.0;
pub const HUD_LABEL_SCORE_POSITION_Y: f32 = 345.0;
pub const HUD_LABEL_SCORE_POSITION: (f32, f32) =
    (HUD_LABEL_SCORING_POSITION_X, HUD_LABEL_SCORE_POSITION_Y);
pub const HUD_LABEL_MAXCOMBO_POSITION: (f32, f32) = (
    HUD_LABEL_SCORING_POSITION_X,
    HUD_LABEL_SCORE_POSITION_Y + HUD_LABEL_SCORING_DELTA_POSITION_Y,
);
pub const HUD_LABEL_HIGHSCORE_POSITION: (f32, f32) = (
    HUD_LABEL_SCORING_POSITION_X,
    HUD_LABEL_SCORE_POSITION_Y + HUD_LABEL_SCORING_DELTA_POSITION_Y * 2.0,
);
pub const HUD_SCORING_CHAR_SCALE: f32 = HUD_LABEL_SCORING_CHAR_SCALE;
pub const HUD_SCORING_POSITION_X: f32 = HUD_LABEL_SCORING_POSITION_X + 1.0;
pub const HUD_SCORING_DELTA_POSITION_Y: f32 = 20.0;
pub const HUD_SCORE_POSITION: (f32, f32) = (
    HUD_SCORING_POSITION_X,
    HUD_LABEL_SCORE_POSITION_Y + HUD_SCORING_DELTA_POSITION_Y,
);
pub const HUD_MAXCOMBO_POSITION: (f32, f32) = (
    HUD_SCORING_POSITION_X,
    HUD_LABEL_MAXCOMBO_POSITION.1 + HUD_SCORING_DELTA_POSITION_Y,
);
pub const HUD_HIGHSCORE_POSITION: (f32, f32) = (
    HUD_SCORING_POSITION_X,
    HUD_LABEL_HIGHSCORE_POSITION.1 + HUD_SCORING_DELTA_POSITION_Y,
);

// GAME ARENA
pub const GAME_ARENA_COLUMNS: usize = 9;
pub const GAME_ARENA_ROWS: usize = 18;
pub const GAME_ARENA_MARGIN: f32 = 30.0;
pub const GAME_ARENA_RECT: Rect = Rect::new(
    WINDOW_WIDTH - GAME_ARENA_COLUMNS as f32 * BLOCK_SIZE - GAME_ARENA_MARGIN,
    WINDOW_HEIGHT - GAME_ARENA_ROWS as f32 * BLOCK_SIZE - GAME_ARENA_MARGIN,
    GAME_ARENA_COLUMNS as f32 * BLOCK_SIZE,
    GAME_ARENA_ROWS as f32 * BLOCK_SIZE,
);
pub const GAME_ARENA_MARGIN_LEFT: f32 = BLOCK_SIZE * 2.0 + 4.0;

// GAMEPLAY
pub const FPS: u32 = 60;
//pub const NUM_TICKS_FOR_NEW_CARGO: usize = 20;    // TODO ?
pub const STARTING_NUM_TICKS_FOR_CARGO_DESCENT: usize = 45;
pub const NUM_DESCENDED_CARGOES_GAMEPLAY_ACCELERATION: usize = 9;
pub const NUM_TICKS_GAMEPLAY_ACCELERATION_LIMIT: usize = 18;
pub const NUM_TICKS_SEQUENCE_FOR_MATCHES_REMOVAL: [usize; 4] = [12, 11, 11, 24];
pub const NUM_TICKS_FOR_PAUSED_BLOCKS_SHUFFLE: usize = 8;
