use std::collections::HashSet;

use ggez::{
    graphics::{self, Color, DrawParam, Font, Mesh, PxScale, Text, TextFragment},
    mint::Point2,
    Context, GameResult,
};

use crate::{
    blocks::idx_pair_to_center_point_of_block,
    constants::{MATCH_COMBO_POINTS_CHAR_SCALE, MATCH_DIRECTION_INDICATOR_WIDTH},
};

use super::{
    pile::{Matches, Pile},
    Block,
};

/*******************************************************************************
**** MATCHING
*******************************************************************************/
type ExtractedMatchingData = (
    Vec<usize>,
    Vec<(Color, [Point2<f32>; 2])>,
    HashSet<[usize; 2]>,
    Vec<Block>,
);

#[derive(Debug)]
pub struct Matching {
    num_of_sequential_matchings: usize,

    num_of_matching_blocks: Vec<usize>,
    match_direction_indicators: Vec<(Color, [Point2<f32>; 2])>,
    unique_matching_blocks_indexes: HashSet<[usize; 2]>,
    blocks: Vec<Block>,

    pub blinking_animation_stage: usize,
}

impl Matching {
    pub fn new(matches: &Matches, pile: &mut Pile) -> Self {
        let (num_of_matching_blocks, match_direction_indicators, unique_match_indexes, blocks) =
            Self::extract_matching_data_from_matches(matches, pile);

        Self {
            num_of_sequential_matchings: 1,

            num_of_matching_blocks,
            match_direction_indicators,
            unique_matching_blocks_indexes: unique_match_indexes,
            blocks,

            blinking_animation_stage: 0,
        }
    }

    pub fn new_chained_match(&mut self, matches: &Matches, pile: &mut Pile) {
        self.num_of_sequential_matchings += 1;

        let (num_of_matching_blocks, match_direction_indicators, unique_match_indexes, blocks) =
            Self::extract_matching_data_from_matches(matches, pile);

        self.num_of_matching_blocks = num_of_matching_blocks;
        self.match_direction_indicators = match_direction_indicators;
        self.unique_matching_blocks_indexes = unique_match_indexes;
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
        matches: &Matches,
        pile: &mut Pile,
    ) -> ExtractedMatchingData {
        let mut num_of_matching_blocks = Vec::new();
        let mut match_direction_indicators = Vec::new();
        let mut unique_match_indexes = HashSet::new();
        for matches in matches.values() {
            for r#match in matches {
                num_of_matching_blocks.push(r#match.1.len());

                #[allow(clippy::unwrap_used)]
                let pos_first = r#match.1.first().unwrap();
                let start_point = idx_pair_to_center_point_of_block(pos_first);

                #[allow(clippy::unwrap_used)]
                let pos_last = r#match.1.last().unwrap();
                let end_point = idx_pair_to_center_point_of_block(pos_last);

                match_direction_indicators.push((r#match.0, [start_point, end_point]));

                for position in &r#match.1 {
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
    pub fn get_unique_matching_blocks_indexes(&self) -> &HashSet<[usize; 2]> {
        &self.unique_matching_blocks_indexes
    }

    // TODO remove
    /*#[inline]
    pub fn get_num_of_sequential_matchings(&self) -> usize {
        self.num_of_sequential_matchings
    }*/

    pub fn get_blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.blinking_animation_stage % 2 == 0 {
            for block in &mut self.blocks {
                block.draw(ctx)?;
            }
        } else {
            for (color, points) in &self.match_direction_indicators {
                let line_mesh =
                    Mesh::new_line(ctx, points, MATCH_DIRECTION_INDICATOR_WIDTH, *color)?;
                graphics::draw(ctx, &line_mesh, DrawParam::default())?;
            }
        }
        Ok(())
    }
}

/*******************************************************************************
**** COMBO POINTS ANIMATION
*******************************************************************************/
pub struct ComboPointsAnimationsHolder {
    font: Font,
    pub current_animations: Vec<ComboPointsAnimation>,
}

impl ComboPointsAnimationsHolder {
    pub fn new(font: Font) -> Self {
        Self {
            font,
            current_animations: Vec::with_capacity(5),
        }
    }

    pub fn start_new_animation(
        &mut self,
        points: usize,
        matching_blocks_indexes: &HashSet<[usize; 2]>,
    ) {
        let starting_position =
            Self::calculate_animation_starting_position(matching_blocks_indexes);
        let new_animation = ComboPointsAnimation::new(points, starting_position, self.font);
        self.current_animations.push(new_animation);
    }

    fn calculate_animation_starting_position(
        matching_blocks_indexes: &HashSet<[usize; 2]>,
    ) -> Point2<f32> {
        #[allow(clippy::unwrap_used)]
        let highest_y_idx = matching_blocks_indexes
            .iter()
            .map(|point| point[1])
            .reduce(|accum, item| if accum >= item { accum } else { item })
            .unwrap();
        #[allow(clippy::unwrap_used)]
        let highest_rightmost_idx_pair = matching_blocks_indexes
            .iter()
            .filter(|point| point[1] == highest_y_idx)
            .reduce(|accum, item| if accum[0] >= item[0] { accum } else { item })
            .unwrap();

        idx_pair_to_center_point_of_block(highest_rightmost_idx_pair)
    }

    pub fn update_animations(&mut self) {
        if !self.current_animations.is_empty() {
            //println!("NUM ANIMATIONS: {}", self.current_animations.len());
            let mut idx_of_finished_animation = None;
            for idx in 0..self.current_animations.len() {
                let is_over = self.current_animations[idx].update();
                if is_over {
                    idx_of_finished_animation = Some(idx);
                }
            }
            if let Some(idx_of_finished_animation) = idx_of_finished_animation {
                /*println!(
                    "---- removing animation with idx {}",
                    idx_of_finished_animation
                );*/
                self.current_animations
                    .swap_remove(idx_of_finished_animation);
            }
        }
    }

    pub fn draw_animations(&mut self, ctx: &mut Context) -> GameResult {
        if !self.current_animations.is_empty() {
            for animation in &mut self.current_animations {
                animation.draw(ctx)?;
            }
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.current_animations.clear();
    }
}

#[derive(Debug)]
pub struct ComboPointsAnimation {
    points: usize,
    position: Point2<f32>,
    font: Font,
    bckg_color: Color,
    color: Color,
    alpha: f32,
}

impl ComboPointsAnimation {
    pub fn new(points: usize, position: Point2<f32>, font: Font) -> Self {
        Self {
            points,
            position,
            font,
            bckg_color: Color::BLACK,
            color: Color::WHITE,
            alpha: 1.0,
        }
    }

    pub fn update(&mut self) -> bool {
        /*println!(
            "\n*** UPDATE ANIMATION: {} // {:?}",
            self.points, self.position
        );*/

        self.position.x += 0.05;
        self.position.y -= 0.6;

        self.bckg_color.a = self.alpha;
        self.color.a = self.alpha;
        if self.alpha <= 0.0 {
            return true;
        }
        self.alpha -= 0.016;
        false
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        /*println!(
            "*** DRAW ANIMATION: {:?} - {:?} \t// next alpha: {}\n",
            self.bckg_color, self.color, self.alpha
        );*/

        // OUTLINE
        let points_bckg = Text::new(TextFragment {
            text: self.points.to_string(),
            color: Some(self.bckg_color),
            font: Some(self.font),
            scale: Some(PxScale::from(MATCH_COMBO_POINTS_CHAR_SCALE)),
        });
        let bckg_offset = 1.5;
        let mut bckg_position;
        // up
        bckg_position = Point2 {
            x: self.position.x,
            y: self.position.y - bckg_offset,
        };
        graphics::queue_text(ctx, &points_bckg, bckg_position, None);
        // upper-right corner
        bckg_position = Point2 {
            x: self.position.x + bckg_offset,
            y: self.position.y - bckg_offset,
        };
        graphics::queue_text(ctx, &points_bckg, bckg_position, None);
        // right
        bckg_position = Point2 {
            x: self.position.x + bckg_offset,
            y: self.position.y,
        };
        graphics::queue_text(ctx, &points_bckg, bckg_position, None);
        // lower-right corner
        bckg_position = Point2 {
            x: self.position.x + bckg_offset,
            y: self.position.y + bckg_offset,
        };
        graphics::queue_text(ctx, &points_bckg, bckg_position, None);
        // down
        bckg_position = Point2 {
            x: self.position.x,
            y: self.position.y + bckg_offset,
        };
        graphics::queue_text(ctx, &points_bckg, bckg_position, None);
        // lower-left corner
        bckg_position = Point2 {
            x: self.position.x - bckg_offset,
            y: self.position.y + bckg_offset,
        };
        graphics::queue_text(ctx, &points_bckg, bckg_position, None);
        // left
        bckg_position = Point2 {
            x: self.position.x - bckg_offset,
            y: self.position.y,
        };
        graphics::queue_text(ctx, &points_bckg, bckg_position, None);
        // upper-left corner
        bckg_position = Point2 {
            x: self.position.x - bckg_offset,
            y: self.position.y - bckg_offset,
        };
        graphics::queue_text(ctx, &points_bckg, bckg_position, None);

        let points = Text::new(TextFragment {
            text: self.points.to_string(),
            color: Some(self.color),
            font: Some(self.font),
            scale: Some(PxScale::from(MATCH_COMBO_POINTS_CHAR_SCALE)),
        });
        graphics::queue_text(ctx, &points, self.position, None);

        graphics::draw_queued_text(
            ctx,
            DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )?;

        Ok(())
    }
}
