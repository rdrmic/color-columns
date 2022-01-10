use ggez::{
    graphics::{self, Align, DrawParam, PxScale, Text, TextFragment},
    Context, GameResult,
};
use glam::Vec2;

use crate::{
    constants::{
        COLOR_LIGHT_GRAY, ABOUT_CHAR_SCALE, ABOUT_TEXT_POSITION, GO_BACK_LABEL_POSITION,
        HOWTOPLAY_AND_ABOUT_AREA_WIDTH,
    },
    input::Event,
    resources::Resources,
};

use super::{Stage, StageTrait};

pub struct About {
    go_back_instruction: Text,
    text: Text,
}

impl About {
    pub fn new(resources: &Resources) -> Self {
        let font = resources.get_fonts().get_semi_bold();

        let text_str = "\n\
            This game is a remake of\n\
            various old, \"classic\",\n\
            columns-like games.\n\
            And it's made in an attempt\n\
            to learn Rust programming\n\
            language.\n\n\n\
            rdrmic@gmail.com\n\
            ( any feedback is welcome :)
        ";
        let mut text = Text::new(TextFragment {
            text: text_str.to_string(),
            color: Some(COLOR_LIGHT_GRAY),
            font: Some(font),
            scale: Some(PxScale::from(ABOUT_CHAR_SCALE)),
        });
        text.set_bounds(
            Vec2::new(HOWTOPLAY_AND_ABOUT_AREA_WIDTH, f32::INFINITY),
            Align::Left,
        );

        Self {
            go_back_instruction: resources
                .get_navigation_instructions()
                .get_go_back()
                .clone(),
            text,
        }
    }
}

impl StageTrait for About {
    fn update(&mut self, input_event: Event) -> GameResult<Option<Stage>> {
        if let Event::Escape = input_event {
            //println!("### Stage::About -> Stage::MainMenu");
            return Ok(Some(Stage::MainMenu));
        }
        Ok(Some(Stage::About))
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::queue_text(
            ctx,
            &self.go_back_instruction,
            Vec2::new(GO_BACK_LABEL_POSITION.0, GO_BACK_LABEL_POSITION.1),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.text,
            Vec2::new(ABOUT_TEXT_POSITION.0, ABOUT_TEXT_POSITION.1),
            None,
        );

        graphics::draw_queued_text(
            ctx,
            DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )
        .unwrap();

        Ok(())
    }
}
