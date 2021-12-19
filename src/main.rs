//#![warn(clippy::all, clippy::pedantic)]
#![windows_subsystem = "windows"]

use std::{env, mem, path::PathBuf};

//use winit::dpi::LogicalPosition;

use ggez::{
    conf::WindowMode,
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color},
    Context, ContextBuilder, GameError, GameResult,
};

use config::{FPS, WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};

use crate::{
    input::InputEvent,
    resources::Resources,
    stages::{
        credits::Credits, how_to_play::HowToPlay, main_menu::MainMenu, playing::Playing, Stage,
        StageTrait,
    },
};

mod blocks;
mod config;
mod fonts;
mod input;
mod navigation_instructions;
mod resources;
mod snapshot;
mod stages;

fn main() {
    // SET PATH TO RESOURCES DIRECTORY
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    };
    // CREATE GAME CONTEXT
    let (mut ctx, event_loop) =
        ContextBuilder::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_AUTHORS"))
            .window_mode(WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
            .add_resource_path(resource_dir)
            .build()
            .unwrap();
    // SET WINDOW POSITION
    //graphics::set_window_position(&ctx, LogicalPosition::new(1000.0, 20.0)).unwrap();     // FIXME
    // SET WINDOW TITLE
    let app_version = env!("CARGO_PKG_VERSION");
    let window_title = format!("{} {}", WINDOW_TITLE, app_version);
    graphics::set_window_title(&ctx, &window_title);
    // CREATE APP STATE
    let app_state = AppState::new(&mut ctx, Stage::MainMenu);
    // RUN MAIN LOOP
    event::run(ctx, event_loop, app_state);
}

/*******************************************************************************
**** APP STATE
*******************************************************************************/
struct AppState {
    stages: Vec<Box<dyn StageTrait>>,
    current_stage: Stage,
    input_event: InputEvent, // FIXME multiple events
}

impl AppState {
    fn new(ctx: &mut Context, initial_stage: Stage) -> Self {
        let resources = Resources::new(ctx);
        AppState {
            stages: vec![
                Box::new(MainMenu::new(&resources, ctx)),
                Box::new(Playing::new(&resources)),
                Box::new(HowToPlay::new(&resources)),
                Box::new(Credits::new(&resources)),
            ],
            current_stage: initial_stage,
            input_event: InputEvent::None,
        }
    }

    fn get_currrent_stage(&mut self) -> &mut Box<dyn StageTrait> {
        match self.current_stage {
            Stage::MainMenu => &mut self.stages[Stage::MainMenu as usize],
            Stage::Playing => &mut self.stages[Stage::Playing as usize],
            Stage::HowToPlay => &mut self.stages[Stage::HowToPlay as usize],
            Stage::Credits => &mut self.stages[Stage::Credits as usize],
        }
    }
}

impl EventHandler<GameError> for AppState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, FPS) {
            let user_input = mem::take(&mut self.input_event);
            if let Some(stage_from_update) = self.get_currrent_stage().update(user_input) {
                /*if let Stage::MainMenu = self.current_stage {
                    if let Stage::Playing = stage_from_update {
                        //let new_playing_stage = &mut self.stages[Stage::Playing as usize] as Playing;
                        //println!("{:?}", (&mut self.stages[Stage::Playing as usize] as Playing).matching);
                        //
                    }
                }*/
                self.current_stage = stage_from_update;
            } else {
                event::quit(ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        self.get_currrent_stage().draw(ctx);
        graphics::present(ctx)

        // TODO timer::yield_now() needed?
        //graphics::present(ctx)?;
        //timer::yield_now();
        //Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        /*println!(
            "***\nkeycode: {:#?}\nkeymods: {:#?}\nrepeat: {:#?}\n***",
            keycode,
            _keymods,
            _repeat
        );*/
        self.input_event = InputEvent::map_input(keycode);

        // FIXME remove
        /*if keycode == KeyCode::Escape {
            event::quit(_ctx);
        }*/
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if !gained {
            self.input_event = InputEvent::LostFocus;
        }
    }

    /*fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        println!("*** BYE...");
        if let Stage::Playing = self.current_stage {
            println!(" // ... from Playing");
            self.input_event = InputEvent::Quit;
            return true
        }
        false
    }*/

    // TODO
    /*fn on_error(&mut self, _ctx: &mut Context, _origin: event::ErrorOrigin, _e: GameError) -> bool {

    }*/
}
