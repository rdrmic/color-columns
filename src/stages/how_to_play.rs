use ggez::{
    graphics::{self, DrawParam, PxScale, Text, TextFragment},
    Context,
};
use glam::Vec2;

use crate::{
    config::{
        COLOR_GREEN, COLOR_RED, GO_BACK_LABEL_POSITION, HOWTOPLAY_AND_CREDITS_CHAR_SCALE,
        HOWTOPLAY_CONTROLS_TEXT_POSITION, HOWTOPLAY_SCORING_RULES_TEXT_POSITION,
    },
    input::InputEvent,
    resources::Resources,
};

use super::{Stage, StageTrait};

pub struct HowToPlay {
    go_back_instruction: Text,
    controls: Text,
    scoring_rules: Text,
}

impl HowToPlay {
    pub fn new(resources: &Resources) -> Self {
        let font = resources.get_fonts().get_semi_bold();

        // FIXME
        let controls_text = "-- CONTROLS --
Right                          RIGHT / D
Left                             LEFT / A
Shuffle up                  UP / W
Shuffle down             DOWN / S
Drop                            SPACE
        ";
        let scoring_rules_text = "-- SCORING RULES --
You gain points by matching
same-colored blocks in all 4
directions.
The more matched blocks in a
line - the more points.
Also, the points are
multiplicated by number of
sequential matchings.
        ";

        HowToPlay {
            go_back_instruction: resources
                .get_navigation_instructions()
                .get_go_back()
                .to_owned(),
            controls: Text::new(TextFragment {
                text: controls_text.to_string(),
                color: Some(COLOR_RED),
                font: Some(font),
                scale: Some(PxScale::from(HOWTOPLAY_AND_CREDITS_CHAR_SCALE)),
            }),
            scoring_rules: Text::new(TextFragment {
                text: scoring_rules_text.to_string(),
                color: Some(COLOR_GREEN),
                font: Some(font),
                scale: Some(PxScale::from(HOWTOPLAY_AND_CREDITS_CHAR_SCALE)),
            }),
        }
    }
}

impl StageTrait for HowToPlay {
    fn update(&mut self, input_event: InputEvent) -> Option<Stage> {
        if let InputEvent::Escape = input_event {
            //println!("### Stage::HowToPlay -> Stage::MainMenu");
            return Some(Stage::MainMenu);
        }
        Some(Stage::HowToPlay)
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
            &self.controls,
            Vec2::new(
                HOWTOPLAY_CONTROLS_TEXT_POSITION.0,
                HOWTOPLAY_CONTROLS_TEXT_POSITION.1,
            ),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.scoring_rules,
            Vec2::new(
                HOWTOPLAY_SCORING_RULES_TEXT_POSITION.0,
                HOWTOPLAY_SCORING_RULES_TEXT_POSITION.1,
            ),
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
