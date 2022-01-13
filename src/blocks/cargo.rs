use std::fmt::{Display, Formatter, Result};

use ggez::{graphics::Rect, Context, GameResult};

use crate::constants::{BLOCK_SIZE, GAME_ARENA_COLUMNS, GAME_ARENA_RECT};

use super::{pile::Pile, Block};

/*******************************************************************************
**** CARGO
*******************************************************************************/
#[derive(Debug)]
pub struct Cargo {
    pub blocks: [Block; 3],
    pub rect: Rect,
    pub column_idx: usize,
}

impl Cargo {
    pub fn new(blocks: [Block; 3]) -> Self {
        let first_block_rect = blocks[0].rect;
        Self {
            blocks,
            rect: Rect::new(
                first_block_rect.x,
                first_block_rect.y,
                first_block_rect.w,
                first_block_rect.h * 3.0,
            ),
            column_idx: usize::MAX,
        }
    }

    pub fn move_to_right(&mut self, pile: &Pile) {
        if self.column_idx + 1 < GAME_ARENA_COLUMNS {
            let pile_next_column_top = pile.column_tops[self.column_idx + 1].1;
            if self.rect.bottom() <= pile_next_column_top {
                self.rect.x += BLOCK_SIZE;
                self.column_idx += 1;
                for i in 0..3 {
                    self.blocks[i].rect.x += BLOCK_SIZE;
                }
            }
        }
    }

    pub fn move_to_left(&mut self, pile: &Pile) {
        if self.column_idx > 0 {
            let pile_previous_column_top = pile.column_tops[self.column_idx - 1].1;
            if self.rect.bottom() <= pile_previous_column_top {
                self.rect.x -= BLOCK_SIZE;
                self.column_idx -= 1;
                for i in 0..3 {
                    self.blocks[i].rect.x -= BLOCK_SIZE;
                }
            }
        }
    }

    pub fn rearrange_up(&mut self) {
        let code_color_0 = self.blocks[0].color;
        self.blocks[0].color = self.blocks[1].color;
        self.blocks[1].color = self.blocks[2].color;
        self.blocks[2].color = code_color_0;
    }

    pub fn rearrange_down(&mut self) {
        let code_color_2 = self.blocks[2].color;
        self.blocks[2].color = self.blocks[1].color;
        self.blocks[1].color = self.blocks[0].color;
        self.blocks[0].color = code_color_2;
    }

    pub fn drop(&mut self, pile: &Pile) {
        let pile_column_top = pile.column_tops[self.column_idx].1;
        self.rect.y = pile_column_top - self.rect.h;
        for i in 0..3 {
            self.blocks[i].rect.y = BLOCK_SIZE.mul_add(i as f32, self.rect.y);
        }
    }

    pub fn descend_one_step(&mut self, pile: &Pile) -> bool {
        let pile_column_top = pile.column_tops[self.column_idx].1;
        let is_descending_over = self.rect.bottom() == pile_column_top;
        if !is_descending_over {
            self.rect.y += BLOCK_SIZE;
            for i in 0..3 {
                self.blocks[i].rect.y += BLOCK_SIZE;
            }
        }
        self.rect.bottom() == pile_column_top
    }

    #[inline]
    pub fn is_at_bottom(&self, pile: &Pile) -> bool {
        self.rect.bottom() == pile.column_tops[self.column_idx].1
    }

    pub fn get_visible_blocks(&self) -> Vec<Block> {
        let mut blocks = Vec::with_capacity(3);
        if self.rect.right() < GAME_ARENA_RECT.left() {
            // next cargo
            for i in 0..3 {
                blocks.push(self.blocks[i]);
            }
        } else {
            // descending cargo
            for i in 0..3 {
                let block = self.blocks[i];
                if block.rect.bottom() > GAME_ARENA_RECT.top() {
                    blocks.push(block);
                }
            }
        }
        blocks
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        for i in 0..3 {
            if self.blocks[i].rect.bottom() > GAME_ARENA_RECT.top() {
                self.blocks[i].draw(ctx)?;
            }
        }
        Ok(())
    }
}

impl Display for Cargo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let repr = self
            .blocks
            .iter()
            .map(|block| format!("{}", block))
            .fold(String::new(), |accumulator, element| {
                accumulator + &element + ", "
            });
        let mut repr_iter = repr.chars();
        repr_iter.next_back();
        repr_iter.next_back();
        write!(f, "{}", repr_iter.as_str())
    }
}
