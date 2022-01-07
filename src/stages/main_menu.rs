use ggez::graphics::{Align, Color, DrawParam, Font, PxScale, Text, TextFragment};
use ggez::mint::Point2;
use ggez::{graphics, Context};
use glam::Vec2;

use super::StageTrait;
use crate::blocks::Block;
use crate::constants::{
    BLOCK_COLOR_BLUE, BLOCK_COLOR_GREEN, BLOCK_COLOR_YELLOW, COLOR_BLUE, COLOR_GREEN, COLOR_YELLOW,
    MAIN_MENU_ITEMS_Y_POSITIONS, MAIN_MENU_ITEM_AREA_CENTER, MAIN_MENU_ITEM_AREA_WIDTH,
    MAIN_MENU_ITEM_AREA_X, MAIN_MENU_ITEM_CHAR_SCALE,
    MAIN_MENU_SELECTED_ITEM_BLOCK_ALPHA_INCREMENT_ACCELERATION,
    MAIN_MENU_SELECTED_ITEM_BLOCK_FADE_IN_TRESHOLD, MAIN_MENU_SELECTED_ITEM_BLOCK_MARGIN_X,
    MAIN_MENU_SELECTED_ITEM_BLOCK_MARGIN_Y, MAIN_MENU_SELECTED_ITEM_BLOCK_SIZE,
};
use crate::input::Event;
use crate::resources::Resources;
use crate::stages::Stage;

/*******************************************************************************
**** ITEM LABELS CACHE
*******************************************************************************/
struct ItemLabels {
    play: Text,
    how_to_play: Text,
    credits: Text,
}

impl ItemLabels {
    pub fn new(font: Font) -> Self {
        Self {
            play: Self::create_item_label(font, "PLAY", COLOR_GREEN),
            how_to_play: Self::create_item_label(font, "HOW TO PLAY", COLOR_YELLOW),
            credits: Self::create_item_label(font, "CREDITS", COLOR_BLUE),
        }
    }

    fn create_item_label(font: Font, item: &str, text_color: Color) -> Text {
        let mut item = Text::new(TextFragment {
            text: item.to_string(),
            color: Some(text_color),
            font: Some(font),
            scale: Some(PxScale::from(MAIN_MENU_ITEM_CHAR_SCALE)),
        });
        item.set_bounds(
            Vec2::new(MAIN_MENU_ITEM_AREA_WIDTH, f32::INFINITY),
            Align::Center,
        );
        item
    }
}

/*******************************************************************************
**** SELECTED ITEM INDICATOR
*******************************************************************************/
struct SelectedItemBlocksPositions {
    play: [Point2<f32>; 2],
    how_to_play: [Point2<f32>; 2],
    credits: [Point2<f32>; 2],
}

impl SelectedItemBlocksPositions {
    fn new(item_widths: [f32; 3]) -> Self {
        let positions = Self::create_blocks_positions(item_widths);
        Self {
            play: positions[0],
            how_to_play: positions[1],
            credits: positions[2],
        }
    }

    fn create_blocks_positions(item_widths: [f32; 3]) -> Vec<[Point2<f32>; 2]> {
        let mut positions = Vec::with_capacity(3);
        for i in 0..item_widths.len() {
            let item_width_half = item_widths[i] / 2.0;
            let left_block_x = MAIN_MENU_ITEM_AREA_CENTER
                - item_width_half
                - MAIN_MENU_SELECTED_ITEM_BLOCK_SIZE
                - MAIN_MENU_SELECTED_ITEM_BLOCK_MARGIN_X;
            let right_block_x = MAIN_MENU_ITEM_AREA_CENTER
                + item_width_half
                + MAIN_MENU_SELECTED_ITEM_BLOCK_MARGIN_X;

            let y = MAIN_MENU_ITEMS_Y_POSITIONS[i] + MAIN_MENU_SELECTED_ITEM_BLOCK_MARGIN_Y;

            let left_block_point = Point2 { x: left_block_x, y };
            let right_block_point = Point2 {
                x: right_block_x,
                y,
            };
            positions.push([left_block_point, right_block_point]);
        }
        positions
    }
}

struct SelectedItemIndicator {
    blocks_positions: SelectedItemBlocksPositions,
}

impl SelectedItemIndicator {
    fn new(item_widths: [f32; 3]) -> Self {
        Self {
            blocks_positions: SelectedItemBlocksPositions::new(item_widths),
        }
    }

    fn create_blocks(&self, selected_item: Stage) -> [Block; 2] {
        let blocks_positions;
        let color;
        match selected_item {
            Stage::HowToPlay => {
                blocks_positions = self.blocks_positions.how_to_play;
                color = BLOCK_COLOR_YELLOW;
            }
            Stage::Credits => {
                blocks_positions = self.blocks_positions.credits;
                color = BLOCK_COLOR_BLUE;
            }
            _ => {
                // Stage::Playing
                blocks_positions = self.blocks_positions.play;
                color = BLOCK_COLOR_GREEN;
            }
        };

        let left_block = Block::new(
            blocks_positions[0],
            MAIN_MENU_SELECTED_ITEM_BLOCK_SIZE,
            color,
        );
        let right_block = Block::new(
            blocks_positions[1],
            MAIN_MENU_SELECTED_ITEM_BLOCK_SIZE,
            color,
        );
        [left_block, right_block]
    }
}

/*******************************************************************************
**** MAIN MENU
*******************************************************************************/
pub struct MainMenu {
    item_labels: ItemLabels,
    selected_item_indicator: SelectedItemIndicator,
    selected_item_idx: usize,
    selected_item_blocks: [Block; 2],
    selected_item_blocks_alpha: f32,
    selected_item_blocks_alpha_increment: f32,
}

impl MainMenu {
    pub const ITEMS: [Stage; 3] = [Stage::Playing, Stage::HowToPlay, Stage::Credits];

    pub fn new(resources: &Resources, ctx: &mut Context) -> Self {
        let font = resources.get_fonts().get_extra_bold();
        let item_labels = ItemLabels::new(font);

        let item_widths = [
            item_labels.play.dimensions(ctx).w,
            item_labels.how_to_play.dimensions(ctx).w,
            item_labels.credits.dimensions(ctx).w,
        ];
        let selected_item_indicator = SelectedItemIndicator::new(item_widths);

        let initially_selected_item_idx = 0;
        let initially_selected_item = Self::ITEMS[initially_selected_item_idx];
        let initially_selected_item_blocks =
            selected_item_indicator.create_blocks(initially_selected_item);

        Self {
            item_labels,
            selected_item_indicator,
            selected_item_idx: initially_selected_item_idx,
            selected_item_blocks: initially_selected_item_blocks,
            selected_item_blocks_alpha: 0.0,
            selected_item_blocks_alpha_increment: 0.0,
        }
    }

    pub fn select_item(&mut self, item_idx: usize) {
        let selected_item = Self::ITEMS[item_idx];
        self.selected_item_blocks = self.selected_item_indicator.create_blocks(selected_item);
    }
}

impl StageTrait for MainMenu {
    fn update(&mut self, input_event: Event) -> Option<Stage> {
        let previous_selected_item_idx = self.selected_item_idx;
        match input_event {
            Event::Down => {
                self.selected_item_idx += 1;
                if self.selected_item_idx > 2 {
                    self.selected_item_idx = 0;
                }
            }
            Event::Up => {
                if self.selected_item_idx == 0 {
                    self.selected_item_idx = 3;
                }
                self.selected_item_idx -= 1;
            }
            Event::Enter => {
                let selected_stage = Self::ITEMS[self.selected_item_idx];
                //println!("### Stage::MainMenu -> Stage::{:?}", selected_stage);
                return Some(selected_stage);
            }
            Event::Escape => {
                //println!("### Stage::MainMenu -> QUIT");
                return None;
            }
            _ => (),
        }
        if self.selected_item_idx != previous_selected_item_idx {
            self.select_item(self.selected_item_idx);

            self.selected_item_blocks_alpha = 0.0;
            self.selected_item_blocks_alpha_increment = 0.0;
        }

        // SELECTED ITEM BLOCKS FADING-IN ANIMATION
        self.selected_item_blocks[0].color.color.a = self.selected_item_blocks_alpha;
        self.selected_item_blocks[1].color.color.a = self.selected_item_blocks_alpha;
        if self.selected_item_blocks_alpha >= MAIN_MENU_SELECTED_ITEM_BLOCK_FADE_IN_TRESHOLD {
            self.selected_item_blocks_alpha = 0.0;
            self.selected_item_blocks_alpha_increment = 0.0;
        } else {
            self.selected_item_blocks_alpha_increment +=
                MAIN_MENU_SELECTED_ITEM_BLOCK_ALPHA_INCREMENT_ACCELERATION;
            self.selected_item_blocks_alpha += self.selected_item_blocks_alpha_increment;
        }

        Some(Stage::MainMenu)
    }

    fn draw(&mut self, ctx: &mut Context) {
        graphics::queue_text(
            ctx,
            &self.item_labels.play,
            Vec2::new(MAIN_MENU_ITEM_AREA_X, MAIN_MENU_ITEMS_Y_POSITIONS[0]),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.item_labels.how_to_play,
            Vec2::new(MAIN_MENU_ITEM_AREA_X, MAIN_MENU_ITEMS_Y_POSITIONS[1]),
            None,
        );
        graphics::queue_text(
            ctx,
            &self.item_labels.credits,
            Vec2::new(MAIN_MENU_ITEM_AREA_X, MAIN_MENU_ITEMS_Y_POSITIONS[2]),
            None,
        );

        graphics::draw_queued_text(
            ctx,
            DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )
        .unwrap();

        self.selected_item_blocks[0].draw(ctx);
        self.selected_item_blocks[1].draw(ctx);
    }
}
