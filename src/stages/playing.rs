use std::{env, mem};

use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect, StrokeOptions};
use ggez::{Context, GameResult};

use self::hud::{GameInfo, GameInfoType};
use self::scoring::Scoring;

use super::{Stage, StageTrait};
use crate::blocks::cargo::Cargo;
use crate::blocks::matches::Matching;
use crate::blocks::pile::Pile;
use crate::blocks::{Block, Factory};
use crate::constants::{
    COLOR_GRAY, GAME_ARENA_RECT, NUM_DESCENDED_CARGOES_GAMEPLAY_ACCELERATION,
    NUM_TICKS_FOR_PAUSED_BLOCKS_SHUFFLE, NUM_TICKS_GAMEPLAY_ACCELERATION_LIMIT,
    STARTING_NUM_TICKS_FOR_CARGO_DESCENT,
};
use crate::input::Event;
use crate::resources::Resources;
use crate::snapshot;
use crate::stages::playing::hud::Hud;

mod hud;
mod scoring;

/*******************************************************************************
**** GAME ARENA
*******************************************************************************/
struct GameArena {
    border_rect: Rect,
    border_color: Color,
}

impl GameArena {
    fn new() -> Self {
        // compensating for border line width
        let mut border_rect = GAME_ARENA_RECT;
        border_rect.w += 1.0;
        border_rect.h += 1.0;

        Self {
            border_rect,
            border_color: COLOR_GRAY,
        }
    }

    // FIXME use GameResult throughout the app instead of unwrap()
    fn draw(&mut self, ctx: &mut Context) {
        let game_arena_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::Stroke(StrokeOptions::default()),
            self.border_rect,
            self.border_color,
        )
        .unwrap();
        graphics::draw(ctx, &game_arena_mesh, DrawParam::default()).unwrap();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Vertical,
    Horizontal,
    DiagonalSlash,
    DiagonalBackslash,
}

/*******************************************************************************
**** PLAYING
*******************************************************************************/
#[derive(Debug, Clone, Copy)]
enum PlayingState {
    Ready,
    DescendingCargo,
    HandlingMatches,
    Pause,
    GameOver,
    QuittingToMainMenu, // FIXME
}

pub struct Playing {
    hud: Hud,
    game_arena: GameArena,
    playing_state: Option<PlayingState>,
    num_frames: usize,
    blocks_factory: Factory,
    next_cargo: Option<Cargo>,
    descending_cargo: Option<Cargo>,
    num_ticks_for_cargo_descent: usize,
    is_cargo_at_bottom: bool,
    is_descending_over: bool,
    num_descended_cargoes: usize,
    pile: Pile,
    matching: Option<Matching>,
    scoring: Scoring,
    paused_blocks: Option<Vec<Block>>,
    playing_state_when_paused: Option<PlayingState>,
    game_info_when_paused: Option<GameInfo>,
    num_frames_pause: usize,
}

impl Playing {
    pub fn new(resources: &Resources) -> Self {
        // REPLAY FROM SNAPSHOT --> FIXME remove when the game is finished
        let mut playing_state = None;
        let mut blocks_factory = Factory::new();
        let mut next_cargo = None;
        let mut pile = Pile::new();

        let mut matching = None; // FIXME remove

        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() == 1 && args[0] == "replay" {
            playing_state = Some(PlayingState::HandlingMatches);
            next_cargo = Some(blocks_factory.create_next_cargo());
            pile = snapshot::create_pile_from_file();

            // FIXME remove
            //println!("/// pile.search_for_matches() ...");
            let matches = pile.search_for_matches();
            if !matches.is_empty() {
                //println!("\n>>> {:?}", matches);
                //self.pile.__print();
                //println!("-----------------------------------\n");

                //println!("/// Matching::new() ...");
                matching = Some(Matching::new(&matches, &mut pile));
            }
            //

            println!("*** REPLAYING FROM SNAPSHOT ***");
            println!("=> PlayingState::HandlingMatches");
        } /* else {
              println!("=> PlayingState::Ready");
          }*/

        Self {
            hud: Hud::new(resources),
            game_arena: GameArena::new(),
            playing_state,
            num_frames: 0,
            blocks_factory,
            next_cargo,
            descending_cargo: None,
            num_ticks_for_cargo_descent: STARTING_NUM_TICKS_FOR_CARGO_DESCENT,
            is_cargo_at_bottom: false,
            is_descending_over: false,
            num_descended_cargoes: 0,
            pile,
            matching,                 // FIXME matching: None,
            scoring: Scoring::new(0), // FIXME refactor
            paused_blocks: None,
            playing_state_when_paused: None,
            game_info_when_paused: None,
            num_frames_pause: 0,
        }
    }

    // TODO refactor
    fn new_game(&mut self) {
        //println!("Playing.new_game()");
        let highscore = Scoring::load_highscore();

        self.scoring = Scoring::new(highscore);

        self.hud.new_game(highscore);
        self.hud.set_game_info(GameInfoType::Ready);

        self.playing_state = Some(PlayingState::Ready);
        self.num_frames = 0;

        self.next_cargo = Some(self.blocks_factory.create_next_cargo());
        self.descending_cargo = None;
        self.num_ticks_for_cargo_descent = STARTING_NUM_TICKS_FOR_CARGO_DESCENT;
        self.is_cargo_at_bottom = false;
        self.is_descending_over = false;
        self.num_descended_cargoes = 0;

        self.pile = Pile::new();
        self.matching = None;

        self.paused_blocks = None;
        self.playing_state_when_paused = None;
        self.game_info_when_paused = None;
        self.num_frames_pause = 0;
    }

    fn begin_next_cargo_descent(&mut self) {
        self.descending_cargo = Some(
            self.blocks_factory
                .put_cargo_in_arena(mem::take(&mut self.next_cargo).unwrap()),
        );
        self.next_cargo = Some(self.blocks_factory.create_next_cargo());

        if self.num_ticks_for_cargo_descent > NUM_TICKS_GAMEPLAY_ACCELERATION_LIMIT
            && self.num_descended_cargoes > 0
            && self.num_descended_cargoes % NUM_DESCENDED_CARGOES_GAMEPLAY_ACCELERATION == 0
        {
            self.num_ticks_for_cargo_descent -= 1;
            if self.num_ticks_for_cargo_descent == NUM_TICKS_GAMEPLAY_ACCELERATION_LIMIT {
                self.hud.maxspeed_reached = true;
            }
            self.hud.set_game_info(GameInfoType::Speedup);
        }
    }

    fn pause(&mut self) {
        self.paused_blocks = Some(self.get_all_visible_blocks());
        self.shuffle_block_colors();

        self.playing_state_when_paused = self.playing_state;
        self.playing_state = Some(PlayingState::Pause);

        self.game_info_when_paused = self.hud.game_info.clone();
        self.hud.set_game_info(GameInfoType::Pause);

        self.num_frames_pause = 0;
    }

    fn get_all_visible_blocks(&self) -> Vec<Block> {
        let mut num_of_visible_blocks = 0;
        // NEXT CARGO
        let mut next_cargo_blocks = self.next_cargo.as_ref().unwrap().get_visible_blocks();
        num_of_visible_blocks += next_cargo_blocks.len();
        // DESCENDING CARGO
        let mut descending_cargo_visible_blocks = None;
        if let Some(descending_cargo) = &self.descending_cargo {
            let visible_blocks = descending_cargo.get_visible_blocks();
            num_of_visible_blocks += visible_blocks.len();

            descending_cargo_visible_blocks = Some(visible_blocks);
        }
        // PILE
        let mut pile_blocks = self.pile.get_blocks();
        num_of_visible_blocks += pile_blocks.len();
        // MATCHING
        let mut matches_blocks = None;
        if let Some(matching) = &self.matching {
            let blocks = matching.get_blocks();
            num_of_visible_blocks += blocks.len();

            matches_blocks = Some(blocks);
        }

        let mut visible_blocks: Vec<Block> = Vec::with_capacity(num_of_visible_blocks);
        visible_blocks.append(&mut next_cargo_blocks);
        if let Some(descending_cargo_visible_blocks) = &mut descending_cargo_visible_blocks {
            visible_blocks.append(descending_cargo_visible_blocks);
        }
        visible_blocks.append(&mut pile_blocks);
        if let Some(matches_blocks) = &mut matches_blocks {
            visible_blocks.append(matches_blocks);
        }
        visible_blocks
    }

    fn shuffle_block_colors(&mut self) {
        for block in self.paused_blocks.as_mut().unwrap() {
            self.blocks_factory.change_block_color_randomly(block);
        }
    }

    fn game_over(&mut self) {
        //println!("Playing.game_over()");
        self.scoring.save_highscore();
        self.hud.set_game_info(GameInfoType::GameOver);
        self.playing_state = Some(PlayingState::GameOver);
    }

    fn quit_to_main_menu(&mut self) {
        //println!("Playing.quit_to_main_menu()");
        // FIXME refactor
        self.playing_state = Some(PlayingState::QuittingToMainMenu);
        self.descending_cargo = None;
        self.pile = Pile::new();
        self.paused_blocks = None;
        self.matching = None;
    }

    /*** UPDATE'S PLAYING STATE VARIANTS [BEGIN] ***/
    fn update_state_ready(&mut self, input_event: &Event) {
        if self.hud.game_info.is_none() {
            self.hud.set_game_info(GameInfoType::Ready);
        }

        if self.next_cargo.is_none() {
            self.next_cargo = Some(self.blocks_factory.create_next_cargo());
        }

        match input_event {
            Event::Enter => {
                self.hud.set_game_info(GameInfoType::Go);
                self.playing_state = Some(PlayingState::DescendingCargo);
                //println!("PlayingState::DescendingCargo =>");
            }
            Event::Escape => {
                //println!("### Stage::Playing / Ready -> Stage::MainMenu");
                //return Some(Stage::MainMenu);
                self.quit_to_main_menu();
            }
            _ => (),
        };
    }

    fn update_state_descending_cargo(&mut self, input_event: &Event) {
        self.num_frames += 1;
        if let Event::Escape | Event::LostFocus = input_event {
            self.pause();
            //println!("PlayingState::Pause =>");
        }

        if self.is_cargo_at_bottom && self.is_descending_over {
            self.begin_next_cargo_descent();
            self.is_cargo_at_bottom = self
                .descending_cargo
                .as_mut()
                .unwrap()
                .descend_one_step(&self.pile);
            self.is_descending_over = false;
        }

        if self.num_frames % self.num_ticks_for_cargo_descent == 0
            && self.descending_cargo.is_none()
        {
            self.begin_next_cargo_descent();
            self.is_descending_over = false;
        }

        if let Some(descending_cargo) = self.descending_cargo.as_mut() {
            self.is_cargo_at_bottom = descending_cargo.is_at_bottom(&self.pile);
            if self.num_frames % self.num_ticks_for_cargo_descent == 0 && self.is_cargo_at_bottom {
                self.is_descending_over = true;
            }

            if !self.is_descending_over {
                match input_event {
                    Event::Right => descending_cargo.move_to_right(&self.pile),
                    Event::Left => descending_cargo.move_to_left(&self.pile),
                    Event::Up => descending_cargo.rearrange_up(),
                    Event::Down => descending_cargo.rearrange_down(),
                    Event::Drop => {
                        descending_cargo.drop(&self.pile);
                        self.is_descending_over = true;
                    }
                    _ => (),
                };
            }

            if self.num_frames % self.num_ticks_for_cargo_descent == 0
                && (!self.is_descending_over || self.is_cargo_at_bottom)
            {
                self.is_cargo_at_bottom = descending_cargo.descend_one_step(&self.pile);
            }

            if self.is_descending_over {
                self.num_descended_cargoes += 1;

                let num_of_remaining_places_in_column = self
                    .pile
                    .take_cargo(&mem::take(&mut self.descending_cargo).unwrap());

                if num_of_remaining_places_in_column < 0 {
                    self.game_over();
                    //println!("PlayingState::GameOver =>");
                } else {
                    let matches = self.pile.search_for_matches();
                    if matches.is_empty() {
                        if num_of_remaining_places_in_column == 0 {
                            self.game_over();
                            //println!("PlayingState::GameOver =>");
                        }
                    } else {
                        //println!("\n>>> {:?}", matches);
                        //self.pile.__print();
                        //println!("-----------------------------------\n");

                        self.matching = Some(Matching::new(&matches, &mut self.pile));

                        self.playing_state = Some(PlayingState::HandlingMatches);
                        //println!("PlayingState::HandlingMatches =>");
                        self.num_frames = 0;
                    }
                }
            }
        }
    }

    fn update_state_handling_matches(&mut self, input_event: &Event) {
        self.num_frames += 1;
        if let Event::Escape | Event::LostFocus = input_event {
            self.pause();
            //println!("PlayingState::Pause =>");
        }

        if let Some(matching) = self.matching.as_mut() {
            let is_animation_over = matching.blinking_animation(self.num_frames);
            if is_animation_over {
                let is_pile_full = self
                    .pile
                    .remove_matches(matching.get_unique_match_indexes());

                /*println!(
                    "AFTER REMOVAL (sequential matchings: {}):",
                    matching.get_num_of_sequential_matchings()
                );
                self.pile.__print();*/

                self.scoring
                    .update_from_matches(matching.get_scoring_data());
                self.hud.update_scoring(&self.scoring);

                let next_matches = self.pile.search_for_matches();
                if next_matches.is_empty() {
                    //println!("===================================\n");
                    if is_pile_full {
                        self.game_over();
                        //println!("PlayingState::GameOver =>");
                    } else {
                        self.playing_state = Some(PlayingState::DescendingCargo);
                        //println!("PlayingState::DescendingCargo =>");
                        self.num_frames = 0;
                    }
                    self.matching = None;
                } else {
                    //println!("\n>>> {:?}", next_matches);
                    matching.new_chained_match(&next_matches, &mut self.pile);

                    self.num_frames = 0; // TODO needed?
                }
            }
        // for REPLAY FROM SNAPSHOT --> FIXME remove when the game is finished
        } else {
            self.playing_state = Some(PlayingState::DescendingCargo);
            //println!("PlayingState::DescendingCargo =>");
            self.num_frames = 0;
        }
    }

    fn update_state_pause(&mut self, input_event: &Event) {
        //println!("// PlayingState::Pause");
        self.num_frames_pause += 1;
        if self.num_frames_pause % NUM_TICKS_FOR_PAUSED_BLOCKS_SHUFFLE == 0 {
            self.shuffle_block_colors();
        }
        match input_event {
            Event::Enter => {
                self.paused_blocks = None;

                self.playing_state = self.playing_state_when_paused;
                self.playing_state_when_paused = None;

                self.hud.game_info = self.game_info_when_paused.clone();
                self.game_info_when_paused = None;
            }
            Event::Escape => {
                //println!("### Stage::Playing / Pause -> Stage::MainMenu");
                //return Some(Stage::MainMenu);
                self.quit_to_main_menu();
            }
            _ => (),
        };
    }

    fn update_state_game_over(&mut self, input_event: &Event) {
        //println!("// PlayingState::GameOver");
        match input_event {
            Event::Enter => {
                //println!("PlayingState::None =>");
                self.playing_state = None;
                self.hud.set_game_info(GameInfoType::None);
            }
            Event::Escape => {
                //println!("### Stage::Playing / GameOver -> Stage::MainMenu");
                //return Some(Stage::MainMenu);
                self.quit_to_main_menu();
            }
            _ => (),
        };
    }
    /*** UPDATE'S PLAYING STATE VARIANTS [END] ***/
}

impl StageTrait for Playing {
    fn update(&mut self, input_event: Event) -> Option<Stage> {
        //InputEvent::__print(&input_event);
        /*if let None = self.playing_state {
            self.new_game();
        }*/
        if let Event::SaveScoreOnQuit = input_event {
            if let Some(
                PlayingState::DescendingCargo | PlayingState::HandlingMatches | PlayingState::Pause,
            ) = self.playing_state
            {
                self.scoring.save_highscore();
                return None;
            }
        }

        match self.playing_state {
            None => self.new_game(),
            Some(PlayingState::Ready) => self.update_state_ready(&input_event),
            Some(PlayingState::DescendingCargo) => self.update_state_descending_cargo(&input_event),
            Some(PlayingState::HandlingMatches) => self.update_state_handling_matches(&input_event),
            Some(PlayingState::Pause) => self.update_state_pause(&input_event),
            Some(PlayingState::GameOver) => self.update_state_game_over(&input_event),
            Some(PlayingState::QuittingToMainMenu) => {
                //println!("// PlayingState::QuittingToMainMenu");
                self.playing_state = None;
                return Some(Stage::MainMenu);
            }
        };
        if let Some(PlayingState::DescendingCargo | PlayingState::HandlingMatches) =
            self.playing_state
        {
            self.hud.update_game_info();
        }
        Some(Stage::Playing)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.hud.draw(ctx);
        self.game_arena.draw(ctx);

        if let Some(paused_blocks) = self.paused_blocks.as_deref_mut() {
            for block in paused_blocks {
                block.draw(ctx);
            }
        } else {
            if let Some(next_cargo) = &mut self.next_cargo {
                next_cargo.draw(ctx);
            }
            if let Some(descending_cargo) = &mut self.descending_cargo {
                descending_cargo.draw(ctx);
            }
            self.pile.draw(ctx);
            if let Some(matching) = &mut self.matching {
                matching.draw(ctx);
            }
        }

        Ok(())
    }
}
