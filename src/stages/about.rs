use ggez::{
    graphics::{self, Align, DrawParam, PxScale, Text, TextFragment},
    Context, GameResult,
};
use glam::Vec2;

use crate::{
    constants::{
        ABOUT_CHAR_SCALE, ABOUT_TEXT_POSITION, ABOUT_VERSION_AND_BUILDTIME_AREA_WIDTH,
        ABOUT_VERSION_AND_BUILDTIME_CHAR_SCALE, ABOUT_VERSION_AND_BUILDTIME_POSITION, BUILD_TIME,
        COLOR_GRAY, COLOR_LIGHT_GRAY, GO_BACK_LABEL_POSITION, HOWTOPLAY_AND_ABOUT_AREA_WIDTH,
    },
    input::Event,
    resources::Resources,
};

use super::{Stage, StageTrait};

pub struct About {
    go_back_instruction: Text,
    about: Text,
    version: Text,
}

impl About {
    pub fn new(resources: &Resources) -> Self {
        let font = resources.get_fonts().get_semi_bold();

        let about_str = "\n\
            This game is a remake of\n\
            various old, \"classic\",\n\
            columns-like games.\n\
            And it's made in an attempt\n\
            to learn Rust programming\n\
            language.\n\n\n\
            rdrmic@gmail.com\n\
            ( any feedback is welcome :)
        ";
        let mut about = Text::new(TextFragment {
            text: about_str.to_string(),
            color: Some(COLOR_LIGHT_GRAY),
            font: Some(font),
            scale: Some(PxScale::from(ABOUT_CHAR_SCALE)),
        });
        about.set_bounds(
            Vec2::new(HOWTOPLAY_AND_ABOUT_AREA_WIDTH, f32::INFINITY),
            Align::Left,
        );

        let mut version = env!("CARGO_PKG_VERSION").to_string();
        if cfg!(debug_assertions) {
            version = format!("{}_dev", version);
        }
        let version_and_buildtime_str = format!("Version: {}\n{}", version, BUILD_TIME);
        let mut version_and_buildtime = Text::new(TextFragment {
            text: version_and_buildtime_str,
            color: Some(COLOR_GRAY),
            font: Some(font),
            scale: Some(PxScale::from(ABOUT_VERSION_AND_BUILDTIME_CHAR_SCALE)),
        });
        version_and_buildtime.set_bounds(
            Vec2::new(ABOUT_VERSION_AND_BUILDTIME_AREA_WIDTH, f32::INFINITY),
            Align::Right,
        );

        Self {
            go_back_instruction: resources
                .get_navigation_instructions()
                .get_go_back()
                .clone(),
            about,
            version: version_and_buildtime,
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
            &self.about,
            Vec2::new(ABOUT_TEXT_POSITION.0, ABOUT_TEXT_POSITION.1),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.version,
            Vec2::new(
                ABOUT_VERSION_AND_BUILDTIME_POSITION.0,
                ABOUT_VERSION_AND_BUILDTIME_POSITION.1,
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
