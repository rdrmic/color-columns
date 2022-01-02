use std::{
    cmp::{max, min, Ord},
    collections::{HashMap, HashSet},
    mem,
};

use ggez::Context;

use crate::{
    config::{BLOCK_SIZE, GAME_ARENA_COLUMNS, GAME_ARENA_RECT, GAME_ARENA_ROWS, NO_BLOCK_CODE},
    stages::playing::Direction,
};

use super::{cargo::Cargo, Block};

/*******************************************************************************
**** PILE
*******************************************************************************/
pub struct Pile {
    matrix: [[Option<Block>; GAME_ARENA_ROWS]; GAME_ARENA_COLUMNS],
    pub column_tops: [(isize, f32); GAME_ARENA_COLUMNS],
}

impl Pile {
    pub fn new() -> Self {
        Pile {
            matrix: [[None; GAME_ARENA_ROWS]; GAME_ARENA_COLUMNS],
            column_tops: [(-1, GAME_ARENA_RECT.bottom()); GAME_ARENA_COLUMNS],
        }
    }

    pub fn from_snapshot(
        matrix: &[[Option<Block>; GAME_ARENA_ROWS]; GAME_ARENA_COLUMNS],
        column_tops: [(isize, f32); GAME_ARENA_COLUMNS],
    ) -> Self {
        Pile {
            matrix: *matrix,
            column_tops,
        }
    }

    pub fn take_cargo(&mut self, cargo: &Cargo) -> isize {
        let pile_column_top = self.column_tops[cargo.column_idx];
        let mut pile_column_top_row_idx = pile_column_top.0;

        let spaces_to_fill = GAME_ARENA_ROWS as isize - 1 - pile_column_top_row_idx;
        let row_increment = min(3, spaces_to_fill) as usize;
        for cargo_block_idx in (3 - row_increment..3).rev() {
            pile_column_top_row_idx += 1;
            self.matrix[cargo.column_idx][pile_column_top_row_idx as usize] =
                Some(cargo.blocks[cargo_block_idx]);
        }

        self.column_tops[cargo.column_idx].0 += row_increment as isize;
        self.column_tops[cargo.column_idx].1 -= BLOCK_SIZE * row_increment as f32;

        spaces_to_fill - 3
    }

    /*** SEARCHING FOR MATCHES [BEGIN] ***/
    pub fn search_for_matches(&self) -> HashMap<Direction, Vec<Vec<(usize, usize)>>> {
        let mut matches = HashMap::<Direction, Vec<Vec<(usize, usize)>>>::new();
        self.collect_vertical_matches(&mut matches);
        self.collect_horizontal_matches(&mut matches);
        self.collect_diagonal_slash_matches(&mut matches);
        self.collect_diagonal_backslash_matches(&mut matches);
        matches
    }

    fn collect_vertical_matches(&self, matches: &mut HashMap<Direction, Vec<Vec<(usize, usize)>>>) {
        let mut sequence = Vec::with_capacity(GAME_ARENA_ROWS);
        let mut match_collector = Vec::<(char, usize, usize)>::with_capacity(5);

        for col_idx in 0..GAME_ARENA_COLUMNS {
            let column_top_idx = self.column_tops[col_idx].0;
            if column_top_idx >= 2 {
                for row_idx in (0..=column_top_idx as usize).rev() {
                    let block = self.matrix[col_idx][row_idx as usize].unwrap();
                    sequence.push((block.color.code, col_idx, row_idx as usize));
                }
                Self::search_sequence_for_vertical_matches(
                    Direction::Vertical,
                    &sequence,
                    &mut match_collector,
                    matches,
                );
                sequence.clear();
            }
        }
    }

    fn collect_horizontal_matches(
        &self,
        matches: &mut HashMap<Direction, Vec<Vec<(usize, usize)>>>,
    ) {
        let mut sequence = Vec::with_capacity(GAME_ARENA_COLUMNS);
        let mut match_collector = Vec::<(char, usize, usize)>::with_capacity(5);

        let topmost_column_idx = self.get_topmost_column_idx();
        if topmost_column_idx > -1 {
            for row_idx in 0..=topmost_column_idx as usize {
                for col_idx in 0..GAME_ARENA_COLUMNS {
                    let code = Self::get_block_code(&self.matrix[col_idx][row_idx as usize]);
                    sequence.push((code, col_idx, row_idx as usize));
                }
                Self::search_sequence_for_matches(
                    Direction::Horizontal,
                    &sequence,
                    &mut match_collector,
                    matches,
                );
                sequence.clear();
            }
        }
    }

    fn collect_diagonal_slash_matches(
        &self,
        matches: &mut HashMap<Direction, Vec<Vec<(usize, usize)>>>,
    ) {
        let mut sequence = Vec::with_capacity(max(GAME_ARENA_COLUMNS, GAME_ARENA_ROWS));
        let mut match_collector = Vec::<(char, usize, usize)>::with_capacity(5);

        let col_idx_start: usize = 0;
        for row_idx_start in (0..GAME_ARENA_ROWS - 2).rev() {
            let mut row_idx = row_idx_start;
            let mut col_idx = col_idx_start;
            while row_idx < GAME_ARENA_ROWS && col_idx < GAME_ARENA_COLUMNS {
                let code = Self::get_block_code(&self.matrix[col_idx][row_idx as usize]);
                sequence.push((code, col_idx, row_idx as usize));
                row_idx += 1;
                col_idx += 1;
            }
            Self::search_sequence_for_matches(
                Direction::DiagonalSlash,
                &sequence,
                &mut match_collector,
                matches,
            );
            sequence.clear();
        }
        let row_idx_start: usize = 0;
        for col_idx_start in 1..GAME_ARENA_COLUMNS - 2 {
            let mut row_idx = row_idx_start;
            let mut col_idx = col_idx_start;
            while row_idx < GAME_ARENA_ROWS && col_idx < GAME_ARENA_COLUMNS {
                let code = Self::get_block_code(&self.matrix[col_idx][row_idx as usize]);
                sequence.push((code, col_idx, row_idx as usize));
                row_idx += 1;
                col_idx += 1;
            }
            Self::search_sequence_for_matches(
                Direction::DiagonalSlash,
                &sequence,
                &mut match_collector,
                matches,
            );
            sequence.clear();
        }
    }

    fn collect_diagonal_backslash_matches(
        &self,
        matches: &mut HashMap<Direction, Vec<Vec<(usize, usize)>>>,
    ) {
        let mut sequence = Vec::with_capacity(max(GAME_ARENA_COLUMNS, GAME_ARENA_ROWS));
        let mut match_collector = Vec::<(char, usize, usize)>::with_capacity(5);

        let row_idx_start: usize = 0;
        for col_idx_start in 2..GAME_ARENA_COLUMNS as isize {
            let mut row_idx = row_idx_start;
            let mut col_idx = col_idx_start;
            while row_idx < GAME_ARENA_ROWS && col_idx >= 0 {
                let code = Self::get_block_code(&self.matrix[col_idx as usize][row_idx]);
                sequence.push((code, col_idx as usize, row_idx as usize));
                row_idx += 1;
                col_idx -= 1;
            }
            Self::search_sequence_for_matches(
                Direction::DiagonalBackslash,
                &sequence,
                &mut match_collector,
                matches,
            );
            sequence.clear();
        }
        let col_idx_start = GAME_ARENA_COLUMNS as isize - 1;
        for row_idx_start in 1..GAME_ARENA_ROWS - 2 {
            let mut row_idx = row_idx_start;
            let mut col_idx = col_idx_start;
            while row_idx < GAME_ARENA_ROWS && col_idx >= 0 {
                let code = Self::get_block_code(&self.matrix[col_idx as usize][row_idx]);
                sequence.push((code, col_idx as usize, row_idx as usize));
                row_idx += 1;
                col_idx -= 1;
            }
            Self::search_sequence_for_matches(
                Direction::DiagonalBackslash,
                &sequence,
                &mut match_collector,
                matches,
            );
            sequence.clear();
        }
    }

    fn get_block_code(block: &Option<Block>) -> char {
        if let Some(block) = block {
            return block.color.code;
        }
        NO_BLOCK_CODE
    }

    fn search_sequence_for_vertical_matches(
        direction: Direction,
        sequence: &[(char, usize, usize)],
        collector: &mut Vec<(char, usize, usize)>,
        matches: &mut HashMap<Direction, Vec<Vec<(usize, usize)>>>,
    ) {
        for block_repr in sequence {
            if let Some(previous_match) = collector.last() {
                if block_repr.0 != previous_match.0 {
                    Self::take_match_from_collector(direction, collector, matches);
                }
            }
            collector.push(*block_repr);
        }
        Self::take_match_from_collector(direction, collector, matches);
    }

    fn search_sequence_for_matches(
        direction: Direction,
        sequence: &[(char, usize, usize)],
        collector: &mut Vec<(char, usize, usize)>,
        matches: &mut HashMap<Direction, Vec<Vec<(usize, usize)>>>,
    ) {
        for block_repr in sequence {
            if block_repr.0 == NO_BLOCK_CODE {
                Self::take_match_from_collector(direction, collector, matches);
            } else {
                if let Some(previous_block_repr) = collector.last() {
                    if block_repr.0 != previous_block_repr.0 {
                        Self::take_match_from_collector(direction, collector, matches);
                    }
                }
                collector.push(*block_repr);
            }
        }
        Self::take_match_from_collector(direction, collector, matches);
    }

    fn take_match_from_collector(
        direction: Direction,
        collector: &mut Vec<(char, usize, usize)>,
        matches: &mut HashMap<Direction, Vec<Vec<(usize, usize)>>>,
    ) {
        if collector.len() >= 3 {
            let vec_of_matches = matches.entry(direction).or_insert_with(Vec::new);
            let r#match = collector
                .iter()
                .map(|block_repr| (block_repr.1, block_repr.2))
                .collect::<Vec<(usize, usize)>>();
            vec_of_matches.push(r#match);
        }
        collector.clear();
    }
    /*** SEARCHING FOR MATCHES [END] ***/

    pub fn extract_matching_blocks(
        &mut self,
        unique_match_positions: &HashSet<(usize, usize)>,
    ) -> Vec<Block> {
        let mut matched_blocks = Vec::with_capacity(unique_match_positions.len());
        for m in unique_match_positions {
            let block = mem::take(&mut self.matrix[m.0][m.1]);
            matched_blocks.push(block.unwrap());
        }
        matched_blocks
    }

    pub fn remove_matches(&mut self, matches: &HashSet<(usize, usize)>) -> bool {
        // COLLECT A MAP OF ROW (VERTICAL) INDEXES OF MATCHED BLOCKS BY COLUMN (HORIZONTAL) INDEX
        let mut matched_blocks_row_idxs_by_col_idx = HashMap::new();
        for m in matches {
            let col_idx = m.0;
            let row_idx = m.1;

            let row_idxs = matched_blocks_row_idxs_by_col_idx
                .entry(col_idx)
                .or_insert_with(Vec::new);
            row_idxs.push(row_idx);
        }
        // MAKE DANGLING BLOCKS FALL
        for (col_idx, row_idxs) in matched_blocks_row_idxs_by_col_idx {
            let lowest_matched_block_idx = *row_idxs.iter().min_by(Ord::cmp).unwrap();
            let mut num_empty_slots: usize = 0;
            for row_idx in lowest_matched_block_idx..=self.column_tops[col_idx].0 as usize {
                match mem::take(&mut self.matrix[col_idx][row_idx]) {
                    None => num_empty_slots += 1,
                    Some(mut block) => {
                        block.rect.y += num_empty_slots as f32 * BLOCK_SIZE;
                        self.matrix[col_idx][row_idx - num_empty_slots] = Some(block);
                    }
                }
            }

            let num_matched_blocks = row_idxs.len();
            self.column_tops[col_idx].0 -= num_matched_blocks as isize;
            self.column_tops[col_idx].1 += num_matched_blocks as f32 * BLOCK_SIZE;
        }
        // CHECK IF PILE IS FULL (TOP OF UPMOST CARGO == TOP OF ARENA)
        self.get_topmost_column_idx() >= (GAME_ARENA_ROWS - 1) as isize
    }

    #[inline]
    fn get_topmost_column_idx(&self) -> isize {
        self.column_tops
            .iter()
            .max_by(|top1, top2| top1.0.cmp(&top2.0))
            .unwrap()
            .0
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        let mut num_blocks: usize = 0;
        for col_idx in 0..GAME_ARENA_COLUMNS {
            let column_top = self.column_tops[col_idx].0;
            if column_top > -1 {
                num_blocks += column_top as usize + 1;
            }
        }

        let mut blocks = Vec::with_capacity(num_blocks);
        for col_idx in 0..GAME_ARENA_COLUMNS {
            let column_top = self.column_tops[col_idx].0;
            if column_top > -1 {
                for row_idx in 0..=column_top as usize {
                    if let Some(block) = self.matrix[col_idx][row_idx] {
                        blocks.push(block);
                    }
                }
            }
        }
        blocks
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for col_idx in 0..GAME_ARENA_COLUMNS {
            for row_idx in 0..GAME_ARENA_ROWS {
                if let Some(mut block) = self.matrix[col_idx][row_idx] {
                    block.draw(ctx);
                }
            }
        }
    }

    pub fn __print(&self) {
        for row_idx in (0..GAME_ARENA_ROWS).rev() {
            for col_idx in 0..GAME_ARENA_COLUMNS {
                if let Some(block) = self.matrix[col_idx][row_idx] {
                    print!("{} ", block.color.code);
                } else {
                    print!("{} ", NO_BLOCK_CODE);
                }
            }
            println!();
        }
    }
}
