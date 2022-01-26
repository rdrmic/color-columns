use ggez::{Context, GameResult};

use crate::input::Event;

pub mod about;
pub mod how_to_play;
pub mod main_menu;
pub mod playing;

#[derive(Debug, Clone, Copy)]
pub enum Stage {
    MainMenu,
    Playing,
    HowToPlay,
    About,
}

pub trait StageTrait {
    fn update(&mut self, ctx: &Context, user_input: Event) -> GameResult<Option<Stage>>;
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
}
