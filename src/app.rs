use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    mem,
    path::PathBuf,
};

use chrono::Local;

use ggez::{
    conf::WindowMode,
    event::{self, ErrorOrigin, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color},
    Context, ContextBuilder, GameError, GameResult,
};

use crate::{
    constants::{APP_NAME, FPS, WINDOW_HEIGHT, WINDOW_WIDTH},
    input::Event,
    resources::Resources,
    stages::{
        about::About, how_to_play::HowToPlay, main_menu::MainMenu, playing::Playing, Stage,
        StageTrait,
    },
};

pub fn run() {
    // GET PATH TO RESOURCES DIRECTORY
    let resources_dir_path = create_exe_relative_dir_path("resources");

    // CREATE GAME CONTEXT
    let ctx_builder_result = ContextBuilder::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_AUTHORS"))
        .window_mode(
            WindowMode::default()
                .dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
                .visible(false),
        )
        .add_resource_path(&resources_dir_path)
        .build();
    let (mut ctx, event_loop);
    match ctx_builder_result {
        Ok(ctx_builder) => {
            ctx = ctx_builder.0;
            event_loop = ctx_builder.1;
        }
        Err(error) => {
            log_error("ContextBuilder", &error);
            return;
        }
    }

    // SET WINDOW
    if let Err(error) = graphics::set_window_icon(&mut ctx, Some("/icon.png")) {
        log_error("set_window_icon", &error);
    }
    graphics::set_window_title(&ctx, APP_NAME);
    // set window position only when developing
    if cfg!(debug_assertions) {
        use ggez::winit;
        if let Err(error) =
            graphics::set_window_position(&ctx, winit::dpi::PhysicalPosition::new(1350.0, 7.0))
        {
            log_error("set_window_position", &error);
        }
    }
    graphics::window(&ctx).set_visible(true);

    // CREATE APP STATE
    let app = App::new(&mut ctx, Stage::MainMenu);

    // RUN MAIN LOOP
    event::run(ctx, event_loop, app);
}

fn create_exe_relative_dir_path(dir: &str) -> PathBuf {
    let root_path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        PathBuf::from("./")
    };
    root_path.join(dir)
}

pub fn log_error(origin: &str, error: &GameError) {
    let mut error_log = None;

    let error_logs_dir_path = create_exe_relative_dir_path("__errors");
    if !error_logs_dir_path.exists() && fs::create_dir(&error_logs_dir_path).is_err() {
        eprintln!("Error logs directory could not be created!");
    }
    if error_logs_dir_path.exists() {
        let date_and_time = Local::now().format("on %Y-%m-%d at %H_%M_%S").to_string();
        let error_log_file_name = format!("ERROR {}.log", date_and_time);
        let error_log_file_path = error_logs_dir_path.join(error_log_file_name);
        if let Ok(created_file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(error_log_file_path)
        {
            error_log = Some(created_file);
        } else {
            eprintln!("Error log file could not be created!");
        }
    }

    if let Some(mut error_log) = error_log {
        let error = format!("# {}\n{}\n\n", origin, error);
        let write_result = error_log.write_all(error.as_bytes());
        if write_result.is_err() {
            eprintln!(
                "Could not write error to a file! {:?}",
                write_result.unwrap_err()
            );
        }
    }
}

/*******************************************************************************
**** APP STATE
*******************************************************************************/
pub struct App {
    stages: Vec<Box<dyn StageTrait>>,
    current_stage: Stage,
    input_event: Event, // FIXME multiple events
}

impl App {
    pub fn new(ctx: &mut Context, initial_stage: Stage) -> Self {
        let resources = Resources::new(ctx);
        Self {
            stages: vec![
                Box::new(MainMenu::new(&resources, ctx)),
                Box::new(Playing::new(&resources)),
                Box::new(HowToPlay::new(&resources)),
                Box::new(About::new(&resources)),
            ],
            current_stage: initial_stage,
            input_event: Event::None,
        }
    }

    fn get_currrent_stage(&mut self) -> &mut Box<dyn StageTrait> {
        match self.current_stage {
            Stage::MainMenu => &mut self.stages[Stage::MainMenu as usize],
            Stage::Playing => &mut self.stages[Stage::Playing as usize],
            Stage::HowToPlay => &mut self.stages[Stage::HowToPlay as usize],
            Stage::About => &mut self.stages[Stage::About as usize],
        }
    }
}

impl EventHandler<GameError> for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while ggez::timer::check_update_time(ctx, FPS) {
            let user_input = mem::take(&mut self.input_event);
            if let Some(stage_from_update) = self.get_currrent_stage().update(user_input)? {
                self.current_stage = stage_from_update;
            } else {
                event::quit(ctx);
                self.quit_event(ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        self.get_currrent_stage().draw(ctx)?;
        graphics::present(ctx)?;

        ggez::timer::yield_now();

        Ok(())
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
        self.input_event = Event::map_input(keycode);
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if !gained {
            self.input_event = Event::LostFocus;
        }
    }

    fn quit_event(&mut self, ctx: &mut Context) -> bool {
        if let Stage::Playing = self.current_stage {
            let result = self.get_currrent_stage().update(Event::SaveScoreOnQuit);
            if let Err(error) = result {
                self.on_error(ctx, ErrorOrigin::Update, error);
            }
        }
        false
    }

    fn on_error(&mut self, _ctx: &mut Context, origin: ErrorOrigin, error: GameError) -> bool {
        let origin = format!("{:?}", origin);
        let origin = origin.as_str();
        log_error(origin, &error);
        true
    }
}
