use glam::Vec2;

use ggez::{
    graphics::{self, Color, DrawParam, Font, PxScale, Text, TextFragment},
    Context,
};

use crate::{
    config::{
        COLOR_BLUE, COLOR_GREEN, COLOR_LIGHT_GRAY, COLOR_RED, COLOR_YELLOW, HUD_HIGHSCORE_POSITION,
        HUD_LABEL_HIGHSCORE_POSITION, HUD_LABEL_INSTRUCTIONS_POSITION, HUD_LABEL_MAXCOMBO_POSITION,
        HUD_LABEL_PLAYING_STATE_CHAR_SCALE, HUD_LABEL_PLAYING_STATE_POSITION,
        HUD_LABEL_SCORE_POSITION, HUD_LABEL_SCORING_CHAR_SCALE, HUD_MAXCOMBO_POSITION,
        HUD_SCORE_POSITION, HUD_SCORING_CHAR_SCALE, NUM_TICKS_FOR_PLAYING_STATE_GO_BLINKING,
    },
    fonts::Fonts,
    resources::Resources,
};

use super::scoring::Scoring;

/*******************************************************************************
**** HUD LABELS CACHE
*******************************************************************************/
struct HudLabels {
    game_info_playing_state_ready: Text,
    game_info_playing_state_go: Text,
    game_info_playing_state_pause: Text,
    game_info_playing_state_gameover: Text,
    game_info_instructions_ready: Text,
    game_info_instructions_go: Text,
    game_info_instructions_pause: Text,
    game_info_instructions_gameover: Text,
    scoring_score: Text,
    scoring_maxcombo: Text,
    scoring_highscore: Text,
}

impl HudLabels {
    pub fn new(resources: &Resources) -> Self {
        let fonts: &Fonts = resources.get_fonts();
        let font_extra_bold = fonts.get_extra_bold();
        let font_semi_bold = fonts.get_semi_bold();

        let navigation_instructions = resources.get_navigation_instructions();

        HudLabels {
            // PLAYING STATES
            game_info_playing_state_ready: Self::create_playing_state_label(
                font_extra_bold,
                "Ready...",
                COLOR_YELLOW,
            ),
            game_info_playing_state_go: Self::create_playing_state_label(
                font_extra_bold,
                "Go!!!",
                Color::BLACK,
            ),
            game_info_playing_state_pause: Self::create_playing_state_label(
                font_extra_bold,
                "Paused",
                COLOR_LIGHT_GRAY,
            ),
            game_info_playing_state_gameover: Self::create_playing_state_label(
                font_extra_bold,
                "Game Over",
                COLOR_RED,
            ),
            // INSTRUCTIONS
            game_info_instructions_ready: navigation_instructions.get_playing_ready().to_owned(),
            game_info_instructions_go: navigation_instructions.get_playing_go().to_owned(),
            game_info_instructions_pause: navigation_instructions.get_playing_pause().to_owned(),
            game_info_instructions_gameover: navigation_instructions
                .get_playing_gameover()
                .to_owned(),
            // SCORING
            scoring_score: Self::create_scoring_label(font_semi_bold, "SCORE", COLOR_GREEN),
            scoring_maxcombo: Self::create_scoring_label(font_semi_bold, "MAX COMBO", COLOR_BLUE),
            scoring_highscore: Self::create_scoring_label(font_semi_bold, "HIGHSCORE", COLOR_RED),
        }
    }

    fn create_playing_state_label(font: Font, playing_state: &str, text_color: Color) -> Text {
        Text::new(TextFragment {
            text: playing_state.to_string(),
            color: Some(text_color),
            font: Some(font),
            scale: Some(PxScale::from(HUD_LABEL_PLAYING_STATE_CHAR_SCALE)),
        })
    }

    fn create_scoring_label(font: Font, scoring_type: &str, text_color: Color) -> Text {
        Text::new(TextFragment {
            text: scoring_type.to_string(),
            color: Some(text_color),
            font: Some(font),
            scale: Some(PxScale::from(HUD_LABEL_SCORING_CHAR_SCALE)),
        })
    }
}

/*******************************************************************************
**** HUD
*******************************************************************************/
pub struct Hud {
    labels: HudLabels,
    pub game_info: Option<GameInfo>,

    num_frames: usize,
    num_blinks: u8,

    scoring_values_font: Font,
    scoring_values: HudScoringValues,
}

impl Hud {
    pub fn new(resources: &Resources) -> Self {
        let font_semi_bold = resources.get_fonts().get_semi_bold();
        Hud {
            labels: HudLabels::new(resources),
            game_info: None,

            num_frames: 0,
            num_blinks: 0,

            scoring_values_font: font_semi_bold,
            scoring_values: HudScoringValues::new(font_semi_bold, 0), // FIXME refactor
        }
    }

    pub fn new_game(&mut self, highscore: usize) {
        self.num_frames = 0;
        self.num_blinks = 0;

        self.scoring_values = HudScoringValues::new(self.scoring_values_font, highscore);
    }

    pub fn set_game_info(&mut self, r#type: GameInfoType) {
        self.game_info = match r#type {
            GameInfoType::Ready => Some(GameInfo {
                playing_state: self.labels.game_info_playing_state_ready.clone(),
                instructions: self.labels.game_info_instructions_ready.clone(),
            }),
            GameInfoType::Go => Some(GameInfo {
                playing_state: self.labels.game_info_playing_state_go.clone(),
                instructions: self.labels.game_info_instructions_go.clone(),
            }),
            GameInfoType::Pause => Some(GameInfo {
                playing_state: self.labels.game_info_playing_state_pause.clone(),
                instructions: self.labels.game_info_instructions_pause.clone(),
            }),
            GameInfoType::GameOver => Some(GameInfo {
                playing_state: self.labels.game_info_playing_state_gameover.clone(),
                instructions: self.labels.game_info_instructions_gameover.clone(),
            }),
            GameInfoType::None => None,
        }
    }

    pub fn update_game_info(&mut self) {
        self.num_frames += 1;
        if self.num_blinks < 3 {
            if self.num_frames % NUM_TICKS_FOR_PLAYING_STATE_GO_BLINKING == 0 {
                let game_info = self.game_info.as_mut().unwrap();
                let mut playing_state_fragment = &mut game_info.playing_state.fragments_mut()[0];
                if let Some(mut playing_state_color) = playing_state_fragment.color {
                    if playing_state_color == Color::BLACK {
                        playing_state_color = COLOR_GREEN;
                    } else {
                        playing_state_color = Color::BLACK;
                        self.num_blinks += 1;
                    }
                    playing_state_fragment.color = Some(playing_state_color);
                }
            }
        } else {
            let game_info = self.game_info.as_mut().unwrap();
            let mut instructions_fragment = &mut game_info.instructions.fragments_mut()[0];
            if let Some(mut instructions_color) = instructions_fragment.color {
                instructions_color.a -= 0.001;
                if instructions_color.a > 0.0 {
                    instructions_fragment.color = Some(instructions_color);
                } else {
                    self.game_info = None;
                }
            }
        }
    }

    pub fn update_scoring(&mut self, scoring: &Scoring) {
        self.scoring_values.score =
            HudScoringValues::set_value(self.scoring_values_font, scoring.score);
        if scoring.is_new_maxcombo {
            self.scoring_values.maxcombo =
                HudScoringValues::set_value(self.scoring_values_font, scoring.maxcombo);
        }
        /*if scoring.is_new_highscore {
            // TODO only strikethrough?
            self.scoring_values.highscore = HudScoringValues::set_value(self.scoring_values_font, value);
        }*/
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        // GAME INFO
        if let Some(game_info) = &self.game_info {
            graphics::queue_text(
                ctx,
                &game_info.playing_state,
                Vec2::new(
                    HUD_LABEL_PLAYING_STATE_POSITION.0,
                    HUD_LABEL_PLAYING_STATE_POSITION.1,
                ),
                None,
            );
            graphics::queue_text(
                ctx,
                &game_info.instructions,
                Vec2::new(
                    HUD_LABEL_INSTRUCTIONS_POSITION.0,
                    HUD_LABEL_INSTRUCTIONS_POSITION.1,
                ),
                None,
            );
        }
        // SCORE
        graphics::queue_text(
            ctx,
            &self.labels.scoring_score,
            Vec2::new(HUD_LABEL_SCORE_POSITION.0, HUD_LABEL_SCORE_POSITION.1),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.scoring_values.score,
            Vec2::new(HUD_SCORE_POSITION.0, HUD_SCORE_POSITION.1),
            None,
        );
        // MAX COMBO
        graphics::queue_text(
            ctx,
            &self.labels.scoring_maxcombo,
            Vec2::new(HUD_LABEL_MAXCOMBO_POSITION.0, HUD_LABEL_MAXCOMBO_POSITION.1),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.scoring_values.maxcombo,
            Vec2::new(HUD_MAXCOMBO_POSITION.0, HUD_MAXCOMBO_POSITION.1),
            None,
        );
        // HIGHSCORE
        graphics::queue_text(
            ctx,
            &self.labels.scoring_highscore,
            Vec2::new(
                HUD_LABEL_HIGHSCORE_POSITION.0,
                HUD_LABEL_HIGHSCORE_POSITION.1,
            ),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.scoring_values.highscore,
            Vec2::new(HUD_HIGHSCORE_POSITION.0, HUD_HIGHSCORE_POSITION.1),
            None,
        );

        graphics::draw_queued_text(
            ctx,
            DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )
        .unwrap();
    }
}

pub enum GameInfoType {
    None,
    Ready,
    Go,
    Pause,
    GameOver,
}

pub struct GameInfo {
    playing_state: Text,
    instructions: Text,
}

struct HudScoringValues {
    score: Text,
    maxcombo: Text,
    highscore: Text,
}

impl HudScoringValues {
    // FIXME refactor
    pub fn new(font: Font, highscore: usize) -> Self {
        HudScoringValues {
            score: Self::set_value(font, 0),
            maxcombo: Self::set_value(font, 0),
            highscore: Self::set_value(font, highscore),
        }
    }

    fn set_value(font: Font, value: usize) -> Text {
        // FIXME refactor: get rid of the font function argument
        Text::new(TextFragment {
            text: value.to_string(),
            color: Some(COLOR_LIGHT_GRAY),
            font: Some(font),
            scale: Some(PxScale::from(HUD_SCORING_CHAR_SCALE)),
        })
    }
}
