use std::{env, mem};

use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect, StrokeOptions};
use ggez::Context;

use self::hud::GameInfoType;
use self::scoring::Scoring;

use super::{Stage, StageTrait};
use crate::blocks::cargo::Cargo;
use crate::blocks::matches::Matching;
use crate::blocks::pile::Pile;
use crate::blocks::{Block, BlocksFactory};
use crate::config::{
    COLOR_GRAY, GAME_ARENA_RECT, NUM_TICKS_FOR_CARGO_DESCENT, NUM_TICKS_FOR_PAUSED_BLOCKS_SHUFFLE,
};
use crate::input::InputEvent;
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

        GameArena {
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
    blocks_factory: BlocksFactory,
    next_cargo: Option<Cargo>,
    descending_cargo: Option<Cargo>,
    is_cargo_at_bottom: bool,
    is_descending_over: bool,
    pile: Pile,
    matching: Option<Matching>,
    scoring: Scoring,
    paused_blocks: Option<Vec<Block>>,
    playing_state_when_paused: Option<Option<PlayingState>>,
    num_frames_pause: usize,
}

impl Playing {
    pub fn new(resources: &Resources) -> Self {
        // REPLAY FROM SNAPSHOT --> FIXME remove when the game is finished
        let mut playing_state = None;
        let mut blocks_factory = BlocksFactory::new();
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
                matching = Some(Matching::new(matches, &mut pile));
            }
            //

            println!("*** REPLAYING FROM SNAPSHOT ***");
            println!("=> PlayingState::HandlingMatches");
        } else {
            //println!("=> PlayingState::Ready");
        }

        Playing {
            hud: Hud::new(resources),
            game_arena: GameArena::new(),
            playing_state,
            num_frames: 0,
            blocks_factory,
            next_cargo,
            descending_cargo: None,
            is_cargo_at_bottom: false,
            is_descending_over: false,
            pile,
            matching,                 // FIXME matching: None,
            scoring: Scoring::new(0), // FIXME refactor
            paused_blocks: None,
            playing_state_when_paused: None,
            num_frames_pause: 0,
        }
    }

    // TODO refactor
    fn new_game(&mut self) {
        let highscore = Scoring::load_highscore();

        self.scoring = Scoring::new(highscore);

        self.hud.new_game(highscore);
        self.hud.update_game_info(GameInfoType::Ready);

        self.playing_state = Some(PlayingState::Ready);
        self.num_frames = 0;

        self.next_cargo = Some(self.blocks_factory.create_next_cargo());
        self.descending_cargo = None;
        self.is_cargo_at_bottom = false;
        self.is_descending_over = false;

        self.pile = Pile::new();
        self.matching = None;

        self.paused_blocks = None;
        self.playing_state_when_paused = None;
        self.num_frames_pause = 0;
    }

    fn begin_next_cargo_descent(&mut self) {
        self.descending_cargo = Some(
            self.blocks_factory
                .put_cargo_in_arena(mem::take(&mut self.next_cargo).unwrap()),
        );
        self.next_cargo = Some(self.blocks_factory.create_next_cargo());
    }

    fn pause(&mut self) {
        self.paused_blocks = Some(self.get_all_visible_blocks());
        self.shuffle_block_colors();
        self.hud.update_game_info(GameInfoType::Pause);
        self.playing_state_when_paused = Some(self.playing_state);
        self.playing_state = Some(PlayingState::Pause);
        self.num_frames_pause = 0;
    }

    fn game_over(&mut self) {
        self.scoring.save_highscore();
        self.hud.update_game_info(GameInfoType::GameOver);
        self.playing_state = Some(PlayingState::GameOver);
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

    fn quit_to_main_menu(&mut self) {
        // FIXME refactor
        self.playing_state = Some(PlayingState::QuittingToMainMenu);
        self.descending_cargo = None;
        self.pile = Pile::new();
        //Some(Stage::MainMenu)
    }
}

impl StageTrait for Playing {
    fn update(&mut self, input_event: InputEvent) -> Option<Stage> {
        //InputEvent::__print(&input_event);
        /*if let None = self.playing_state {
            self.new_game();
        }*/
        match self.playing_state {
            None => self.new_game(),
            Some(PlayingState::Ready) => {
                if self.hud.game_info.is_none() {
                    self.hud.update_game_info(GameInfoType::Ready);
                }

                if self.next_cargo.is_none() {
                    self.next_cargo = Some(self.blocks_factory.create_next_cargo());
                }

                match input_event {
                    InputEvent::Enter => {
                        self.playing_state = Some(PlayingState::DescendingCargo);
                        //println!("PlayingState::DescendingCargo =>");

                        self.hud.update_game_info(GameInfoType::None);
                    }
                    InputEvent::Escape => {
                        //println!("### Stage::Playing / Ready -> Stage::MainMenu");
                        //return Some(Stage::MainMenu);
                        self.quit_to_main_menu();
                    }
                    _ => (),
                }
            }
            Some(PlayingState::DescendingCargo) => {
                self.num_frames += 1;
                if let InputEvent::Escape | InputEvent::LostFocus = input_event {
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

                if self.num_frames % NUM_TICKS_FOR_CARGO_DESCENT == 0
                    && self.descending_cargo.is_none()
                {
                    self.begin_next_cargo_descent();
                    self.is_descending_over = false;
                }

                if let Some(descending_cargo) = self.descending_cargo.as_mut() {
                    self.is_cargo_at_bottom = descending_cargo.is_at_bottom(&self.pile);
                    if self.num_frames % NUM_TICKS_FOR_CARGO_DESCENT == 0 && self.is_cargo_at_bottom
                    {
                        self.is_descending_over = true;
                    }

                    if !self.is_descending_over {
                        match input_event {
                            InputEvent::Right => descending_cargo.move_to_right(&self.pile),
                            InputEvent::Left => descending_cargo.move_to_left(&self.pile),
                            InputEvent::Up => descending_cargo.rearrange_up(),
                            InputEvent::Down => descending_cargo.rearrange_down(),
                            InputEvent::Drop => {
                                descending_cargo.drop(&self.pile);
                                self.is_descending_over = true;
                            }
                            _ => (),
                        }
                    }

                    if self.num_frames % NUM_TICKS_FOR_CARGO_DESCENT == 0
                        && (!self.is_descending_over || self.is_cargo_at_bottom)
                    {
                        self.is_cargo_at_bottom = descending_cargo.descend_one_step(&self.pile);
                    }

                    if self.is_descending_over {
                        let num_of_remaining_places_in_column = self
                            .pile
                            .take_cargo(mem::take(&mut self.descending_cargo).unwrap());
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

                                self.matching = Some(Matching::new(matches, &mut self.pile));

                                self.playing_state = Some(PlayingState::HandlingMatches);
                                //println!("PlayingState::HandlingMatches =>");
                                self.num_frames = 0;
                            }
                        }
                    }
                }
            }
            Some(PlayingState::HandlingMatches) => {
                self.num_frames += 1;
                if let InputEvent::Escape | InputEvent::LostFocus = input_event {
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
                        if !next_matches.is_empty() {
                            //println!("\n>>> {:?}", next_matches);
                            matching.new_chained_match(next_matches, &mut self.pile);

                            self.num_frames = 0; // TODO needed?
                        } else {
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
                        }
                    }
                // for REPLAY FROM SNAPSHOT --> FIXME remove when the game is finished
                } else {
                    self.playing_state = Some(PlayingState::DescendingCargo);
                    //println!("PlayingState::DescendingCargo =>");
                    self.num_frames = 0;
                }
            }
            Some(PlayingState::Pause) => {
                self.num_frames_pause += 1;
                if self.num_frames_pause % NUM_TICKS_FOR_PAUSED_BLOCKS_SHUFFLE == 0 {
                    self.shuffle_block_colors();
                }
                match input_event {
                    InputEvent::Enter => {
                        self.playing_state = self.playing_state_when_paused.unwrap();
                        self.playing_state_when_paused = None;
                        //println!("PlayingState::DescendingCargo =>");
                        self.paused_blocks = None;
                        self.hud.update_game_info(GameInfoType::None);
                    }
                    InputEvent::Escape => {
                        //println!("### Stage::Playing / Pause -> Stage::MainMenu");
                        //return Some(Stage::MainMenu);
                        self.quit_to_main_menu();
                    }
                    _ => (),
                }
            }
            Some(PlayingState::GameOver) => {
                match input_event {
                    InputEvent::Enter => {
                        // TODO reinitialize game
                        self.playing_state = Some(PlayingState::Ready);
                        //println!("PlayingState::Ready =>");

                        self.hud.update_game_info(GameInfoType::None);
                    }
                    InputEvent::Escape => {
                        //println!("### Stage::Playing / GameOver -> Stage::MainMenu");
                        //return Some(Stage::MainMenu);
                        self.quit_to_main_menu();
                    }
                    _ => (),
                }
            }
            Some(PlayingState::QuittingToMainMenu) => {
                self.playing_state = None;
                return Some(Stage::MainMenu);
            }
        }
        Some(Stage::Playing)
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.hud.draw(ctx);
        self.game_arena.draw(ctx);

        if let Some(paused_blocks) = self.paused_blocks.as_deref_mut() {
            for block in paused_blocks {
                block.draw(ctx);
            }
        } else {
            if let Some(next_cargo) = &mut self.next_cargo {
                next_cargo.draw(ctx)
            }
            if let Some(descending_cargo) = &mut self.descending_cargo {
                descending_cargo.draw(ctx)
            }
            self.pile.draw(ctx);
            if let Some(matching) = &mut self.matching {
                matching.draw(ctx);
            }
        }
    }
}