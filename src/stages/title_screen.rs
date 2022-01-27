use ggez::{
    graphics::{self, Align, DrawParam, Font, PxScale, Text, TextFragment},
    Context, GameResult,
};
use glam::Vec2;
use rand::prelude::{SliceRandom, ThreadRng};

use crate::{
    blocks,
    constants::{
        APP_NAME, COLOR_ORANGE, COLOR_RED, NO_BLOCK_CODE, TITLE_SCREEN_AREA_WIDTH,
        TITLE_SCREEN_NAVIGATION_INSTRUCTIONS_CHAR_SCALE,
        TITLE_SCREEN_NAVIGATION_INSTRUCTIONS_POSITION, TITLE_SCREEN_NUM_FRAMES_FOR_ANIMATION,
        TITLE_SCREEN_TITLE_CHAR_SCALE, TITLE_SCREEN_TITLE_POSITION,
    },
    input::Event,
    resources::Resources,
};

use super::{Stage, StageTrait};

pub struct TitleScreen {
    title: Text,
    navigation_instructions: Text,
    num_frames: usize,
    rng: ThreadRng,
    previous_frame_title_colors_codes: [char; APP_NAME.len()],
}

impl TitleScreen {
    pub fn new(resources: &Resources) -> Self {
        let fonts = resources.get_fonts();

        let mut title = Text::default();
        for char in APP_NAME.chars() {
            title.add(TextFragment {
                text: char.to_string(),
                color: None,
                font: Some(fonts.bold),
                scale: Some(PxScale::from(TITLE_SCREEN_TITLE_CHAR_SCALE)),
            });
        }
        title.set_bounds(
            Vec2::new(TITLE_SCREEN_AREA_WIDTH, f32::INFINITY),
            Align::Center,
        );

        let mut navigation_instructions =
            Self::create_navigation_instructions_text(fonts.light_italic);
        navigation_instructions.set_bounds(
            Vec2::new(TITLE_SCREEN_AREA_WIDTH, f32::INFINITY),
            Align::Center,
        );

        Self {
            title,
            navigation_instructions,
            num_frames: 0,
            rng: rand::thread_rng(),
            previous_frame_title_colors_codes: [NO_BLOCK_CODE; APP_NAME.len()],
        }
    }

    // "Navigate the menu using [Enter] / [Escape] and [Up] / [Down] keys"
    fn create_navigation_instructions_text(font: Font) -> Text {
        let mut text = Text::new(
            Self::create_navigation_instructions_textfragment_decription_text(
                font,
                "Navigate the menu using ",
            ),
        );
        text.add(Self::create_navigation_instructions_textfragment_key(
            font, "Enter",
        ));
        text.add(Self::create_navigation_instructions_textfragment_decription_text(font, " / "));
        text.add(Self::create_navigation_instructions_textfragment_key(
            font, "Escape",
        ));

        text.add(Self::create_navigation_instructions_textfragment_decription_text(font, " and "));
        text.add(Self::create_navigation_instructions_textfragment_key(
            font, "Up",
        ));
        text.add(Self::create_navigation_instructions_textfragment_decription_text(font, " / "));
        text.add(Self::create_navigation_instructions_textfragment_key(
            font, "Down",
        ));
        text.add(Self::create_navigation_instructions_textfragment_decription_text(font, " keys"));
        text
    }

    // FIXME avoid font as parameter
    fn create_navigation_instructions_textfragment_decription_text(
        font: Font,
        key: &str,
    ) -> TextFragment {
        TextFragment {
            text: key.to_string(),
            color: Some(COLOR_ORANGE),
            font: Some(font),
            scale: Some(PxScale::from(
                TITLE_SCREEN_NAVIGATION_INSTRUCTIONS_CHAR_SCALE,
            )),
        }
    }

    // FIXME avoid font as parameter
    fn create_navigation_instructions_textfragment_key(font: Font, key: &str) -> TextFragment {
        TextFragment {
            text: format!("[{}]", key),
            color: Some(COLOR_RED),
            font: Some(font),
            scale: Some(PxScale::from(
                TITLE_SCREEN_NAVIGATION_INSTRUCTIONS_CHAR_SCALE,
            )),
        }
    }

    fn shuffle_title_colors(&mut self) {
        let mut prev_color_code = NO_BLOCK_CODE;
        let mut new_random_color;
        for (char_frag_idx, char_frag) in self.title.fragments_mut().iter_mut().enumerate() {
            let mut new_color_code;
            #[allow(clippy::unwrap_used)]
            loop {
                new_random_color = blocks::Factory::COLORS.choose(&mut self.rng).unwrap();
                new_color_code = new_random_color.code;
                if new_color_code != prev_color_code
                    && new_color_code != self.previous_frame_title_colors_codes[char_frag_idx]
                {
                    break;
                }
            }
            char_frag.color = Some(new_random_color.color);
            prev_color_code = new_color_code;
            self.previous_frame_title_colors_codes[char_frag_idx] = new_color_code;
        }
    }
}

impl StageTrait for TitleScreen {
    fn update(&mut self, _ctx: &Context, input_event: Event) -> GameResult<Option<Stage>> {
        match input_event {
            Event::Enter => Ok(Some(Stage::MainMenu)),
            Event::Escape => Ok(None),
            _ => {
                self.num_frames += 1;
                if self.num_frames % TITLE_SCREEN_NUM_FRAMES_FOR_ANIMATION == 0 {
                    self.shuffle_title_colors();
                }
                Ok(Some(Stage::TitleScreen))
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::queue_text(
            ctx,
            &self.title,
            Vec2::new(
                TITLE_SCREEN_TITLE_POSITION[0],
                TITLE_SCREEN_TITLE_POSITION[1],
            ),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.navigation_instructions,
            Vec2::new(
                TITLE_SCREEN_NAVIGATION_INSTRUCTIONS_POSITION[0],
                TITLE_SCREEN_NAVIGATION_INSTRUCTIONS_POSITION[1],
            ),
            None,
        );

        graphics::draw_queued_text(
            ctx,
            DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )?;

        Ok(())
    }
}
