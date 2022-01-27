use ggez::{
    graphics::{self, Align, DrawParam, PxScale, Text, TextFragment},
    Context, GameResult,
};
use glam::Vec2;

use crate::{
    constants::{
        APP_NAME, COLOR_GREEN, COLOR_ORANGE, HOWTOPLAY_AND_ABOUT_AREA_WIDTH,
        TITLE_SCREEN_TITLE_AREA_WIDTH, TITLE_SCREEN_TITLE_CHAR_SCALE, TITLE_SCREEN_TITLE_POSITION,
    },
    input::Event,
    resources::Resources,
};

use super::{Stage, StageTrait};

pub struct TitleScreen {
    title: Text,
    main_menu_navigation_instructions: Text,
}

impl TitleScreen {
    pub fn new(resources: &Resources) -> Self {
        let mut title = Text::new(TextFragment {
            text: APP_NAME.to_string(),
            color: Some(COLOR_GREEN),
            font: Some(resources.get_fonts().bold),
            scale: Some(PxScale::from(TITLE_SCREEN_TITLE_CHAR_SCALE)),
        });
        title.set_bounds(
            Vec2::new(TITLE_SCREEN_TITLE_AREA_WIDTH, f32::INFINITY),
            Align::Center,
        );

        let mut main_menu_navigation_instructions = Text::new(TextFragment {
            text: "Navigate the menu using [Enter] / [Escape] and [Up] / [Down] keys".to_string(),
            color: Some(COLOR_ORANGE),
            font: Some(resources.get_fonts().light_italic),
            scale: Some(PxScale::from(16.0)),
        });
        main_menu_navigation_instructions.set_bounds(
            Vec2::new(HOWTOPLAY_AND_ABOUT_AREA_WIDTH, f32::INFINITY),
            Align::Center,
        );

        Self {
            title,
            main_menu_navigation_instructions,
        }
    }
}

impl StageTrait for TitleScreen {
    fn update(&mut self, _ctx: &Context, input_event: Event) -> GameResult<Option<Stage>> {
        match input_event {
            Event::Enter => Ok(Some(Stage::MainMenu)),
            Event::Escape => Ok(None),
            _ => Ok(Some(Stage::TitleScreen)),
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
            &self.main_menu_navigation_instructions,
            Vec2::new(50.0, 278.0),
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
