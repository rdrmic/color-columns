use ggez::{
    graphics::{self, DrawParam, PxScale, Text, TextFragment},
    Context,
};
use glam::Vec2;

use crate::{
    config::{
        COLOR_GREEN, CREDITS_TEXT_POSITION, GO_BACK_LABEL_POSITION,
        HOWTOPLAY_AND_CREDITS_CHAR_SCALE,
    },
    input::InputEvent,
    resources::Resources,
};

use super::{Stage, StageTrait};

pub struct Credits {
    go_back_instruction: Text,
    text: Text,
}

impl Credits {
    pub fn new(resources: &Resources) -> Self {
        let font = resources.get_fonts().get_semi_bold();

        // FIXME
        let controls_text = "
This game is a remake of
various old, 'classic',
columns-like games.
And it's made in an attempt to
learn Rust programming
language.


rdrmic@gmail.com
( any feedback is welcome :)
        ";

        Credits {
            go_back_instruction: resources
                .get_navigation_instructions()
                .get_go_back()
                .to_owned(),
            text: Text::new(TextFragment {
                text: controls_text.to_string(),
                color: Some(COLOR_GREEN),
                font: Some(font),
                scale: Some(PxScale::from(HOWTOPLAY_AND_CREDITS_CHAR_SCALE)),
            }),
        }
    }
}

impl StageTrait for Credits {
    fn update(&mut self, input_event: InputEvent) -> Option<Stage> {
        if let InputEvent::Escape = input_event {
            //println!("### Stage::Credits -> Stage::MainMenu");
            return Some(Stage::MainMenu);
        }
        Some(Stage::Credits)
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::queue_text(
            ctx,
            &self.go_back_instruction,
            Vec2::new(GO_BACK_LABEL_POSITION.0, GO_BACK_LABEL_POSITION.1),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.text,
            Vec2::new(CREDITS_TEXT_POSITION.0, CREDITS_TEXT_POSITION.1),
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
