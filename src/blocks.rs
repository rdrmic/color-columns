#![allow(
    clippy::float_cmp,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::missing_const_for_fn
)]

use std::fmt::{Display, Formatter, Result};

use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::Rng;

use ggez::graphics::{self, mint::Point2, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::{Context, GameResult};

use crate::constants::{
    BLOCK_COLOR_BLUE, BLOCK_COLOR_GREEN, BLOCK_COLOR_MAGENTA, BLOCK_COLOR_ORANGE, BLOCK_COLOR_RED,
    BLOCK_COLOR_YELLOW, BLOCK_SIZE, GAME_ARENA_COLUMNS, GAME_ARENA_MARGIN_LEFT, GAME_ARENA_RECT,
};

use self::cargo::Cargo;

pub mod cargo;
pub mod matches;
pub mod pile;

pub fn position_to_idx(pos: f32, axis: char) -> usize {
    let idx: usize;
    if axis == 'x' {
        idx = ((pos - GAME_ARENA_RECT.left()) / BLOCK_SIZE) as usize;
    } else if axis == 'y' {
        idx = ((GAME_ARENA_RECT.bottom() - pos - BLOCK_SIZE) / BLOCK_SIZE) as usize;
    } else {
        panic!("Wrong axis attribute!")
    }
    idx
}

pub fn idx_to_position(idx: usize, axis: char) -> f32 {
    let position: f32;
    if axis == 'x' {
        position = BLOCK_SIZE.mul_add(idx as f32, GAME_ARENA_RECT.left());
    } else if axis == 'y' {
        position = (GAME_ARENA_RECT.bottom() - BLOCK_SIZE * idx as f32) - BLOCK_SIZE;
    } else {
        panic!("Wrong axis attribute!")
    }
    position
}

pub fn idx_pair_to_center_point_of_block(idxs: &[usize; 2]) -> Point2<f32> {
    Point2 {
        x: BLOCK_SIZE.mul_add(idxs[0] as f32, GAME_ARENA_RECT.left()) + BLOCK_SIZE / 2.0,
        y: (GAME_ARENA_RECT.bottom() - BLOCK_SIZE * idxs[1] as f32) - BLOCK_SIZE + BLOCK_SIZE / 2.0,
    }
}

/*******************************************************************************
**** BLOCK
*******************************************************************************/
#[derive(Debug, Copy, Clone)]
pub struct Block {
    rect: Rect,
    pub color: BlockColor,
}

impl Block {
    // TODO glam::Vec2::new(10.0, 10.0) ?
    pub fn new(point: Point2<f32>, size: f32, color: BlockColor) -> Self {
        Self {
            rect: Rect::new(point.x, point.y, size, size),
            color,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let block_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), self.rect, self.color.color)?;
        graphics::draw(ctx, &block_mesh, DrawParam::default())?;

        Ok(())
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{} ({},{})",
            self.color.code,
            position_to_idx(self.rect.x, 'x'),
            position_to_idx(self.rect.y, 'y')
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BlockColor {
    pub code: char,
    pub color: Color,
}

/*******************************************************************************
**** BLOCKS FACTORY
*******************************************************************************/
pub struct Factory {
    rng: ThreadRng,
}

impl Factory {
    pub const COLORS: [BlockColor; 6] = [
        BLOCK_COLOR_RED,
        BLOCK_COLOR_GREEN,
        BLOCK_COLOR_BLUE,
        BLOCK_COLOR_ORANGE,
        BLOCK_COLOR_MAGENTA,
        BLOCK_COLOR_YELLOW,
    ];

    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn get_block_color_by_code(code: char) -> Color {
        for block_color in Self::COLORS {
            if block_color.code == code {
                return block_color.color;
            }
        }
        panic!("Block color with code '{}' does not exist!", code);
    }

    pub fn create_next_cargo(&mut self) -> Cargo {
        let mut color_block_randomly = |point| {
            #[allow(clippy::unwrap_used)]
            let random_color = Self::COLORS.choose(&mut self.rng).unwrap();
            Block::new(point, BLOCK_SIZE, *random_color)
        };

        let x = GAME_ARENA_RECT.left() - GAME_ARENA_MARGIN_LEFT;
        let y = GAME_ARENA_RECT.top();
        let blocks = [
            color_block_randomly(Point2 {
                x,
                y: BLOCK_SIZE.mul_add(0.0, y),
            }),
            color_block_randomly(Point2 {
                x,
                y: BLOCK_SIZE.mul_add(1.0, y),
            }),
            color_block_randomly(Point2 {
                x,
                y: BLOCK_SIZE.mul_add(2.0, y),
            }),
        ];
        Cargo::new(blocks)
    }

    pub fn put_cargo_in_arena(&mut self, mut cargo: Cargo) -> Cargo {
        let x = BLOCK_SIZE.mul_add(
            self.rng.gen_range(0..GAME_ARENA_COLUMNS) as f32,
            GAME_ARENA_RECT.left(),
        );
        let y = GAME_ARENA_RECT.top() - BLOCK_SIZE * 3.0;
        cargo.rect.x = x;
        cargo.rect.y = y;
        cargo.column_idx = position_to_idx(cargo.rect.x, 'x');
        for i in 0..3 {
            cargo.blocks[i].rect.x = x;
            cargo.blocks[i].rect.y = BLOCK_SIZE.mul_add(i as f32, y);
        }
        cargo
    }

    pub fn change_block_color_randomly(&mut self, block: &mut Block) {
        let mut new_random_block_color;
        #[allow(clippy::unwrap_used)]
        loop {
            new_random_block_color = Self::COLORS.choose(&mut self.rng).unwrap();
            if new_random_block_color.code != block.color.code {
                break;
            }
        }
        block.color = *new_random_block_color;
    }
}
