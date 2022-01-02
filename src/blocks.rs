#![allow(
    clippy::float_cmp,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use std::fmt::{Display, Formatter, Result};

use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::Rng;

use ggez::graphics::{self, mint::Point2, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::Context;

use crate::config::{
    BLOCK_COLOR_BLUE, BLOCK_COLOR_CYAN, BLOCK_COLOR_GREEN, BLOCK_COLOR_MAGENTA, BLOCK_COLOR_RED,
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
        position = GAME_ARENA_RECT.left() + BLOCK_SIZE * idx as f32;
    } else if axis == 'y' {
        position = (GAME_ARENA_RECT.bottom() - BLOCK_SIZE * idx as f32) - BLOCK_SIZE;
    } else {
        panic!("Wrong axis attribute!")
    }
    position
}

// TODO glam::Vec2::new(10.0, 10.0)
pub fn idx_pair_to_center_point_of_block(idxs: &(usize, usize)) -> Point2<f32> {
    Point2 {
        x: (GAME_ARENA_RECT.left() + BLOCK_SIZE * idxs.0 as f32) + BLOCK_SIZE / 2.0,
        y: (GAME_ARENA_RECT.bottom() - BLOCK_SIZE * idxs.1 as f32) - BLOCK_SIZE + BLOCK_SIZE / 2.0,
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
    // TODO glam::Vec2::new(10.0, 10.0)
    pub fn new(point: Point2<f32>, size: f32, color: BlockColor) -> Self {
        Block {
            rect: Rect::new(point.x, point.y, size, size),
            color,
        }
    }

    // TODO ggez::graphics::MeshBatch
    pub fn draw(&mut self, ctx: &mut Context) {
        let block_mesh =
            Mesh::new_rectangle(ctx, DrawMode::fill(), self.rect, self.color.color).unwrap();
        graphics::draw(ctx, &block_mesh, DrawParam::default()).unwrap();
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
        BLOCK_COLOR_CYAN,
        BLOCK_COLOR_MAGENTA,
        BLOCK_COLOR_YELLOW,
    ];

    pub fn new() -> Self {
        Factory {
            rng: rand::thread_rng(),
        }
    }

    pub fn create_next_cargo(&mut self) -> Cargo {
        let mut color_block_randomly = |point| {
            let color = Self::COLORS.choose(&mut self.rng).unwrap();
            Block::new(point, BLOCK_SIZE, *color)
        };

        let x = GAME_ARENA_RECT.left() - GAME_ARENA_MARGIN_LEFT;
        let y = GAME_ARENA_RECT.top();
        let blocks = [
            color_block_randomly(Point2 {
                x,
                y: y + BLOCK_SIZE * 0.0,
            }),
            color_block_randomly(Point2 {
                x,
                y: y + BLOCK_SIZE * 1.0,
            }),
            color_block_randomly(Point2 {
                x,
                y: y + BLOCK_SIZE * 2.0,
            }),
        ];
        Cargo::new(blocks)
    }

    pub fn put_cargo_in_arena(&mut self, mut cargo: Cargo) -> Cargo {
        let x =
            GAME_ARENA_RECT.left() + BLOCK_SIZE * self.rng.gen_range(0..GAME_ARENA_COLUMNS) as f32;
        let y = GAME_ARENA_RECT.top() - BLOCK_SIZE * 3.0;
        cargo.rect.x = x;
        cargo.rect.y = y;
        cargo.column_idx = position_to_idx(cargo.rect.x, 'x');
        for i in 0..3 {
            cargo.blocks[i].rect.x = x;
            cargo.blocks[i].rect.y = y + i as f32 * BLOCK_SIZE;
        }
        cargo
    }

    pub fn change_block_color_randomly(&mut self, block: &mut Block) {
        let mut new_block_color;
        loop {
            new_block_color = Self::COLORS.choose(&mut self.rng).unwrap();
            if new_block_color.code != block.color.code {
                break;
            }
        }
        block.color = *new_block_color;
    }
}
