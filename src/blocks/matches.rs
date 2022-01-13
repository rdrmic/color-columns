use std::collections::{HashMap, HashSet};

use ggez::{
    graphics::{self, DrawParam, Mesh},
    mint::Point2,
    Context, GameResult,
};

use crate::{
    blocks::idx_pair_to_center_point_of_block,
    constants::{COLOR_LIGHT_GRAY, NUM_TICKS_SEQUENCE_FOR_MATCHES_REMOVAL},
    stages::playing::Direction,
};

use super::{pile::Pile, Block};

/*******************************************************************************
**** MATCHING
*******************************************************************************/
#[derive(Debug)]
pub struct Matching {
    num_of_sequential_matchings: usize,

    num_of_matching_blocks: Vec<usize>,
    match_direction_indicators: Vec<(Point2<f32>, Point2<f32>)>,
    unique_match_indexes: HashSet<(usize, usize)>,
    blocks: Vec<Block>,

    blinking_animation_stage: usize,
}

type MatchingData = (
    Vec<usize>,
    Vec<(Point2<f32>, Point2<f32>)>,
    HashSet<(usize, usize)>,
    Vec<Block>,
);

impl Matching {
    pub fn new(matches: &HashMap<Direction, Vec<Vec<(usize, usize)>>>, pile: &mut Pile) -> Self {
        let (num_of_matching_blocks, match_direction_indicators, unique_match_indexes, blocks) =
            Self::extract_matching_data_from_matches(matches, pile);

        Self {
            num_of_sequential_matchings: 1,

            num_of_matching_blocks,
            match_direction_indicators,
            unique_match_indexes,
            blocks,

            blinking_animation_stage: 0,
        }
    }

    pub fn new_chained_match(
        &mut self,
        matches: &HashMap<Direction, Vec<Vec<(usize, usize)>>>,
        pile: &mut Pile,
    ) {
        self.num_of_sequential_matchings += 1;

        let (num_of_matching_blocks, match_direction_indicators, unique_match_indexes, blocks) =
            Self::extract_matching_data_from_matches(matches, pile);

        self.num_of_matching_blocks = num_of_matching_blocks;
        self.match_direction_indicators = match_direction_indicators;
        self.unique_match_indexes = unique_match_indexes;
        self.blocks = blocks;

        self.blinking_animation_stage = 0;
    }

    #[inline]
    pub fn get_scoring_data(&self) -> (&Vec<usize>, usize) {
        (
            &self.num_of_matching_blocks,
            self.num_of_sequential_matchings,
        )
    }

    fn extract_matching_data_from_matches(
        matches: &HashMap<Direction, Vec<Vec<(usize, usize)>>>,
        pile: &mut Pile,
    ) -> MatchingData {
        let mut num_of_matching_blocks = Vec::new();
        let mut match_direction_indicators = Vec::new();
        let mut unique_match_indexes = HashSet::new();
        for matches in matches.values() {
            for r#match in matches {
                num_of_matching_blocks.push(r#match.len());

                #[allow(clippy::unwrap_used)]
                let pos_first = r#match.first().unwrap();
                let start_point = idx_pair_to_center_point_of_block(pos_first);

                #[allow(clippy::unwrap_used)]
                let pos_last = r#match.last().unwrap();
                let end_point = idx_pair_to_center_point_of_block(pos_last);

                match_direction_indicators.push((start_point, end_point));

                for position in r#match {
                    unique_match_indexes.insert(*position);
                }
            }
        }

        let matching_blocks = pile.extract_matching_blocks(&unique_match_indexes);
        (
            num_of_matching_blocks,
            match_direction_indicators,
            unique_match_indexes,
            matching_blocks,
        )
    }

    #[inline]
    pub fn get_unique_match_indexes(&self) -> &HashSet<(usize, usize)> {
        &self.unique_match_indexes
    }

    // TODO remove
    /*#[inline]
    pub fn get_num_of_sequential_matchings(&self) -> usize {
        self.num_of_sequential_matchings
    }*/

    pub fn blinking_animation(&mut self, num_frames: usize) -> bool {
        if num_frames % NUM_TICKS_SEQUENCE_FOR_MATCHES_REMOVAL[self.blinking_animation_stage] == 0 {
            self.blinking_animation_stage += 1;
        }
        self.blinking_animation_stage > 3
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.blinking_animation_stage % 2 == 0 {
            for block in &mut self.blocks {
                block.draw(ctx)?;
            }
        } else {
            for points in &self.match_direction_indicators {
                let line_mesh = Mesh::new_line(ctx, &[points.0, points.1], 1.0, COLOR_LIGHT_GRAY)?;
                graphics::draw(ctx, &line_mesh, DrawParam::default())?;
            }
        }
        Ok(())
    }
}
