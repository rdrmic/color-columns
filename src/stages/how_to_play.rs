use ggez::{
    graphics::{self, Align, DrawParam, PxScale, Text, TextFragment},
    mint::Point2,
    Context, GameResult,
};
use glam::Vec2;

use crate::{
    blocks::Block,
    constants::{
        BLOCK_COLOR_BLUE, BLOCK_COLOR_GREEN, BLOCK_COLOR_MAGENTA, BLOCK_COLOR_ORANGE,
        BLOCK_COLOR_RED, BLOCK_COLOR_YELLOW, COLOR_GREEN, COLOR_ORANGE, COLOR_YELLOW,
        GO_BACK_LABEL_POSITION, HOWTOPLAY_AND_ABOUT_AREA_WIDTH, HOWTOPLAY_CONTROLS_CHAR_SCALE,
        HOWTOPLAY_CONTROLS_LEFTSIDE_TEXT_POSITION_X, HOWTOPLAY_CONTROLS_RIGHTSIDE_TEXT_POSITION_X,
        HOWTOPLAY_CONTROLS_TEXT_POSITION_Y, HOWTOPLAY_HEADER_BLOCK_SIZE,
        HOWTOPLAY_HEADER_CONTROLS_POSITION_Y, HOWTOPLAY_HEADER_POSITION_X,
        HOWTOPLAY_HEADER_SCORING_RULES_POSITION_Y, HOWTOPLAY_SCORING_RULES_CHAR_SCALE,
        HOWTOPLAY_SCORING_RULES_TEXT_POSITION,
    },
    input::Event,
    resources::Resources,
};

use super::{Stage, StageTrait};

pub struct HowToPlay {
    blocks_header_controls: [Block; 3],
    blocks_header_scoring_rules: [Block; 3],
    go_back_instruction: Text,
    controls_leftside: Text,
    controls_rightside: Text,
    scoring_rules: Text,
}

impl HowToPlay {
    pub fn new(resources: &Resources) -> Self {
        let font = resources.get_fonts().semi_bold;

        let controls_leftside_str = "\n\
            Right:\n\
            Left:\n\
            Shuffle up:\n\
            Shuffle down:\n\
            Drop:
        ";
        let mut controls_leftside = Text::new(TextFragment {
            text: controls_leftside_str.to_string(),
            color: Some(COLOR_ORANGE),
            font: Some(font),
            scale: Some(PxScale::from(HOWTOPLAY_CONTROLS_CHAR_SCALE)),
        });
        controls_leftside.set_bounds(
            Vec2::new(HOWTOPLAY_AND_ABOUT_AREA_WIDTH, f32::INFINITY),
            Align::Left,
        );

        let controls_rightside_str = "\n\
            RIGHT / D\n\
            LEFT / A\n\
            UP / W\n\
            DOWN / S\n\
            SPACE
        ";
        let mut controls_rightside = Text::new(TextFragment {
            text: controls_rightside_str.to_string(),
            color: Some(COLOR_YELLOW),
            font: Some(font),
            scale: Some(PxScale::from(HOWTOPLAY_CONTROLS_CHAR_SCALE)),
        });
        controls_rightside.set_bounds(
            Vec2::new(HOWTOPLAY_AND_ABOUT_AREA_WIDTH, f32::INFINITY),
            Align::Left,
        );

        let scoring_rules_str = "\n\
            Points are gained by matching\n\
            same-colored blocks in all 4\n\
            directions.\n\n\
            The more matched blocks in a\n\
            line - the more points gained.\n\n\
            Also, the points are\n\
            multiplicated by the number of\n\
            sequential cascading matchings.
        ";
        let mut scoring_rules = Text::new(TextFragment {
            text: scoring_rules_str.to_string(),
            color: Some(COLOR_GREEN),
            font: Some(font),
            scale: Some(PxScale::from(HOWTOPLAY_SCORING_RULES_CHAR_SCALE)),
        });
        scoring_rules.set_bounds(
            Vec2::new(HOWTOPLAY_AND_ABOUT_AREA_WIDTH, f32::INFINITY),
            Align::Left,
        );

        Self {
            blocks_header_controls: Self::create_header_for_controls(),
            blocks_header_scoring_rules: Self::create_header_for_scoring_rules(),
            go_back_instruction: resources
                .get_navigation_instructions()
                .get_go_back()
                .clone(),
            controls_leftside,
            controls_rightside,
            scoring_rules,
        }
    }

    fn create_header_for_controls() -> [Block; 3] {
        [
            Block::new(
                Point2 {
                    x: HOWTOPLAY_HEADER_POSITION_X,
                    y: HOWTOPLAY_HEADER_CONTROLS_POSITION_Y,
                },
                HOWTOPLAY_HEADER_BLOCK_SIZE,
                BLOCK_COLOR_BLUE,
            ),
            Block::new(
                Point2 {
                    x: HOWTOPLAY_HEADER_POSITION_X + HOWTOPLAY_HEADER_BLOCK_SIZE,
                    y: HOWTOPLAY_HEADER_CONTROLS_POSITION_Y,
                },
                HOWTOPLAY_HEADER_BLOCK_SIZE,
                BLOCK_COLOR_YELLOW,
            ),
            Block::new(
                Point2 {
                    x: HOWTOPLAY_HEADER_BLOCK_SIZE.mul_add(2.0, HOWTOPLAY_HEADER_POSITION_X),
                    y: HOWTOPLAY_HEADER_CONTROLS_POSITION_Y,
                },
                HOWTOPLAY_HEADER_BLOCK_SIZE,
                BLOCK_COLOR_GREEN,
            ),
        ]
    }

    fn create_header_for_scoring_rules() -> [Block; 3] {
        [
            Block::new(
                Point2 {
                    x: HOWTOPLAY_HEADER_POSITION_X,
                    y: HOWTOPLAY_HEADER_SCORING_RULES_POSITION_Y,
                },
                HOWTOPLAY_HEADER_BLOCK_SIZE,
                BLOCK_COLOR_RED,
            ),
            Block::new(
                Point2 {
                    x: HOWTOPLAY_HEADER_POSITION_X + HOWTOPLAY_HEADER_BLOCK_SIZE,
                    y: HOWTOPLAY_HEADER_SCORING_RULES_POSITION_Y,
                },
                HOWTOPLAY_HEADER_BLOCK_SIZE,
                BLOCK_COLOR_ORANGE,
            ),
            Block::new(
                Point2 {
                    x: HOWTOPLAY_HEADER_BLOCK_SIZE.mul_add(2.0, HOWTOPLAY_HEADER_POSITION_X),
                    y: HOWTOPLAY_HEADER_SCORING_RULES_POSITION_Y,
                },
                HOWTOPLAY_HEADER_BLOCK_SIZE,
                BLOCK_COLOR_MAGENTA,
            ),
        ]
    }
}

impl StageTrait for HowToPlay {
    fn update(&mut self, _ctx: &Context, input_event: Event) -> GameResult<Option<Stage>> {
        if let Event::Escape = input_event {
            //println!("### Stage::HowToPlay -> Stage::MainMenu");
            return Ok(Some(Stage::MainMenu));
        }
        Ok(Some(Stage::HowToPlay))
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for mut block in self.blocks_header_controls {
            block.draw(ctx)?;
        }
        for mut block in self.blocks_header_scoring_rules {
            block.draw(ctx)?;
        }

        graphics::queue_text(
            ctx,
            &self.go_back_instruction,
            Vec2::new(GO_BACK_LABEL_POSITION[0], GO_BACK_LABEL_POSITION[1]),
            None,
        );

        graphics::queue_text(
            ctx,
            &self.controls_leftside,
            Vec2::new(
                HOWTOPLAY_CONTROLS_LEFTSIDE_TEXT_POSITION_X,
                HOWTOPLAY_CONTROLS_TEXT_POSITION_Y,
            ),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.controls_rightside,
            Vec2::new(
                HOWTOPLAY_CONTROLS_RIGHTSIDE_TEXT_POSITION_X,
                HOWTOPLAY_CONTROLS_TEXT_POSITION_Y,
            ),
            None,
        );

        graphics::queue_text(
            ctx,
            &self.scoring_rules,
            Vec2::new(
                HOWTOPLAY_SCORING_RULES_TEXT_POSITION[0],
                HOWTOPLAY_SCORING_RULES_TEXT_POSITION[1],
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
