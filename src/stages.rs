use ggez::{Context, GameResult};

use crate::input::Event;

pub mod credits;
pub mod how_to_play;
pub mod main_menu;
pub mod playing;

#[derive(Debug, Clone, Copy)]
pub enum Stage {
    MainMenu,
    Playing,
    HowToPlay,
    Credits,
}

pub trait StageTrait {
    fn update(&mut self, user_input: Event) -> Option<Stage>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
}
