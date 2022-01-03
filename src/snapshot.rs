#![allow(clippy::cast_possible_wrap)]

use std::fs;

use ggez::mint::Point2;

use crate::blocks::pile::Pile;
use crate::blocks::{idx_to_position, Block, Factory};
use crate::config::{
    BLOCK_SIZE, GAME_ARENA_COLUMNS, GAME_ARENA_RECT, GAME_ARENA_ROWS, NO_BLOCK_CODE,
};

pub fn create_pile_from_file() -> Pile {
    let matrix_snapshot = fs::read_to_string("snapshots/snapshot.txt").unwrap();

    let mut matrix_snapshot_vec = Vec::with_capacity(GAME_ARENA_ROWS);
    for line in matrix_snapshot.lines().rev() {
        let row: Vec<char> = line
            .split_whitespace()
            .map(|str| str.chars().next().unwrap())
            .collect();
        matrix_snapshot_vec.push(row);
    }
    if matrix_snapshot_vec.len() != GAME_ARENA_ROWS
        || matrix_snapshot_vec[0].len() != GAME_ARENA_COLUMNS
    {
        panic!("Snapshot's dimensions don't match those in app!");
    }

    let mut matrix = [[Option::<Block>::None; GAME_ARENA_ROWS]; GAME_ARENA_COLUMNS];
    for (row_idx, row) in matrix_snapshot_vec.iter().enumerate() {
        for (col_idx, snapshot_block_code) in row.iter().enumerate() {
            let block = if *snapshot_block_code == NO_BLOCK_CODE {
                None
            } else {
                let point = Point2 {
                    x: idx_to_position(col_idx, 'x'),
                    y: idx_to_position(row_idx, 'y'),
                };
                let color = Factory::COLORS
                    .into_iter()
                    .find(|color| color.code == *snapshot_block_code)
                    .expect("Snapshot's colors don't match those in app!");
                Some(Block::new(point, BLOCK_SIZE, color))
            };
            matrix[col_idx][row_idx] = block;
        }
    }

    let mut column_tops = [(-1, GAME_ARENA_RECT.bottom()); GAME_ARENA_COLUMNS];
    for col_idx in 0..GAME_ARENA_COLUMNS {
        for row_idx in (0..GAME_ARENA_ROWS).rev() {
            if matrix[col_idx][row_idx].is_some() {
                column_tops[col_idx] = (row_idx as isize, idx_to_position(row_idx, 'y'));
                break;
            }
        }
    }

    Pile::from_snapshot(&matrix, column_tops)
}
